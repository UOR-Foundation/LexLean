//! Children that lead a process group of their own, the one function that
//! ends them, and the thread that calls it when the executable is
//! interrupted (SPEC.md §17.17, §24.6).
//!
//! A child is started in a group of its own so that a timeout can end what it
//! started (`lake` starts `lean`). The cost is that the terminal's Ctrl-C,
//! which reaches the foreground group only, no longer reaches the child, and
//! neither does the SIGTERM that `timeout(1)` or a service manager sends to
//! the process that started it: the children would be left running at full
//! speed with nothing to end them.
//!
//! [`end_live_children`] is how they are ended. Every child the library starts
//! is registered here, under one lock, for as long as it has not been seen to
//! exit; the function ends every registered group and reports what became of
//! each. It is public because the process that ends them is not always the
//! executable: a program that embeds the library (a service, a build tool, a
//! test harness) owns its own signals and its own shutdown, so the library
//! installs no handler for it, changes no signal disposition, and never
//! signals or terminates it. The host decides when the children end (from its
//! own signal handling, its own shutdown path, or a timeout of its own) and
//! calls the function; the library provides the means and nothing else.
//!
//! The call also puts the library in a shutdown state: from then on a child
//! start is refused with `LLV7018` until the host calls
//! [`allow_new_children`]. A start takes the same lock as the call and checks
//! the state, starts the child, and registers it before releasing it, so a
//! child started concurrently with the call is either registered before it
//! (and ended by it) or refused after it; none is left running.
//!
//! The executable is the function's first user. Its `main` arms a thread that
//! receives SIGINT, SIGTERM, SIGHUP, and SIGQUIT, calls [`end_live_children`],
//! and then lets the signal do what it would have done (the process is
//! terminated by it, so the exit status is the usual one). SIGKILL cannot be
//! caught by any process; a `lexlean` killed that way leaves its children,
//! which is the one case this does not cover. The thread watches only the
//! signals that were not ignored when the process began, so a
//! `nohup lexlean verify` that was told to ignore SIGHUP is not ended by one.
//! A timeout of one child ends its group through the same [`end_groups`],
//! so there is exactly one implementation of ending a group or a tree.

use std::process::{Child, Command};
#[cfg(unix)]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(unix)]
use std::sync::OnceLock;
use std::sync::{Mutex, MutexGuard, PoisonError};

#[cfg(unix)]
use rustix::process::{kill_process_group, test_kill_process_group, Pid, Signal};
#[cfg(unix)]
use signal_hook::consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
#[cfg(unix)]
use signal_hook::iterator::Signals;

/// What became of one child process group (unix) or process tree (windows)
/// that [`end_live_children`] ended.
#[derive(Debug)]
pub enum GroupOutcome {
    /// It was signalled, and no process of it is running any more. A process
    /// of the group that is a zombie has ended: nothing is left of it to end.
    /// On Windows, `taskkill` ended the tree: it begins the termination of
    /// each of its processes, which the system completes.
    Ended,
    /// Nothing of it was running when the call came: it had already exited
    /// and was not yet seen to by the thread that started it.
    AlreadyGone,
    /// It could not be ended, and its processes may still be running: the
    /// group could not be signalled (`EPERM`, for example), was still there
    /// two seconds after `SIGKILL`, or `taskkill` failed or could not be run.
    /// The error is the operating system's where it gave one.
    Failed(std::io::Error),
}

/// One group and its outcome.
#[derive(Debug)]
pub struct EndedGroup {
    /// The process group, which is the process ID of the child the library
    /// started and which leads it (unix), or that child's process ID, the
    /// root of its tree (windows).
    pub group: u32,
    /// What became of it.
    pub outcome: GroupOutcome,
}

/// What [`end_live_children`] did: one entry per group the library had
/// started and not seen exit, in the order they were started. Empty when no
/// child was live.
#[derive(Debug, Default)]
#[must_use = "a group that could not be ended is reported here and nowhere else"]
pub struct EndReport {
    /// Every group the call found, with its outcome.
    pub groups: Vec<EndedGroup>,
}

