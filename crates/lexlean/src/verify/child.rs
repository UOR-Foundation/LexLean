//! Child process execution and output normalization (SPEC.md §22.3, §22.7,
//! §25.2, §25.4): direct executable and argv invocation with no shell, a
//! deterministic allow-list environment, checked limits, and the exact
//! path-replacement normalization.

use std::collections::BTreeSet;
use std::io::Read;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use camino::{Utf8Path, Utf8PathBuf};

use crate::artifact::canonical_json::Json;
use crate::artifact::content_id::Sha256Digest;
use crate::code;
use crate::config::Limits;
use crate::diagnostic::Diagnostic;
use crate::verify::lifeline;
use crate::verify::telemetry;

/// The §22.7 normalizer: ordered longest-prefix replacements.
#[derive(Debug, Clone, Default)]
pub struct Normalizer {
    replacements: Vec<(String, &'static str)>,
}

impl Normalizer {
    /// Build the ordered replacement list: staging, project, Lake
    /// workspace, toolchain, home.
    #[must_use]
    pub fn new(
        staging: &Utf8Path,
        project: &Utf8Path,
        lake_workspace: &Utf8Path,
        toolchain: &Utf8Path,
    ) -> Self {
        let mut replacements = vec![
            (staging.to_string(), "$STAGING"),
            (project.to_string(), "$PROJECT"),
            (lake_workspace.to_string(), "$LAKE_WORKSPACE"),
            (toolchain.to_string(), "$TOOLCHAIN"),
        ];
        if let Some(home) = std::env::var_os("HOME") {
            if let Some(text) = home.to_str() {
                replacements.push((text.to_owned(), "$HOME"));
            }
        }
        Self { replacements }
    }

    /// Normalize process output (§22.7): CRLF and CR to LF, ANSI escapes
    /// removed, prefixes replaced in order, trailing spaces removed, blank
    /// final lines collapsed to one final LF.
    #[must_use]
    pub fn normalize(&self, bytes: &[u8]) -> String {
        let raw = String::from_utf8_lossy(bytes);
        // 1. Line endings.
        let mut text = String::with_capacity(raw.len());
        let mut chars = raw.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\r' {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                text.push('\n');
            } else {
                text.push(c);
            }
        }
        // 2. ANSI escape sequences.
        let mut stripped = String::with_capacity(text.len());
        let mut iterator = text.chars().peekable();
        while let Some(c) = iterator.next() {
            if c == '\u{1b}' {
                if iterator.peek() == Some(&'[') {
                    iterator.next();
                    for terminator in iterator.by_ref() {
                        if terminator.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
                continue;
            }
            stripped.push(c);
        }
        // 3. Ordered prefix replacement.
        let mut replaced = stripped;
        for (prefix, token) in &self.replacements {
            if !prefix.is_empty() {
                replaced = replaced.replace(prefix, token);
            }
        }
        // 4–5. Trailing spaces; blank final lines collapse to one LF.
        let mut lines: Vec<&str> = replaced.split('\n').map(str::trim_end).collect();
        while lines.last() == Some(&"") {
            lines.pop();
        }
        let mut result = lines.join("\n");
        result.push('\n');
        result
    }

    /// Normalize one argv element for records.
    #[must_use]
    pub fn normalize_arg(&self, argument: &str) -> String {
        let mut out = argument.to_owned();
        for (prefix, token) in &self.replacements {
            if !prefix.is_empty() {
                out = out.replace(prefix, token);
            }
        }
        out
    }

    /// Does normalized output still contain an unexpected absolute path
    /// (§22.7)? After the ordered prefix replacement, every remaining
    /// `/`-rooted path (`/` at a token boundary followed by a path segment)
    /// or drive-rooted Windows path (`X:\` or `X:/` at a token boundary)
    /// is unexpected. `$PLACEHOLDER`-prefixed forms and URL schemes
    /// (`scheme://host`) are not rooted paths. Successful output carrying
    /// one fails attestation construction.
    #[must_use]
    pub fn has_unexpected_absolute_path(&self, text: &str) -> bool {
        first_unexpected_absolute_path(text).is_some()
    }
}

/// A character that can begin a path segment after the root separator.
fn starts_segment(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '.'
}

/// A character that separates tokens, so a following `/` starts a rooted
/// path rather than continuing one.
fn is_token_boundary(c: char) -> bool {
    c.is_whitespace()
        || matches!(
            c,
            '"' | '\'' | '`' | '(' | '[' | '{' | '<' | ',' | ';' | '='
        )
}

/// The first unexpected absolute path in normalized text, when any: the
/// byte offset and the offending token.
#[must_use]
pub fn first_unexpected_absolute_path(text: &str) -> Option<(usize, String)> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    for (index, &(offset, c)) in chars.iter().enumerate() {
        let previous = index.checked_sub(1).map(|i| chars[i].1);
        let at_boundary = previous.is_none_or(is_token_boundary);
        let next = chars.get(index.saturating_add(1)).map(|pair| pair.1);
        let rooted_posix = c == '/' && at_boundary && next.is_some_and(starts_segment);
        let rooted_windows = c.is_ascii_alphabetic()
            && at_boundary
            && next == Some(':')
            && chars
                .get(index.saturating_add(2))
                .is_some_and(|pair| pair.1 == '/' || pair.1 == '\\')
            && chars
                .get(index.saturating_add(3))
                .is_some_and(|pair| starts_segment(pair.1));
        if rooted_posix || rooted_windows {
            let token: String = text[offset..]
                .chars()
                .take_while(|c| {
                    !c.is_whitespace()
                        && !matches!(c, '"' | '\'' | '`' | ')' | ']' | '}' | '>' | ',' | ';')
                })
                .collect();
            return Some((offset, token));
        }
    }
    None
}

