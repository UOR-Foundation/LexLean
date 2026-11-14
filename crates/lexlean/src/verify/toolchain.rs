//! Toolchain preflight (SPEC.md §8.2, §22.2): locate the pinned Lean
//! toolchain, require the exact reported version, and record executable
//! digests.

use camino::Utf8PathBuf;

use crate::artifact::content_id::Sha256Digest;
use crate::code;
use crate::config::Limits;
use crate::diagnostic::Diagnostic;
use crate::verify::child::{run as run_child, ChildHome, ChildSpec, Normalizer};

/// One preflighted tool.
#[derive(Debug, Clone)]
pub struct Tool {
    /// The absolute executable path.
    pub path: Utf8PathBuf,
    /// SHA-256 of the executable bytes.
    pub sha256: Sha256Digest,
    /// The recorded, normalized version output.
    pub version_output: String,
}

/// The pinned toolchain.
#[derive(Debug, Clone)]
pub struct Toolchain {
    /// The toolchain root directory.
    pub root: Utf8PathBuf,
    /// `lean`.
    pub lean: Tool,
    /// `lake`.
    pub lake: Tool,
    /// `leanchecker`.
    pub leanchecker: Tool,
}

/// The format of [`Addresses`] as a part of a verification writes it.
pub const ADDRESSES_SPEC: &str = "lexlean/verify-toolchain/1";

/// The addresses of every executable a verification ran or was built by
/// (SPEC.md §22.11): `lean`, `lake`, and `leanchecker` as the preflight
/// hashed them, the running `lexlean`, and the `rustc` that compiled it.
/// Two parts of one verification agree on all five or neither trusts the
/// other's records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Addresses {
    /// `lean`.
    pub lean: Sha256Digest,
    /// `lake`.
    pub lake: Sha256Digest,
    /// `leanchecker`.
    pub leanchecker: Sha256Digest,
    /// The running `lexlean` executable.
    pub lexlean: Sha256Digest,
    /// The `rustc` that compiled this `lexlean` (its executable and driver
    /// library, [`crate::build_identity::RUSTC_SHA256`]).
    pub rustc: Sha256Digest,
}

impl Addresses {
    /// The addresses of this preflighted toolchain, the running executable
    /// whose digest is `lexlean`, and the compiler that built it.
    #[must_use]
    pub fn of(toolchain: &Toolchain, lexlean: Sha256Digest) -> Self {
        Self {
            lean: toolchain.lean.sha256,
            lake: toolchain.lake.sha256,
            leanchecker: toolchain.leanchecker.sha256,
            lexlean,
            rustc: Sha256Digest::from_hex(crate::build_identity::RUSTC_SHA256)
                .expect("the build script writes a SHA-256"),
        }
    }

    /// Every tool with its address, in name order.
    #[must_use]
    pub fn tools(&self) -> [(&'static str, Sha256Digest); 5] {
        [
            ("lake", self.lake),
            ("lean", self.lean),
            ("leanchecker", self.leanchecker),
            ("lexlean", self.lexlean),
            ("rustc", self.rustc),
        ]
    }

    /// The canonical JSON form.
    #[must_use]
    pub fn to_json(&self) -> crate::artifact::canonical_json::Json {
        use crate::artifact::canonical_json::Json;
        let mut pairs = vec![("spec", Json::Str(ADDRESSES_SPEC.to_owned()))];
        for (tool, address) in self.tools() {
            pairs.push((tool, Json::Str(address.to_hex())));
        }
        Json::object(pairs)
    }

    /// Read the canonical JSON form, refusing anything else: another
    /// format, a missing or malformed address, or an unknown field.
    ///
    /// # Errors
    ///
    /// What is wrong with `json`.
    pub fn from_json(json: &crate::artifact::canonical_json::Json) -> Result<Self, String> {
        use crate::artifact::canonical_json::Json;
        let Json::Obj(fields) = json else {
            return Err("the toolchain addresses are an object".to_owned());
        };
        if fields.get("spec") != Some(&Json::Str(ADDRESSES_SPEC.to_owned())) {
            return Err(format!("`spec` must be `{ADDRESSES_SPEC}`"));
        }
        let address = |tool: &str| match fields.get(tool) {
            Some(Json::Str(hex)) => Sha256Digest::from_hex(hex)
                .map_err(|reason| format!("the address of `{tool}`: {reason}")),
            _ => Err(format!("the address of `{tool}` is required")),
        };
        let addresses = Self {
            lean: address("lean")?,
            lake: address("lake")?,
            leanchecker: address("leanchecker")?,
            lexlean: address("lexlean")?,
            rustc: address("rustc")?,
        };
        if let Some(unknown) = fields
            .keys()
            .find(|key| *key != "spec" && !addresses.tools().iter().any(|(tool, _)| tool == key))
        {
            return Err(format!("unknown field `{unknown}`"));
        }
        Ok(addresses)
    }

    /// The first tool whose address differs between `self` and `other`, with
    /// both addresses, in name order.
    #[must_use]
    pub fn first_difference(
        &self,
        other: &Self,
    ) -> Option<(&'static str, Sha256Digest, Sha256Digest)> {
        self.tools()
            .into_iter()
            .zip(other.tools())
            .find(|((_, mine), (_, theirs))| mine != theirs)
            .map(|((tool, mine), (_, theirs))| (tool, mine, theirs))
    }
}

fn environment(message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code!("LLV7001"), message)
}