impl EndReport {
    /// Whether every group was ended or already gone, so that nothing the
    /// library started is left running.
    #[must_use]
    pub fn all_ended(&self) -> bool {
        self.failures().next().is_none()
    }

    /// The groups that could not be ended.
    pub fn failures(&self) -> impl Iterator<Item = &EndedGroup> {
        self.groups
            .iter()
            .filter(|group| matches!(group.outcome, GroupOutcome::Failed(_)))
    }
}

/// A child the library started and has not seen exit.
struct Live {
    id: u32,
    /// The host has ended it, so that its run reports that rather than the
    /// tool's failure.
    ended_by_host: bool,
}

/// The children that are running now, and whether the host has ended them.
struct Registry {
    live: Vec<Live>,
    shut_down: bool,
}

static REGISTRY: Mutex<Registry> = Mutex::new(Registry {
    live: Vec::new(),
    shut_down: false,
});
#[cfg(unix)]
static INSTALLED: OnceLock<Result<(), String>> = OnceLock::new();
/// Set by the executable's `main` before anything runs.
#[cfg(unix)]
static ARMED: AtomicBool = AtomicBool::new(false);
/// The signals that were ignored when the process began, as the kernel's
/// mask (bit `n - 1` for signal `n`); `None` when the host cannot say.
#[cfg(unix)]
static IGNORED: OnceLock<Option<u64>> = OnceLock::new();

fn registry() -> MutexGuard<'static, Registry> {
    // A poisoned registry is still the registry: ending the children is the
    // one thing that must not depend on some other thread having finished.
    REGISTRY.lock().unwrap_or_else(PoisonError::into_inner)
}

/// End every child process group the library has started in this process
/// and not yet seen exit, and refuse every child start after it until
/// [`allow_new_children`].
///
/// Each group is killed (`SIGKILL` to the group, through the audited
/// signalling dependency and never a `kill` executable) and awaited for up to
/// two seconds; on Windows the child's process tree is ended with
/// `taskkill /T /F`. The report says, per group, whether it was ended, had
/// already exited, or could not be ended and why; a failure is never taken
/// for success. A call with no live child signals nothing, and a second call
/// finds the groups the first ended gone (or no group at all, once the
/// threads that started them have seen them exit).
///
/// The library installs no signal handler for this, changes no signal
/// disposition, and never signals the calling process or its process group:
/// a registered group that is the host's own is refused with
/// `PermissionDenied` rather than signalled. Zombies of the group that are
/// children of this process are reaped by process ID, never by a wait for any
/// child, and never the child that leads the group, which the thread that
/// started it waits for; a host that waits for its own children is not
/// disturbed.
///
/// # Thread safety
///
/// It may be called from any ordinary thread, concurrently with child starts
/// and with itself. It is **not** async-signal-safe: it takes a lock,
/// allocates, and may run `ps` or `taskkill`, so it must not be called from
/// inside a signal handler. A host that ends the children on a signal calls
/// it from a thread that its handler wakes (as `signal-hook`'s iterator
/// does, and as the `lexlean` executable does).
///
/// # Shutdown state
///
/// The call enters the shutdown state before it ends anything, under the
/// same lock a child start registers its child under: a start that came
/// first is ended by the call, and one that comes after it is refused with
/// `LLV7018`, as is every later start until the host calls
/// [`allow_new_children`]. A run whose child the call ended returns
/// `LLV7018` as well, rather than the tool's own failure.
pub fn end_live_children() -> EndReport {
    let mut registry = registry();
    registry.shut_down = true;
    let ids: Vec<u32> = registry
        .live
        .iter_mut()
        .map(|live| {
            live.ended_by_host = true;
            live.id
        })
        .collect();
    // The lock is held while the groups end: a start waits and is then
    // refused, and a group is not forgotten by its thread meanwhile.
    let outcomes = end_groups(&ids);
    drop(registry);
    EndReport {
        groups: ids
            .into_iter()
            .zip(outcomes)
            .map(|(group, outcome)| EndedGroup { group, outcome })
            .collect(),
    }
}