/// One recorded child process (§22.3).
#[derive(Debug, Clone)]
pub struct ChildRecord {
    /// The tool name.
    pub tool: String,
    /// The module this run concerned, when per-module.
    pub module: Option<String>,
    /// The normalized argv.
    pub argv: Vec<String>,
    /// The exit code.
    pub exit_code: i32,
    /// Normalized stdout.
    pub stdout: String,
    /// Normalized stderr.
    pub stderr: String,
    /// SHA-256 of the executable.
    pub executable_sha256: Sha256Digest,
}

impl ChildRecord {
    /// The canonical JSON process record.
    #[must_use]
    pub fn to_json(&self) -> Json {
        let mut fields = vec![
            ("tool", Json::Str(self.tool.clone())),
            (
                "argv",
                Json::Arr(self.argv.iter().cloned().map(Json::Str).collect()),
            ),
            ("exit_code", Json::Int(i64::from(self.exit_code))),
            (
                "executable_sha256",
                Json::Str(self.executable_sha256.to_hex()),
            ),
            ("stdout", Json::Str(self.stdout.clone())),
            ("stderr", Json::Str(self.stderr.clone())),
            (
                "stdout_sha256",
                Json::Str(Sha256Digest::of(self.stdout.as_bytes()).to_hex()),
            ),
            (
                "stderr_sha256",
                Json::Str(Sha256Digest::of(self.stderr.as_bytes()).to_hex()),
            ),
        ];
        if let Some(module) = &self.module {
            fields.push(("module", Json::Str(module.clone())));
        }
        Json::object(fields)
    }