/// The elan toolchain directory for the pinned toolchain string: `/` maps
/// to `--` and `:` to `---` under `$ELAN_HOME/toolchains` or
/// `$HOME/.elan/toolchains`.
pub fn toolchain_root() -> Result<Utf8PathBuf, Diagnostic> {
    let elan_home = std::env::var_os("ELAN_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|home| std::path::PathBuf::from(home).join(".elan"))
        })
        .ok_or_else(|| environment("neither ELAN_HOME nor HOME is set"))?;
    let mangled = crate::LEAN_TOOLCHAIN.replace('/', "--").replace(':', "---");
    let root = elan_home.join("toolchains").join(mangled);
    Utf8PathBuf::from_path_buf(root).map_err(|bad| crate::project::non_utf8_path(&bad))
}

/// The version banner of a tool under the allow-list environment: stdout,
/// or stderr when stdout is empty. The probe is a child like any other
/// (§25.5), so it runs through the shared runner and is bounded by the
/// configured `child_timeout_ms` and `max_child_output_bytes`: a toolchain
/// executable that hangs or floods its banner is `LLS8002`, never an
/// unbounded read. The banner carries no path, so the normalizer's
/// line-ending and trailing-blank rules are the whole normalization.
fn version_probe(
    name: &str,
    path: &Utf8PathBuf,
    argv: &[&str],
    bin_dir: &Utf8PathBuf,
    sha256: Sha256Digest,
    limits: &Limits,
) -> Result<String, Diagnostic> {
    let record = run_child(
        &ChildSpec {
            tool: name,
            module: None,
            program: path,
            executable_sha256: sha256,
            argv: argv.iter().map(|argument| (*argument).to_owned()).collect(),
            cwd: bin_dir,
            extra_env: Vec::new(),
            home: ChildHome::Toolchain {
                toolchain_bin: bin_dir,
            },
        },
        limits,
        &Normalizer::default(),
    )?;
    let stdout = record.stdout.trim_end().to_owned();
    Ok(if stdout.trim().is_empty() {
        record.stderr.trim_end().to_owned()
    } else {
        stdout
    })
}

fn load_tool(
    bin_dir: &Utf8PathBuf,
    name: &str,
    version_argv: Option<&[&str]>,
    limits: &Limits,
) -> Result<Tool, Diagnostic> {
    let path = bin_dir.join(name);
    let bytes = std::fs::read(path.as_std_path()).map_err(|io_error| {
        environment(format!(
            "the pinned toolchain has no `{name}` executable: {path}: {io_error}"
        ))
    })?;
    let sha256 = Sha256Digest::of(&bytes);
    let version_output = match version_argv {
        Some(argv) => version_probe(name, &path, argv, bin_dir, sha256, limits)?,
        None => String::new(),
    };
    Ok(Tool {
        path,
        sha256,
        version_output,
    })
}