/// Leave the shutdown state that [`end_live_children`] entered: child starts
/// are allowed again. A host calls it when it means to use the library again
/// after ending its children; until then every start is refused with
/// `LLV7018`. It has no effect outside the shutdown state.
pub fn allow_new_children() {
    registry().shut_down = false;
}

/// Arm the thread: from now on the first child started installs it. Called by
/// the executable's `main` (before any signal disposition is changed, so that
/// the dispositions it reads are the ones the process was started with), and
/// by nothing else.
pub fn arm() {
    #[cfg(unix)]
    {
        let _ = IGNORED.get_or_init(ignored_at_start);
        ARMED.store(true, Ordering::SeqCst);
    }
}

/// The mask of signals ignored now: `SigIgn` of `/proc/self/status` where the
/// host has `/proc`, the `ignored` column of `ps` where it does not (macOS).
#[cfg(unix)]
fn ignored_at_start() -> Option<u64> {
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        return status.lines().find_map(|line| {
            let mask = line.strip_prefix("SigIgn:")?;
            u64::from_str_radix(mask.trim(), 16).ok()
        });
    }
    // An absolute path: nothing is searched for on `PATH`. BSD `ps` spells
    // the column `ignored` or `sigignore` depending on the system, and may
    // print the mask with a `0x` prefix; a host that does none of these
    // cannot say, which `watched` handles.
    let ps = ["/bin/ps", "/usr/bin/ps"]
        .into_iter()
        .find(|path| std::path::Path::new(path).is_file())?;
    ["ignored", "sigignore"].into_iter().find_map(|keyword| {
        let output = Command::new(ps)
            .args([
                "-o",
                &format!("{keyword}="),
                "-p",
                &std::process::id().to_string(),
            ])
            .stdin(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .output()
            .ok()?;
        let text = String::from_utf8_lossy(&output.stdout);
        let digits = text.trim();
        let digits = digits.strip_prefix("0x").unwrap_or(digits);
        u64::from_str_radix(digits, 16).ok()
    })
}

/// The signals to watch: the four that end a run, less the ones the process
/// was started ignoring. Where the host cannot say what is ignored, SIGHUP is
/// left alone (the one a `nohup` ignores), because ending a run that was told
/// to survive its terminal is worse than leaving the children of one that
/// was not.
#[cfg(unix)]
fn watched(ignored: Option<u64>) -> Vec<i32> {
    let is_ignored = |signal: i32| ignored.is_some_and(|mask| mask & (1_u64 << (signal - 1)) != 0);
    [SIGINT, SIGTERM, SIGHUP, SIGQUIT]
        .into_iter()
        .filter(|signal| !is_ignored(*signal))
        .filter(|signal| ignored.is_some() || *signal != SIGHUP)
        .collect()
}

/// Install the handler once per process, when the executable armed it.
#[cfg(unix)]
fn install() -> Result<(), String> {
    if !ARMED.load(Ordering::SeqCst) {
        return Ok(());
    }
    INSTALLED
        .get_or_init(|| {
            let signals = watched(IGNORED.get().copied().flatten());
            if signals.is_empty() {
                return Ok(());
            }
            let mut signals = Signals::new(signals)
                .map_err(|error| format!("cannot watch for interruption: {error}"))?;
            std::thread::Builder::new()
                .name("lexlean-interrupt".to_owned())
                .spawn(move || {
                    // The first signal ends the process, so there is no second.
                    if let Some(signal) = signals.forever().next() {
                        // The public function, not a path of its own: the
                        // process is about to end, so the report has no reader.
                        let _ = end_live_children();
                        // Restores the default action and raises the
                        // signal again, which ends this process.
                        let _ = signal_hook::low_level::emulate_default_handler(signal);
                        std::process::exit(128_i32.saturating_add(signal));
                    }
                })
                .map(|_| ())
                .map_err(|error| format!("cannot watch for interruption: {error}"))
        })
        .clone()
}