    /// A record read back from the JSON [`Self::to_json`] wrote, as a later
    /// part of a verification reads an earlier part's (§9.4). The stream
    /// digests are checked against the streams: a record whose text and
    /// digest disagree was not written by the verifier.
    ///
    /// # Errors
    ///
    /// A value that is not a process record.
    pub fn from_json(json: &Json) -> Result<Self, String> {
        let Json::Obj(fields) = json else {
            return Err("a process record is an object".to_owned());
        };
        let text = |name: &str| -> Result<String, String> {
            match fields.get(name) {
                Some(Json::Str(text)) => Ok(text.clone()),
                _ => Err(format!("`{name}` is a required string")),
            }
        };
        let argv = match fields.get("argv") {
            Some(Json::Arr(items)) => items
                .iter()
                .map(|item| match item {
                    Json::Str(text) => Ok(text.clone()),
                    _ => Err("`argv` holds strings".to_owned()),
                })
                .collect::<Result<Vec<String>, String>>()?,
            _ => return Err("`argv` is a required array".to_owned()),
        };
        let exit_code = match fields.get("exit_code") {
            Some(Json::Int(code)) => {
                i32::try_from(*code).map_err(|_| "`exit_code` is an i32".to_owned())?
            }
            _ => return Err("`exit_code` is a required integer".to_owned()),
        };
        let stdout = text("stdout")?;
        let stderr = text("stderr")?;
        for (name, stream) in [("stdout_sha256", &stdout), ("stderr_sha256", &stderr)] {
            if text(name)? != Sha256Digest::of(stream.as_bytes()).to_hex() {
                return Err(format!("`{name}` is not the digest of the stream"));
            }
        }
        let module = match fields.get("module") {
            None => None,
            Some(Json::Str(module)) => Some(module.clone()),
            Some(_) => return Err("`module` is a string".to_owned()),
        };
        let known: BTreeSet<&str> = [
            "tool",
            "argv",
            "exit_code",
            "executable_sha256",
            "stdout",
            "stderr",
            "stdout_sha256",
            "stderr_sha256",
            "module",
        ]
        .into_iter()
        .collect();
        if let Some(unknown) = fields.keys().find(|key| !known.contains(key.as_str())) {
            return Err(format!("`{unknown}` is not a field of a process record"));
        }
        Ok(Self {
            tool: text("tool")?,
            module,
            argv,
            exit_code,
            stdout,
            stderr,
            executable_sha256: Sha256Digest::from_hex(&text("executable_sha256")?)?,
        })
    }
}

/// The home a child sees (§25.4).
#[derive(Debug, Clone, Copy)]
pub enum ChildHome<'a> {
    /// The toolchain children: the platform home and `ELAN_HOME` are
    /// retained so `lake` and `lean` locate the pinned toolchain.
    Toolchain {
        /// The toolchain bin directory, prepended to `PATH`.
        toolchain_bin: &'a Utf8Path,
    },
    /// The PDF provider: an isolated temporary home, no `ELAN_HOME`, and
    /// the inherited `PATH` unchanged.
    Isolated {
        /// The temporary home directory.
        home: &'a Utf8Path,
    },
}

/// One child invocation.
pub struct ChildSpec<'a> {
    /// The tool label for records.
    pub tool: &'a str,
    /// The module label, when per-module.
    pub module: Option<String>,
    /// The absolute executable.
    pub program: &'a Utf8Path,
    /// The executable digest, already computed by preflight.
    pub executable_sha256: Sha256Digest,
    /// The argument vector.
    pub argv: Vec<String>,
    /// The working directory (the Lake workspace, §22.2).
    pub cwd: &'a Utf8Path,
    /// Extra environment rows (`LEAN_PATH` and friends).
    pub extra_env: Vec<(String, String)>,
    /// The home and `PATH` shape.
    pub home: ChildHome<'a>,
}

/// Locate an executable by name on the current `PATH` (§25.2): the
/// resolution is explicit and recorded, never left to the child.
pub fn resolve_on_path(name: &str) -> Result<Utf8PathBuf, Diagnostic> {
    let path_value = std::env::var_os("PATH").unwrap_or_default();
    for directory in std::env::split_paths(&path_value) {
        if directory.as_os_str().is_empty() {
            continue;
        }
        let candidate = directory.join(name);
        let Ok(metadata) = std::fs::metadata(&candidate) else {
            continue;
        };
        if !metadata.is_file() {
            continue;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o111 == 0 {
                continue;
            }
        }
        return Utf8PathBuf::from_path_buf(candidate).map_err(|bad| {
            Diagnostic::new(
                code!("LLV7008"),
                format!(
                    "non-UTF-8 path in the environment: {}",
                    bad.to_string_lossy()
                ),
            )
        });
    }
    Err(Diagnostic::new(
        code!("LLV7001"),
        format!("no `{name}` executable is available on PATH"),
    ))
}

/// Stop a child and everything it started: the whole process group it leads
/// (unix) or its process tree (windows), then the child itself, and wait
/// until nothing of the group is left running, so that a timeout ends the
/// work and not only the process that was waited for.
///
/// Ending the group is [`lifeline::end_leader`], the one place anything is
/// signalled to end children (§24.6): the timeout, the host-driven
/// `end_live_children`, and the executable's interrupt thread share it, and a
/// failure is returned and reported, never swallowed: a timeout that could
/// not end the processes says so. The leader is reaped afterwards: a zombie
/// leader is an ended one to the confirmation.
fn stop(child: &mut std::process::Child) -> Result<(), String> {
    let ended = lifeline::end_leader(child.id());
    let _ = child.kill();
    let _ = child.wait();
    match ended {
        lifeline::GroupEnd::Failed(reason) => Err(reason),
        lifeline::GroupEnd::Ended | lifeline::GroupEnd::AlreadyGone => Ok(()),
    }
}