/// Locate and preflight the pinned toolchain: `lean` and `lake` must report
/// version 4.32.2 (§8.2); every executable digest is recorded (§22.9). The
/// version probes run under the configured child limits (§25.5).
pub fn preflight(limits: &Limits) -> Result<Toolchain, Diagnostic> {
    let root = toolchain_root()?;
    let bin_dir = root.join("bin");
    if !bin_dir.as_std_path().is_dir() {
        return Err(environment(format!(
            "the pinned toolchain `{}` is not installed at {root}",
            crate::LEAN_TOOLCHAIN
        )));
    }
    let lean = load_tool(&bin_dir, "lean", Some(&["--version"]), limits)?;
    if !reports_pinned_version("lean", &lean.version_output) {
        return Err(environment(format!(
            "lean reports `{}`, not version {}",
            lean.version_output,
            crate::LEAN_VERSION
        )));
    }
    let lake = load_tool(&bin_dir, "lake", Some(&["--version"]), limits)?;
    if !reports_pinned_version("lake", &lake.version_output) {
        return Err(environment(format!(
            "lake reports `{}`, not Lean version {}",
            lake.version_output,
            crate::LEAN_VERSION
        )));
    }
    // `leanchecker` has no version flag (any first argument is a module
    // name), so its recorded identity is the executable's path relative to
    // the toolchain root and its digest; `load_tool` already required the
    // file to exist and hashed it. The replay runs themselves (§22.4) are
    // the functional check.
    let mut leanchecker = load_tool(&bin_dir, "leanchecker", None, limits)?;
    leanchecker.version_output = format!(
        "leanchecker bin/leanchecker sha256:{}",
        leanchecker.sha256.to_hex()
    );
    Ok(Toolchain {
        root,
        lean,
        lake,
        leanchecker,
    })
}

pub(crate) fn reports_pinned_version(tool: &str, banner: &str) -> bool {
    if banner.contains(['\n', '\r']) {
        return false;
    }
    // Lake has its own version: only the enclosed Lean version identifies
    // the kernel. Match the whole field so patch and prerelease suffixes
    // cannot satisfy the pin by sharing its prefix.
    let version = match tool {
        "lean" => banner
            .strip_prefix("Lean (version ")
            .and_then(|rest| rest.strip_suffix(')'))
            .and_then(|rest| rest.split_once(", "))
            .filter(|(_, details)| !details.is_empty())
            .map(|(version, _)| version),
        "lake" => banner
            .strip_prefix("Lake version ")
            .and_then(|rest| rest.strip_suffix(')'))
            .and_then(|rest| rest.split_once(" (Lean version "))
            .filter(|(lake, _)| !lake.is_empty() && !lake.contains(char::is_whitespace))
            .map(|(_, version)| version),
        _ => None,
    };
    version == Some(crate::LEAN_VERSION)
}

#[cfg(test)]
mod tests {
    use super::reports_pinned_version;

    #[test]
    fn version_banners_require_the_exact_lean_version() {
        for (tool, banner) in [
            (
                "lean",
                "Lean (version 4.32.2, x86_64-unknown-linux-gnu, commit f3b06c7, Release)",
            ),
            (
                "lake",
                "Lake version 5.0.0-src+f3b06c7 (Lean version 4.32.2)",
            ),
        ] {
            assert!(reports_pinned_version(tool, banner), "{tool}: {banner}");
        }
        for (tool, banner) in [
            ("lean", "Lean (version 4.32.1, release)"),
            ("lean", "Lean (version 4.32.20, release)"),
            ("lean", "Lean (version 4.32.10, release)"),
            ("lean", "Lean (version 4.32.2-rc1, release)"),
            ("lean", "Lean (version 4.31.0, built with 4.32.2)"),
            ("lean", "unrecognized banner 4.32.2"),
            ("lean", "Lean (version 4.32.2, release"),
            ("lean", "Lean (version 4.32.2, release)\nextra output"),
            ("lake", "Lake version 5.0.0 (Lean version 4.32.1)"),
            ("lake", "Lake version 5.0.0 (Lean version 4.32.20)"),
            ("lake", "Lake version 5.0.0 (Lean version 4.32.10)"),
            ("lake", "Lake version 5.0.0 (Lean version 4.32.2-rc1)"),
            ("lake", "Lake version 4.32.2 (Lean version 4.31.0)"),
            ("lake", "Lake version 5.0.0 (Lean version 4.32.2"),
            (
                "lake",
                "Lake version 5.0.0 (Lean version 4.32.2)\nextra output",
            ),
        ] {
            assert!(!reports_pinned_version(tool, banner), "{tool}: {banner}");
        }
    }
}