/// A registered child, forgotten when it has been waited for (a group number
/// that is no longer in use may be given to someone else).
pub struct Guard(u32);

impl Guard {
    /// Whether [`end_live_children`] ended this child.
    pub fn ended_by_host(&self) -> bool {
        registry()
            .live
            .iter()
            .any(|live| live.id == self.0 && live.ended_by_host)
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        registry().live.retain(|live| live.id != self.0);
    }
}

/// Why a child was not started.
pub enum Refusal {
    /// The host ended the live children and has not allowed new ones.
    ShutDown,
    /// It could not be started or watched, and why.
    Failed(String),
}

/// Start the child and register it in one step under the lock that
/// [`end_live_children`] takes: a call that arrives between the two waits for
/// the registration, and so ends the child, and a start that arrives after
/// the call is refused.
///
/// # Errors
///
/// [`Refusal::ShutDown`] in the shutdown state, and the reason it could not
/// be started or watched otherwise.
pub fn spawn(command: &mut Command) -> Result<(Child, Guard), Refusal> {
    #[cfg(unix)]
    install().map_err(Refusal::Failed)?;
    let mut registry = registry();
    if registry.shut_down {
        return Err(Refusal::ShutDown);
    }
    let child = command
        .spawn()
        .map_err(|io_error| Refusal::Failed(format!("cannot start: {io_error}")))?;
    let id = child.id();
    #[cfg(unix)]
    if i32::try_from(id).ok().and_then(Pid::from_raw).is_none() {
        let mut child = child;
        let _ = child.kill();
        let _ = child.wait();
        return Err(Refusal::Failed(format!(
            "process {id} has no usable group number"
        )));
    }
    registry.live.push(Live {
        id,
        ended_by_host: false,
    });
    Ok((child, Guard(id)))
}

/// End each group (unix) or process tree (windows) and say what became of
/// it, one outcome per ID in order. The one implementation of ending a child
/// and what it started: [`end_live_children`] and a child's timeout both
/// come here.
///
/// On unix every group is signalled first and then all are awaited together,
/// for two seconds at most. A group whose processes are all zombies has
/// ended; the zombies that are children of this process are reaped by
/// process ID, except the leader, which belongs to the `Child` that started
/// it (reaping it here would let its ID be reused under that `Child`).
pub(crate) fn end_groups(ids: &[u32]) -> Vec<GroupOutcome> {
    #[cfg(unix)]
    {
        let host = rustix::process::getpid();
        let host_group = rustix::process::getpgrp();
        let mut outcomes: Vec<Option<GroupOutcome>> = ids
            .iter()
            .map(|id| signal_group(*id, host, host_group))
            .collect();
        for _ in 0..200 {
            let mut waiting = false;
            for (id, outcome) in ids.iter().zip(outcomes.iter_mut()) {
                if outcome.is_none() {
                    if group_gone(*id) {
                        *outcome = Some(GroupOutcome::Ended);
                    } else {
                        waiting = true;
                    }
                }
            }
            if !waiting {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        ids.iter()
            .zip(outcomes)
            .map(|(id, outcome)| {
                outcome.unwrap_or_else(|| {
                    GroupOutcome::Failed(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        format!("processes of group {id} still existed 2000 ms after SIGKILL"),
                    ))
                })
            })
            .collect()
    }
    #[cfg(windows)]
    {
        ids.iter().map(|id| end_tree(*id)).collect()
    }
}

/// Signal one group, or say why it was not: `None` when it was signalled and
/// is to be awaited.
#[cfg(unix)]
fn signal_group(id: u32, host: Pid, host_group: Pid) -> Option<GroupOutcome> {
    let Some(group) = i32::try_from(id).ok().and_then(Pid::from_raw) else {
        return Some(GroupOutcome::Failed(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("process {id} has no usable group number"),
        )));
    };
    // A child's ID is never this process's, nor (the kernel does not give a
    // process the number of a group in use) its group's; this says so where
    // the group is signalled rather than relying on it.
    if group == host || group == host_group {
        return Some(GroupOutcome::Failed(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!("group {id} is this process's own, which is never signalled"),
        )));
    }
    if running(group) == Some(false) {
        return Some(GroupOutcome::AlreadyGone);
    }
    after_signal(kill_process_group(group, Signal::KILL))
}