/// How a diagnostic reports that the children could not be ended.
fn left_running(outcome: Result<(), String>) -> String {
    match outcome {
        Ok(()) => String::new(),
        Err(reason) => format!("; the processes it started may still be running: {reason}"),
    }
}

/// Run one child under the allow-list environment (§25.4), the timeout,
/// and the output cap (§25.5). Every arithmetic step over the configured
/// limits is checked; a limit failure is `LLS8002` naming the limit, the
/// configured value, the observed value, and the tool.
pub fn run(
    spec: &ChildSpec<'_>,
    limits: &Limits,
    normalizer: &Normalizer,
) -> Result<ChildRecord, Diagnostic> {
    let existing_path = std::env::var_os("PATH").unwrap_or_default();
    let mut command = Command::new(spec.program.as_std_path());
    command
        .args(&spec.argv)
        .current_dir(spec.cwd.as_std_path())
        .env_clear()
        .env("NO_COLOR", "1")
        .env("LANG", "C.UTF-8")
        .env("LC_ALL", "C.UTF-8")
        .env("GIT_TERMINAL_PROMPT", "0")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    match spec.home {
        ChildHome::Toolchain { toolchain_bin } => {
            let mut path = std::ffi::OsString::from(format!("{toolchain_bin}:"));
            path.push(&existing_path);
            command.env("PATH", path).env(
                "HOME",
                std::env::var_os("HOME").unwrap_or_else(|| "/".into()),
            );
            if let Some(elan_home) = std::env::var_os("ELAN_HOME") {
                command.env("ELAN_HOME", elan_home);
            }
        }
        ChildHome::Isolated { home } => {
            command
                .env("PATH", existing_path)
                .env("HOME", home.as_std_path());
        }
    }
    for (key, value) in &spec.extra_env {
        command.env(key, value);
    }
    // The child leads a process group of its own, so that what it starts
    // (`lake` starts `lean`) can be stopped with it: a kill of the leader
    // alone leaves the grandchild running with nothing to end it.
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    let (mut child, tracked) = lifeline::spawn(&mut command).map_err(|refusal| match refusal {
        // The host ended the live children and has not re-enabled new ones
        // (§24.6): a registered refusal, not a toolchain failure.
        lifeline::CannotStart::Shutdown => Diagnostic::new(
            code!("LLV7018"),
            format!(
                "{}: `{}`: the host ended the live children of this process and has not re-enabled starting children",
                spec.tool, spec.program
            ),
        ),
        lifeline::CannotStart::Failed(reason) => Diagnostic::new(
            code!("LLV7001"),
            format!("{}: `{}`: {reason}", spec.tool, spec.program),
        ),
    })?;
    let cap = limits.max_child_output_bytes;
    // Read one byte past the cap so an overflow is observed as `> cap`
    // without ever buffering unbounded output.
    let read_limit = cap.saturating_add(1);
    let (Some(mut stdout_pipe), Some(mut stderr_pipe)) = (child.stdout.take(), child.stderr.take())
    else {
        let ended = stop(&mut child);
        return Err(Diagnostic::new(
            code!("LLI9001"),
            format!(
                "{}: the child pipes were not attached{}",
                spec.tool,
                left_running(ended)
            ),
        ));
    };
    let stdout_reader = std::thread::spawn(move || {
        let mut buffer = Vec::new();
        let _ = stdout_pipe
            .by_ref()
            .take(read_limit)
            .read_to_end(&mut buffer);
        buffer
    });
    let stderr_reader = std::thread::spawn(move || {
        let mut buffer = Vec::new();
        let _ = stderr_pipe
            .by_ref()
            .take(read_limit)
            .read_to_end(&mut buffer);
        buffer
    });
    let started = Instant::now();
    let timeout = Duration::from_millis(limits.child_timeout_ms);
    let deadline = started.checked_add(timeout);
    // Measurement is observation only (§22.12): without a telemetry
    // context in force none of it is done, and with one it is written to the
    // sink and read by nobody here.
    let context = telemetry::current();
    let mut watch = context
        .as_ref()
        .map(|_| telemetry::GroupWatch::new(child.id()));
    let status = {
        // A descendant can hold either pipe after the leader exits. Keep
        // its group registered and the deadline active until reading ends.
        let _tracked = tracked;
        loop {
            match child.try_wait() {
                Ok(Some(status)) if stdout_reader.is_finished() && stderr_reader.is_finished() => {
                    break status;
                }
                Ok(_) => {
                    if let Some(watch) = watch.as_mut() {
                        watch.poll();
                    }
                    let now = Instant::now();
                    if deadline.is_none_or(|deadline| now >= deadline) {
                        let ended = stop(&mut child);
                        let elapsed_ms = now.saturating_duration_since(started).as_millis();
                        return Err(Diagnostic::new(
                        code!("LLS8002"),
                        format!(
                            "child_timeout_ms exceeded by `{}` in phase {}: configured {}, observed {} ms{}",
                            spec.tool,
                            spec.module.as_deref().unwrap_or("verify"),
                            limits.child_timeout_ms,
                            elapsed_ms,
                            left_running(ended)
                        ),
                    ));
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(io_error) => {
                    let ended = stop(&mut child);
                    return Err(Diagnostic::new(
                        code!("LLV7001"),
                        format!(
                            "waiting for {}: {io_error}{}",
                            spec.program,
                            left_running(ended)
                        ),
                    ));
                }
            }
        }
    };
    if let Some(context) = &context {
        context.process(&telemetry::ProcessReport {
            tool: spec.tool,
            module: spec.module.as_deref(),
            exit_code: status.code().unwrap_or(-1),
            started,
            peak_rss_kib: watch.as_ref().and_then(telemetry::GroupWatch::peak_rss_kib),
            peak_swap_kib: watch
                .as_ref()
                .and_then(telemetry::GroupWatch::peak_swap_kib),
        });
    }
    let stdout_bytes = stdout_reader.join().unwrap_or_default();
    let stderr_bytes = stderr_reader.join().unwrap_or_default();
    let stdout_len = u64::try_from(stdout_bytes.len()).unwrap_or(u64::MAX);
    let stderr_len = u64::try_from(stderr_bytes.len()).unwrap_or(u64::MAX);
    if stdout_len > cap || stderr_len > cap {
        let (stream, observed) = if stdout_len > cap {
            ("stdout", stdout_len)
        } else {
            ("stderr", stderr_len)
        };
        return Err(Diagnostic::new(
            code!("LLS8002"),
            format!(
                "max_child_output_bytes exceeded by `{}` on {stream} in phase {}: configured {}, observed at least {observed} bytes",
                spec.tool,
                spec.module.as_deref().unwrap_or("verify"),
                limits.max_child_output_bytes
            ),
        ));
    }
    Ok(ChildRecord {
        tool: spec.tool.to_owned(),
        module: spec.module.clone(),
        argv: spec
            .argv
            .iter()
            .map(|argument| normalizer.normalize_arg(argument))
            .collect(),
        exit_code: status.code().unwrap_or(-1),
        stdout: normalizer.normalize(&stdout_bytes),
        stderr: normalizer.normalize(&stderr_bytes),
        executable_sha256: spec.executable_sha256,
    })
}

#[cfg(test)]
mod timeout_tests {
    #[cfg(unix)]
    use super::{run, ChildHome, ChildSpec, Normalizer};
    #[cfg(unix)]
    use crate::artifact::content_id::Sha256Digest;
    #[cfg(unix)]
    use camino::Utf8Path;

    #[cfg(unix)]
    fn limits(timeout_ms: u64) -> crate::config::Limits {
        crate::config::Limits {
            max_file_bytes: 4_194_304,
            max_total_source_bytes: 67_108_864,
            max_primitive_atoms: 2_000_000,
            max_token_lattice_edges: 4_000_000,
            max_parse_states: 4_000_000,
            max_ir_nodes: 2_000_000,
            max_scope_depth: 1024,
            max_import_depth: 128,
            max_diagnostics: 256,
            max_child_output_bytes: 16_777_216,
            child_timeout_ms: timeout_ms,
        }
    }

    /// The processes whose command line holds `marker`, from the process
    /// table: `/proc` where the host has one and `ps` otherwise (macOS), so
    /// the case is not vacuous there; a host with neither is an error.
    #[cfg(unix)]
    fn running(marker: &str) -> Vec<String> {
        if std::path::Path::new("/proc/self").exists() {
            return std::fs::read_dir("/proc")
                .into_iter()
                .flatten()
                .flatten()
                .filter_map(|entry| {
                    let pid = entry.file_name().to_str()?.parse::<u32>().ok()?;
                    let command = std::fs::read(format!("/proc/{pid}/cmdline")).ok()?;
                    let command = String::from_utf8_lossy(&command).replace('\0', " ");
                    // A zombie is not running; nothing is left of it to end.
                    let status = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
                    let zombie = status.rsplit(") ").next()?.starts_with('Z');
                    (command.contains(marker) && !zombie).then(|| format!("{pid}: {command}"))
                })
                .collect();
        }
        let ps = ["/bin/ps", "/usr/bin/ps"]
            .into_iter()
            .find(|path| std::path::Path::new(path).is_file())
            .expect("the host has neither /proc nor ps, so the process table cannot be read");
        let output = std::process::Command::new(ps)
            .args(["-axo", "pid=,stat=,command="])
            .output()
            .expect("ps runs");
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter_map(|line| {
                let mut fields = line.split_whitespace();
                let pid = fields.next()?.parse::<u32>().ok()?;
                let state = fields.next()?;
                let command = fields.collect::<Vec<_>>().join(" ");
                (command.contains(marker) && !state.starts_with('Z'))
                    .then(|| format!("{pid}: {command}"))
            })
            .collect()
    }

    /// A group that could not be ended is reported as such, in the words of
    /// the timeout diagnostic, and an ended one adds nothing to it.
    #[test]
    fn a_group_that_could_not_be_ended_is_reported() {
        assert_eq!(super::left_running(Ok(())), "");
        let text = super::left_running(Err("EPERM".to_owned()));
        assert!(
            text.contains("may still be running") && text.contains("EPERM"),
            "{text}"
        );
    }

    /// A child that starts a long-running grandchild, as `lake` starts `lean`,
    /// and outlives the timeout: after the timeout is reported neither is
    /// running (the grandchild used to be orphaned, to run at full speed and
    /// to grow, with nothing left to end it).
    #[test]
    fn a_timeout_ends_what_the_child_started() {
        timeout_scenario();
    }

    /// Process groups are a unix notion; elsewhere the child is stopped with
    /// its process tree by other means, which this test does not read.
    #[cfg(not(unix))]
    fn timeout_scenario() {}

    #[cfg(unix)]
    fn timeout_scenario() {
        use std::os::unix::fs::PermissionsExt;
        let marker = format!("lexlean-grandchild-{}", std::process::id());
        let directory = tempfile::tempdir().expect("tempdir");
        let script = directory.path().join("lake");
        std::fs::write(
            &script,
            format!("#!/bin/sh\nsleep 1000.{} &\nwait\n", std::process::id()),
        )
        .expect("write");
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).expect("mode");
        let program = Utf8Path::from_path(&script).expect("utf-8");
        let cwd = Utf8Path::from_path(directory.path()).expect("utf-8");
        let spec = ChildSpec {
            tool: "lake",
            module: Some(marker.clone()),
            program,
            executable_sha256: Sha256Digest::of(b"script"),
            argv: Vec::new(),
            cwd,
            extra_env: Vec::new(),
            home: ChildHome::Isolated { home: cwd },
        };
        let normalizer = Normalizer::default();
        let sleeping = format!("sleep 1000.{}", std::process::id());
        let error = run(&spec, &limits(700), &normalizer).expect_err("the child outlives 700 ms");
        assert!(
            error.message.contains("child_timeout_ms exceeded"),
            "{error:?}"
        );
        let left = running(&sleeping);
        // Do not leave it behind if the assertion is about to fail.
        for line in &left {
            if let Some(pid) = line
                .split(':')
                .next()
                .and_then(|pid| pid.parse::<i32>().ok())
                .and_then(rustix::process::Pid::from_raw)
            {
                let _ = rustix::process::kill_process(pid, rustix::process::Signal::KILL);
            }
        }
        assert!(
            !error.message.contains("may still be running"),
            "the group was ended, so the report does not say otherwise: {error:?}"
        );
        assert!(left.is_empty(), "a timeout left running: {left:?}");
    }

    /// Reaping the leader does not close a pipe its descendant inherited.
    #[test]
    fn a_timeout_bounds_stdout_inherited_after_the_leader_exits() {
        #[cfg(unix)]
        inherited_pipe_timeout("stdout", "2>/dev/null");
    }

    #[test]
    fn a_timeout_bounds_stderr_inherited_after_the_leader_exits() {
        #[cfg(unix)]
        inherited_pipe_timeout("stderr", ">/dev/null");
    }

    #[cfg(unix)]
    fn inherited_pipe_timeout(stream: &str, redirect: &str) {
        use std::os::unix::fs::PermissionsExt;
        use std::time::{Duration, Instant};

        let directory = tempfile::tempdir().expect("tempdir");
        let script = directory.path().join("leader");
        // Finite even with the defect: the failing case leaves no process
        // behind and never relies on an outer timeout to finish.
        std::fs::write(
            &script,
            format!(
                "#!/bin/sh\nprintf '%s\\n' \"$$\" > leader.pid\nsleep 3 {redirect} &\nexit 0\n"
            ),
        )
        .expect("write");
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).expect("mode");
        let program = Utf8Path::from_path(&script).expect("utf-8");
        let cwd = Utf8Path::from_path(directory.path()).expect("utf-8");
        let spec = ChildSpec {
            tool: "pipe-holder",
            module: Some(stream.to_owned()),
            program,
            executable_sha256: Sha256Digest::of(b"script"),
            argv: Vec::new(),
            cwd,
            extra_env: Vec::new(),
            home: ChildHome::Isolated { home: cwd },
        };
        let started = Instant::now();
        let result = run(&spec, &limits(200), &Normalizer::default());
        let elapsed = started.elapsed();
        let group = std::fs::read_to_string(directory.path().join("leader.pid"))
            .expect("the leader recorded its group")
            .trim()
            .parse()
            .expect("a group number");
        let members =
            crate::verify::lifeline::group_members(group).expect("the host has /proc or ps");
        let live: Vec<_> = members.iter().filter(|member| !member.zombie).collect();
        if !live.is_empty() {
            let _ = crate::verify::lifeline::end_leader(u32::try_from(group).expect("group"));
        }
        let error = result.expect_err("an inherited pipe remains subject to the child timeout");
        assert_eq!(error.code.as_str(), "LLS8002", "{error:?}");
        for needle in ["child_timeout_ms", "configured 200", "pipe-holder", stream] {
            assert!(error.message.contains(needle), "{error:?}");
        }
        assert!(live.is_empty(), "a timeout left the descendant running");
        assert!(
            elapsed < Duration::from_secs(2),
            "the finite descendant outlived the configured timeout: {elapsed:?}"
        );
    }

    #[test]
    fn output_from_a_descendant_finishing_within_the_timeout_is_retained() {
        #[cfg(unix)]
        completed_descendant();
    }

    #[cfg(unix)]
    fn completed_descendant() {
        use std::os::unix::fs::PermissionsExt;

        let directory = tempfile::tempdir().expect("tempdir");
        let script = directory.path().join("leader");
        std::fs::write(
            &script,
            "#!/bin/sh\nprintf 'before\\n'\n(sleep 0.05; printf 'after\\n'; printf 'error\\n' >&2) &\nexit 0\n",
        )
        .expect("write");
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).expect("mode");
        let program = Utf8Path::from_path(&script).expect("utf-8");
        let cwd = Utf8Path::from_path(directory.path()).expect("utf-8");
        let spec = ChildSpec {
            tool: "pipe-holder",
            module: None,
            program,
            executable_sha256: Sha256Digest::of(b"script"),
            argv: Vec::new(),
            cwd,
            extra_env: Vec::new(),
            home: ChildHome::Isolated { home: cwd },
        };
        let record = run(&spec, &limits(5_000), &Normalizer::default())
            .expect("the descendant finishes within the timeout");
        assert_eq!(record.exit_code, 0);
        assert_eq!(record.stdout, "before\nafter\n");
        assert_eq!(record.stderr, "error\n");
    }

    /// The exit status `code` as the platform makes one.
    fn exit_status(code: i32) -> std::process::ExitStatus {
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            std::process::ExitStatus::from_raw(code << 8)
        }
        #[cfg(windows)]
        {
            use std::os::windows::process::ExitStatusExt;
            std::process::ExitStatus::from_raw(u32::try_from(code).expect("code"))
        }
    }

    /// `taskkill` exits 128 for a tree that is gone, for a process that has
    /// exited with its handle open, and for access denied, so the outcome of
    /// ending a tree also asks whether `tasklist` still lists the process: a
    /// success is an ended tree; a failure of a process that is no longer
    /// listed is a tree that was already gone; a failure of one that is
    /// listed, or that `tasklist` cannot say anything of, and a `taskkill`
    /// that could not be run, are reported.
    #[test]
    fn a_failing_taskkill_is_reported() {
        use crate::verify::lifeline::{tree_outcome, GroupEnd};
        assert_eq!(tree_outcome(Ok(exit_status(0)), None), GroupEnd::Ended);
        assert_eq!(
            tree_outcome(Ok(exit_status(128)), Some(false)),
            GroupEnd::AlreadyGone,
            "a tree no longer listed is already gone, not a failure"
        );
        let denied = tree_outcome(Ok(exit_status(128)), Some(true));
        assert!(
            matches!(&denied, GroupEnd::Failed(reason) if reason.contains("taskkill failed")),
            "exit 128 with the process still listed is reported: {denied:?}"
        );
        let unknown = tree_outcome(Ok(exit_status(128)), None);
        assert!(
            matches!(&unknown, GroupEnd::Failed(reason) if reason.contains("taskkill failed")),
            "exit 128 that tasklist cannot explain is reported: {unknown:?}"
        );
        let missing = tree_outcome(Err(std::io::Error::other("absent")), None);
        assert!(
            matches!(&missing, GroupEnd::Failed(reason) if reason.contains("could not be run")),
            "a taskkill that could not be run is reported: {missing:?}"
        );
    }

    /// A process group in which one process is a zombie that nothing reaps,
    /// as the grandchildren of a killed `lake` are under a PID 1 that never
    /// reaps (the shipped image runs `lexlean` as PID 1): the zombie is
    /// ended, so the stop neither waits two seconds for it nor says that
    /// processes may still be running. The zombie here is a child of this
    /// process, which `stop` reaps.
    #[test]
    fn a_zombie_in_the_group_counts_as_ended() {
        #[cfg(unix)]
        zombie_scenario();
    }

    #[cfg(unix)]
    fn zombie_scenario() {
        use std::os::unix::process::CommandExt;
        use std::process::{Command, Stdio};
        use std::time::{Duration, Instant};
        let mut leader = Command::new("sleep")
            .arg(format!("1000.{}", std::process::id()))
            .process_group(0)
            .stdin(Stdio::null())
            .spawn()
            .expect("sleep starts");
        let group = i32::try_from(leader.id()).expect("pid");
        let mut zombie = Command::new("true")
            .process_group(group)
            .stdin(Stdio::null())
            .spawn()
            .expect("true starts");
        // Not vacuous: the table shows a zombie in the group before the stop.
        let give_up = Instant::now() + Duration::from_secs(10);
        let seen = loop {
            let members = crate::verify::lifeline::group_members(group)
                .expect("the host has /proc or ps, so the process table can be read");
            if members.iter().any(|member| member.zombie) {
                break members;
            }
            if Instant::now() >= give_up {
                let _ = leader.kill();
                let _ = leader.wait();
                panic!("the exited child never showed as a zombie of the group");
            }
            std::thread::sleep(Duration::from_millis(20));
        };
        assert!(seen.iter().any(|member| !member.zombie), "the leader runs");
        let started = Instant::now();
        let outcome = super::stop(&mut leader);
        assert_eq!(outcome, Ok(()), "a zombie is not running");
        assert!(
            started.elapsed() < Duration::from_millis(1500),
            "the stop did not wait for the zombie: {:?}",
            started.elapsed()
        );
        // Nothing of the group is left once the zombie is reaped too. The
        // test reaps it itself if `stop` has not: where the group signal
        // skips zombies (macOS) `stop` returns before reaping, and that is
        // not what is under test (`stop` reaps it on Linux, and `wait` then
        // finds nothing left, which is not an error here).
        drop(zombie.wait());
        assert!(
            crate::verify::lifeline::group_members(group).is_some_and(|members| members.is_empty())
        );
    }
}