/// The outcome a group signal decides: none when it was sent (the group is
/// then awaited), gone when there was no such group, and the operating
/// system's error otherwise.
#[cfg(unix)]
fn after_signal(sent: Result<(), rustix::io::Errno>) -> Option<GroupOutcome> {
    match sent {
        Ok(()) => None,
        Err(rustix::io::Errno::SRCH) => Some(GroupOutcome::AlreadyGone),
        Err(errno) => Some(GroupOutcome::Failed(std::io::Error::from(errno))),
    }
}

/// Whether nothing of the group is running: there is no such group, or
/// every process of it is a zombie.
#[cfg(unix)]
fn group_gone(id: u32) -> bool {
    let Some(group) = i32::try_from(id).ok().and_then(Pid::from_raw) else {
        return true;
    };
    test_kill_process_group(group) == Err(rustix::io::Errno::SRCH) || running(group) == Some(false)
}

/// Whether any process of the group is running, reaping on the way the
/// zombies of it that are children of this process, by ID, other than its
/// leader; `None` when the process table cannot be read.
///
/// The group exists while any process of it does, and a zombie is one: under
/// a PID 1 that never reaps (the shipped image runs `lexlean` as PID 1) the
/// killed grandchildren stay zombies for good. A zombie is ended, so what
/// counts is a member that is not one.
#[cfg(unix)]
fn running(group: Pid) -> Option<bool> {
    let members = group_members(group.as_raw_pid())?;
    let mut alive = false;
    for member in members {
        if !member.zombie {
            alive = true;
        } else if member.parent == std::process::id()
            && member.pid != group.as_raw_pid().unsigned_abs()
        {
            if let Some(pid) = i32::try_from(member.pid).ok().and_then(Pid::from_raw) {
                let _ = rustix::process::waitpid(Some(pid), rustix::process::WaitOptions::NOHANG);
            }
        }
    }
    Some(alive)
}

/// A process of a group, as the process table shows it.
#[cfg(unix)]
pub(crate) struct Member {
    pub(crate) pid: u32,
    pub(crate) parent: u32,
    pub(crate) zombie: bool,
}

/// The processes of a process group, read from `/proc` where the host has it
/// and from `ps` where it does not (macOS); `None` when neither can be read.
#[cfg(unix)]
pub(crate) fn group_members(group: i32) -> Option<Vec<Member>> {
    if std::path::Path::new("/proc/self").exists() {
        let mut members = Vec::new();
        for entry in std::fs::read_dir("/proc").ok()?.flatten() {
            let Some(pid) = entry
                .file_name()
                .to_str()
                .and_then(|n| n.parse::<u32>().ok())
            else {
                continue;
            };
            // A process that ended meanwhile has no `stat` any more.
            let Ok(stat) = std::fs::read_to_string(format!("/proc/{pid}/stat")) else {
                continue;
            };
            // `pid (comm) S ppid pgrp ...`: `comm` may hold spaces and
            // parentheses, so the fields are counted from the last `)`.
            let Some((_, fields)) = stat.rsplit_once(')') else {
                continue;
            };
            let mut fields = fields.split_whitespace();
            let (Some(state), Some(parent), Some(pgrp)) =
                (fields.next(), fields.next(), fields.next())
            else {
                continue;
            };
            let (Ok(parent), Ok(pgrp)) = (parent.parse::<u32>(), pgrp.parse::<i32>()) else {
                continue;
            };
            if pgrp == group {
                members.push(Member {
                    pid,
                    parent,
                    zombie: state.starts_with('Z'),
                });
            }
        }
        return Some(members);
    }
    // An absolute path: nothing is searched for on `PATH`.
    let ps = ["/bin/ps", "/usr/bin/ps"]
        .into_iter()
        .find(|path| std::path::Path::new(path).is_file())?;
    let output = Command::new(ps)
        .args(["-axo", "pid=,ppid=,pgid=,stat="])
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let mut members = Vec::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let mut fields = line.split_whitespace();
        let (Some(pid), Some(parent), Some(pgid), Some(state)) =
            (fields.next(), fields.next(), fields.next(), fields.next())
        else {
            continue;
        };
        if pgid.parse::<i32>().ok() == Some(group) {
            members.push(Member {
                pid: pid.parse().ok()?,
                parent: parent.parse().ok()?,
                zombie: state.starts_with('Z'),
            });
        }
    }
    Some(members)
}

/// End one process tree with `taskkill /T /F`. A `taskkill` that fails is
/// ended only if the process is no longer listed (it exits 128 both for a
/// process that is gone and for one it was denied).
#[cfg(windows)]
fn end_tree(id: u32) -> GroupOutcome {
    use std::process::Stdio;
    if id == std::process::id() {
        return GroupOutcome::Failed(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!("process {id} is this process, which is never ended"),
        ));
    }
    let killed = Command::new("taskkill")
        .args(["/PID", &id.to_string(), "/T", "/F"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    let listed = match &killed {
        Ok(status) if status.success() => None,
        _ => Command::new("tasklist")
            .args(["/FI", &format!("PID eq {id}"), "/FO", "CSV", "/NH"])
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .ok()
            .filter(|output| output.status.success())
            .map(|output| {
                let quoted = format!("\"{id}\"");
                String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .any(|line| line.split(',').nth(1) == Some(quoted.as_str()))
            }),
    };
    taskkill_outcome(killed, listed)
}

/// What `taskkill` did, as the outcome of ending a process tree: a success
/// ended it; a failure is gone only when the process is not listed, and is
/// otherwise reported, whether it ran and failed (access denied) or could
/// not be run, never taken for success. `listed` is whether `tasklist`
/// still shows the process, `None` when it could not say.
#[cfg(any(windows, test))]
fn taskkill_outcome(
    killed: std::io::Result<std::process::ExitStatus>,
    listed: Option<bool>,
) -> GroupOutcome {
    match killed {
        Ok(status) if status.success() => GroupOutcome::Ended,
        _ if listed == Some(false) => GroupOutcome::AlreadyGone,
        Ok(status) => {
            GroupOutcome::Failed(std::io::Error::other(format!("taskkill failed: {status}")))
        }
        Err(io_error) => GroupOutcome::Failed(std::io::Error::new(
            io_error.kind(),
            format!("taskkill could not be run: {io_error}"),
        )),
    }
}

#[cfg(test)]
mod tests {
    // The cases hold their unix code inside the body: a test hidden by an
    // attribute is a defect of the repository (`conformance_rp_12`). None of
    // them calls `end_live_children`, whose shutdown state is the process's:
    // the library's other tests start children in this process. The
    // conformance case CL-22 runs it in processes of its own.

    use super::GroupOutcome;

    #[test]
    fn an_ignored_signal_is_not_watched() {
        #[cfg(unix)]
        {
            use super::watched;
            use signal_hook::consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
            let bit = |signal: i32| 1_u64 << (signal - 1);
            assert_eq!(
                watched(Some(0)),
                vec![SIGINT, SIGTERM, SIGHUP, SIGQUIT],
                "nothing ignored: all four"
            );
            assert_eq!(
                watched(Some(bit(SIGHUP))),
                vec![SIGINT, SIGTERM, SIGQUIT],
                "nohup"
            );
            assert_eq!(
                watched(Some(bit(SIGINT) | bit(SIGQUIT))),
                vec![SIGTERM, SIGHUP],
                "a background job of a non-interactive shell"
            );
            assert_eq!(
                watched(Some(
                    bit(SIGINT) | bit(SIGTERM) | bit(SIGHUP) | bit(SIGQUIT)
                )),
                Vec::<i32>::new()
            );
            assert_eq!(
                watched(None),
                vec![SIGINT, SIGTERM, SIGQUIT],
                "a host that cannot say leaves SIGHUP alone"
            );
        }
    }

    #[test]
    fn the_mask_of_this_process_is_read() {
        // Not vacuous where the mask is knowable: with `/proc` it must be
        // reported, and a test process does not ignore SIGTERM. Where only
        // `ps` can say (macOS), a host whose `ps` has no such column reports
        // nothing, which `watched` handles (the case above); a mask that is
        // reported must still be right.
        #[cfg(unix)]
        {
            let mask = super::ignored_at_start();
            if std::path::Path::new("/proc/self/status").is_file() {
                assert!(mask.is_some(), "the host reports the ignored signals");
            }
            if let Some(mask) = mask {
                assert_eq!(
                    mask & (1_u64 << (signal_hook::consts::SIGTERM - 1)),
                    0,
                    "{mask:x}"
                );
            }
        }
    }

    /// A group that cannot be signalled is reported with the operating
    /// system's error, never as ended; no such group is gone; a group that
    /// was signalled is awaited.
    #[test]
    fn a_signal_error_is_the_outcome() {
        #[cfg(unix)]
        {
            use super::{after_signal, EndReport, EndedGroup};
            use rustix::io::Errno;
            assert!(
                after_signal(Ok(())).is_none(),
                "a signalled group is awaited"
            );
            assert!(matches!(
                after_signal(Err(Errno::SRCH)),
                Some(GroupOutcome::AlreadyGone)
            ));
            let Some(GroupOutcome::Failed(error)) = after_signal(Err(Errno::PERM)) else {
                panic!("EPERM is a failure");
            };
            assert_eq!(error.raw_os_error(), Some(Errno::PERM.raw_os_error()));
            let report = EndReport {
                groups: vec![
                    EndedGroup {
                        group: 10,
                        outcome: GroupOutcome::Ended,
                    },
                    EndedGroup {
                        group: 11,
                        outcome: after_signal(Err(Errno::PERM)).expect("decided"),
                    },
                ],
            };
            assert!(!report.all_ended(), "a failure is not swallowed");
            assert_eq!(
                report
                    .failures()
                    .map(|group| group.group)
                    .collect::<Vec<_>>(),
                [11]
            );
        }
    }

    /// The host's own process and group are refused, not signalled.
    #[test]
    fn the_hosts_own_group_is_never_signalled() {
        #[cfg(unix)]
        {
            let host = rustix::process::getpid();
            let group = rustix::process::getpgrp();
            for id in [host, group] {
                let outcome = super::signal_group(id.as_raw_pid().unsigned_abs(), host, group);
                let Some(GroupOutcome::Failed(error)) = outcome else {
                    panic!("{id:?} was not refused: {outcome:?}");
                };
                assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
            }
        }
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

    /// A `taskkill` that ran and failed is reported, like one that could not
    /// be run, unless the process it was to end is no longer listed: it
    /// exits 128 both for a process that is gone and for one it was denied.
    #[test]
    fn a_failing_taskkill_is_reported() {
        use super::taskkill_outcome;
        assert!(matches!(
            taskkill_outcome(Ok(exit_status(0)), None),
            GroupOutcome::Ended
        ));
        for listed in [Some(true), None] {
            let GroupOutcome::Failed(failed) = taskkill_outcome(Ok(exit_status(128)), listed)
            else {
                panic!("exit 128 with the process listed ({listed:?}) is a failure");
            };
            assert!(failed.to_string().contains("taskkill failed"), "{failed}");
        }
        assert!(matches!(
            taskkill_outcome(Ok(exit_status(128)), Some(false)),
            GroupOutcome::AlreadyGone
        ));
        let GroupOutcome::Failed(missing) = taskkill_outcome(
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "absent")),
            None,
        ) else {
            panic!("a taskkill that could not be run is a failure");
        };
        assert!(
            missing.to_string().contains("could not be run"),
            "{missing}"
        );
        assert_eq!(missing.kind(), std::io::ErrorKind::NotFound);
    }
}
