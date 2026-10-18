//! Children that lead a process group of their own, the thread that ends
//! them when the executable is interrupted, and the public function a host
//! that embeds the library ends them with (SPEC.md §17.17, §24.6).
//!
//! A child is started in a group of its own so that a timeout can end what it
//! started (`lake` starts `lean`). The cost is that the terminal's Ctrl-C,
//! which reaches the foreground group only, no longer reaches the child, and
//! neither does the SIGTERM that `timeout(1)` or a service manager sends to
//! this process: the children would be left running at full speed with
//! nothing to end them. The thread below receives SIGINT, SIGTERM, SIGHUP,
//! and SIGQUIT, ends every live group, and then lets the signal do what it
//! would have done (the process is terminated by it, so the exit status is the
//! usual one). SIGKILL cannot be caught by any process; a `lexlean` killed
//! that way leaves its children, which is the one case this does not cover.
//!
//! Two things keep the thread from doing more than that. It is armed by the
//! executable's `main` and by nothing else: a host that embeds the library
//! owns its own signals and its own termination, and this crate installs no
//! handler for it, changes no signal disposition, and never ends it. What the
//! library gives the host instead is [`end_live_children`]: called from the
//! host's own signal-handling thread or shutdown path (an ordinary thread,
//! never from inside a signal handler: it is not async-signal-safe), it ends
//! every live group through the same code the thread uses, reports each
//! group, and refuses later child starts until the host calls
//! [`allow_children`]. And the thread watches only the signals that were not
//! ignored when the process began, so a `nohup lexlean verify` that was told
//! to ignore SIGHUP is not ended by one.

use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};

#[cfg(unix)]
use std::sync::OnceLock;

#[cfg(unix)]
use rustix::io::Errno;
#[cfg(unix)]
use rustix::process::{kill_process_group, test_kill_process_group, Pid, Signal};
#[cfg(unix)]
use signal_hook::consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
#[cfg(unix)]
use signal_hook::iterator::Signals;

/// The process groups of the children that are running now (unix), or their
/// process ids (windows, where a tree is ended by its root).
#[cfg(unix)]
type Group = Pid;
#[cfg(not(unix))]
type Group = u32;
static LIVE: Mutex<Vec<Group>> = Mutex::new(Vec::new());
/// Set by [`end_live_children`]: from then on a new child is refused until
/// the host calls [`allow_children`].
static SHUTDOWN: AtomicBool = AtomicBool::new(false);
#[cfg(unix)]
static INSTALLED: OnceLock<Result<(), String>> = OnceLock::new();
/// Set by the interrupt thread on its first signal: the death it then
/// performs (by the signal, or its fallback exit) is the process's end, and
/// `main`'s own exit waits for it rather than racing it to a plain code.
#[cfg(unix)]
static INTERRUPT: AtomicBool = AtomicBool::new(false);
/// Set by the executable's `main` before anything runs.
#[cfg(unix)]
static ARMED: AtomicBool = AtomicBool::new(false);
/// The signals that were ignored when the process began, as the kernel's
/// mask (bit `n - 1` for signal `n`); `None` when the host cannot say.
#[cfg(unix)]
static IGNORED: OnceLock<Option<u64>> = OnceLock::new();

fn live() -> MutexGuard<'static, Vec<Group>> {
    // A poisoned list is still the list: ending the children is the one
    // thing that must not depend on some other thread having finished.
    LIVE.lock().unwrap_or_else(PoisonError::into_inner)
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
                        INTERRUPT.store(true, Ordering::SeqCst);
                        // The one implementation of "end the live groups"
                        // (§24.6); the executable is its first user.
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

/// The outcome of [`end_live_children`] for one child process group the
/// library had started (SPEC.md §24.6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupEnd {
    /// The group was live and is now ended.
    Ended,
    /// The group had already exited.
    AlreadyGone,
    /// The group could not be ended; the OS error, reported, never
    /// swallowed.
    Failed(String),
}

/// One group the library had started and not seen exit, and what became of
/// it in the call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChildGroupEnd {
    /// The process-group number (unix), or the process id at the root of the
    /// ended tree (windows).
    pub group: u32,
    /// What became of the group.
    pub outcome: GroupEnd,
}

/// End every child process group the library has started in this process and
/// has not seen exit, reporting each (SPEC.md §24.6). Idempotent, and a
/// no-op (an empty return) when no child is live.
///
/// After the call, starting a child in this process fails with `LLV7018`
/// until [`allow_children`], so a child started concurrently with the call
/// is either ended by it or refused; none is left running. The library
/// installs no signal handler and changes no signal disposition by this
/// call, and the host's own process and process group are never signalled:
/// the groups ended are exactly the ones the library registered.
///
/// Callable from any ordinary thread. Not async-signal-safe: it takes a
/// lock, waits, and reaps, so it must not run inside a signal handler.
pub fn end_live_children() -> Vec<ChildGroupEnd> {
    let taken = {
        let mut live = live();
        // Under the one lock that registers a child: a spawn that won the
        // lock first is in the list taken here and is ended below; one that
        // comes after sees the shutdown state and is refused. Nothing slips
        // between the two.
        SHUTDOWN.store(true, Ordering::SeqCst);
        std::mem::take(&mut *live)
    };
    end_groups(&taken)
}

/// Re-enable starting children after [`end_live_children`]: the explicit
/// re-enable of the host (SPEC.md §24.6).
pub fn allow_children() {
    SHUTDOWN.store(false, Ordering::SeqCst);
}

/// The process exit of the executable's `main`. When the interrupt thread
/// is ending the children, its death by the signal (or its fallback exit) is
/// the process's end: `main` waits for it rather than racing it to a plain
/// exit code, so an interrupted run keeps the exit status of the signal.
/// Everywhere else the code exits exactly as `std::process::exit` does.
pub fn exit_at_end(code: i32) -> ! {
    #[cfg(unix)]
    if INTERRUPT.load(Ordering::SeqCst) {
        loop {
            std::thread::park();
        }
    }
    std::process::exit(code);
}

/// End each live group, each by the one function that ends one.
fn end_groups(live: &[Group]) -> Vec<ChildGroupEnd> {
    live.iter()
        .map(|group| {
            let id = group_id(group);
            ChildGroupEnd {
                group: id,
                outcome: end_leader(id),
            }
        })
        .collect()
}

#[cfg(unix)]
fn group_id(group: &Group) -> u32 {
    u32::try_from(group.as_raw_pid()).unwrap_or(u32::MAX)
}

#[cfg(not(unix))]
fn group_id(group: &Group) -> u32 {
    *group
}

/// What the signal's answer already settles: no such group is already gone,
/// and any other refusal is reported with the OS error, never taken for
/// success. `None` says the signal was delivered and the end is confirmed
/// below.
#[cfg(unix)]
pub(crate) fn signal_outcome(group: Pid, signalled: Result<(), Errno>) -> Option<GroupEnd> {
    match signalled {
        Ok(()) => None,
        Err(Errno::SRCH) => Some(GroupEnd::AlreadyGone),
        Err(errno) => Some(GroupEnd::Failed(format!(
            "the process group {} could not be signalled: {}",
            group.as_raw_pid(),
            std::io::Error::from(errno)
        ))),
    }
}

/// End the group a child leads (unix) or the tree it roots (windows), and
/// confirm it. This is the one place anything is signalled to end children:
/// [`end_live_children`] ends every live group through it and so does a run
/// that ends its own child at its timeout, so the host-driven end, the
/// executable's interrupt thread, and the timeout cannot disagree about what
/// ending a child means, and a conformance check can count the sites that
/// signal a group (`conformance_cl_22`).
///
/// The OS's refusal is the outcome, never success (SPEC.md §24.6).
#[cfg(unix)]
pub(crate) fn end_leader(id: u32) -> GroupEnd {
    let Some(group) = i32::try_from(id).ok().and_then(Pid::from_raw) else {
        return GroupEnd::Failed(format!("process {id} has no usable group number"));
    };
    signal_outcome(group, kill_process_group(group, Signal::KILL))
        .unwrap_or_else(|| confirm_ended(group))
}

/// The signal was delivered; the end is confirmed the way a timeout's stop
/// confirms it. The group exists while any process of it does, and a zombie
/// is one: a zombie is ended, not running, so what counts is a member that
/// is not one. The zombies this process is the parent of are reaped with a
/// targeted `waitpid` on exactly those (a host that runs as PID 1, as the
/// shipped image does, inherits the grandchildren of the leaders it ends);
/// never a wait-for-any, which could take a child of the host that is not
/// this crate's to reap. The group's leader is left for the run that owns
/// its `Child` handle.
#[cfg(unix)]
pub(crate) fn confirm_ended(group: Pid) -> GroupEnd {
    let leader = u32::try_from(group.as_raw_pid()).unwrap_or(0);
    for _ in 0..200 {
        if test_kill_process_group(group) == Err(Errno::SRCH) {
            return GroupEnd::Ended;
        }
        if let Some(members) = group_members(group.as_raw_pid()) {
            let mut alive = false;
            for member in members {
                if !member.zombie {
                    alive = true;
                } else if member.parent == std::process::id() && member.pid != leader {
                    if let Some(pid) = i32::try_from(member.pid).ok().and_then(Pid::from_raw) {
                        let _ = rustix::process::waitpid(
                            Some(pid),
                            rustix::process::WaitOptions::NOHANG,
                        );
                    }
                }
            }
            if !alive {
                return GroupEnd::Ended;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    GroupEnd::Failed(format!(
        "processes of group {} still existed 2000 ms after SIGKILL",
        group.as_raw_pid()
    ))
}

/// A process of a group, as the process table shows it.
#[cfg(unix)]
pub(crate) struct Member {
    /// Its process id.
    pub(crate) pid: u32,
    /// Its parent's process id.
    pub(crate) parent: u32,
    /// Whether it is a zombie, which is ended, not running.
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
            let mut fields = stat.rsplit_once(')')?.1.split_whitespace();
            let state = fields.next()?;
            let parent = fields.next()?.parse::<u32>().ok()?;
            let pgrp = fields.next()?.parse::<i32>().ok()?;
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

/// End the tree rooted at `id` on windows, and report a `taskkill` that
/// failed like a group that could not be ended (the one place a tree is
/// ended; see the unix function of the same name).
///
/// `taskkill /T /F` exits 128 alike for a tree that is gone, for a process
/// that has exited while its handle is still open (the library's `Child` holds
/// it), and for access denied, so its exit status cannot tell a tree that has
/// already exited, which is not a failure (SPEC.md §24.6), from one that could
/// not be ended. Whether `tasklist` still lists the process can.
#[cfg(windows)]
pub(crate) fn end_leader(id: u32) -> GroupEnd {
    let killed = Command::new("taskkill")
        .args(["/PID", &id.to_string(), "/T", "/F"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
    let listed = match &killed {
        Ok(status) if status.success() => None,
        _ => process_listed(id),
    };
    tree_outcome(killed, listed)
}

/// Whether `tasklist` still shows the process `id`; `None` when it cannot say
/// (it did not run, or it failed).
#[cfg(windows)]
fn process_listed(id: u32) -> Option<bool> {
    let output = Command::new("tasklist")
        .args(["/FI", &format!("PID eq {id}"), "/FO", "CSV", "/NH"])
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()
        .filter(|output| output.status.success())?;
    let quoted = format!("\"{id}\"");
    Some(
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .any(|line| line.split(',').nth(1) == Some(quoted.as_str())),
    )
}

/// What `taskkill` did, as the outcome of ending a process tree. `listed` is
/// whether `tasklist` still shows the root process, `None` when it could not
/// say. A success is an ended tree; a failure of a process that is no longer
/// listed is a tree that was already gone; a tool that could not be run, and
/// one that ran and failed (access denied) while the process is listed or
/// `tasklist` cannot say, are both reported, never taken for success.
#[cfg(any(windows, test))]
pub(crate) fn tree_outcome(
    killed: std::io::Result<std::process::ExitStatus>,
    listed: Option<bool>,
) -> GroupEnd {
    match killed {
        Ok(status) if status.success() => GroupEnd::Ended,
        _ if listed == Some(false) => GroupEnd::AlreadyGone,
        Ok(status) => GroupEnd::Failed(format!("taskkill failed: {status}")),
        Err(io_error) => GroupEnd::Failed(format!("taskkill could not be run: {io_error}")),
    }
}

/// Why a child could not be started.
#[derive(Debug)]
pub enum CannotStart {
    /// The host ended the live children and has not re-enabled new ones
    /// (SPEC.md §24.6; `LLV7018` to the caller).
    Shutdown,
    /// The OS refused the spawn, or the interrupt watcher could not be
    /// installed.
    Failed(String),
}

/// A registered group, forgotten when the child has been waited for (a
/// group number that is no longer in use may be given to someone else) or
/// ended by [`end_live_children`].
#[cfg(unix)]
pub struct Guard(Pid);

#[cfg(unix)]
impl Drop for Guard {
    fn drop(&mut self) {
        live().retain(|group| *group != self.0);
    }
}

/// Start the child and register its group in one step under the lock the
/// handler and [`end_live_children`] take: an interrupt that arrives between
/// the two waits for the registration, and so ends the child, and a child
/// concurrent with an [`end_live_children`] is either ended by it or refused.
///
/// # Errors
///
/// [`CannotStart::Shutdown`] when the host ended the live children and has
/// not re-enabled new ones, otherwise the reason the child could not be
/// started or watched.
#[cfg(unix)]
pub fn spawn(command: &mut Command) -> Result<(Child, Guard), CannotStart> {
    install().map_err(CannotStart::Failed)?;
    let mut groups = live();
    if SHUTDOWN.load(Ordering::SeqCst) {
        return Err(CannotStart::Shutdown);
    }
    let mut child = command
        .spawn()
        .map_err(|io_error| CannotStart::Failed(format!("cannot start: {io_error}")))?;
    let Some(group) = i32::try_from(child.id()).ok().and_then(Pid::from_raw) else {
        let _ = child.kill();
        let _ = child.wait();
        return Err(CannotStart::Failed(format!(
            "process {} has no usable group number",
            child.id()
        )));
    };
    groups.push(group);
    Ok((child, Guard(group)))
}

/// A registered child, forgotten when it has been waited for or ended.
/// Windows children share the console, which delivers Ctrl-C to them.
#[cfg(not(unix))]
pub struct Guard(u32);

#[cfg(not(unix))]
impl Drop for Guard {
    fn drop(&mut self) {
        live().retain(|pid| *pid != self.0);
    }
}

/// Start the child and register it, under the same shutdown rule as on unix.
///
/// # Errors
///
/// [`CannotStart::Shutdown`] when the host ended the live children and has
/// not re-enabled new ones, otherwise the reason it could not be started.
#[cfg(not(unix))]
pub fn spawn(command: &mut Command) -> Result<(Child, Guard), CannotStart> {
    let mut live = live();
    if SHUTDOWN.load(Ordering::SeqCst) {
        return Err(CannotStart::Shutdown);
    }
    let child = command
        .spawn()
        .map_err(|io_error| CannotStart::Failed(format!("cannot start: {io_error}")))?;
    let id = child.id();
    live.push(id);
    Ok((child, Guard(id)))
}

#[cfg(test)]
mod tests {
    // The cases hold their unix code inside the body: a test hidden by an
    // attribute is a defect of the repository (`conformance_rp_12`).

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

    /// The end of a group the OS refuses (a simulated `EPERM`) is reported
    /// with the error, never taken for success; a group with nothing left to
    /// signal is already gone; a delivered signal is confirmed, not assumed.
    #[test]
    fn a_refused_group_is_reported() {
        #[cfg(unix)]
        {
            use super::{signal_outcome, GroupEnd};
            use rustix::io::Errno;
            use rustix::process::Pid;
            let group = Pid::from_raw(4242).expect("a process id");
            let refused = signal_outcome(group, Err(Errno::PERM));
            let Some(GroupEnd::Failed(reason)) = refused else {
                panic!("EPERM is reported, not taken for success: {refused:?}")
            };
            assert!(reason.contains("could not be signalled"), "{reason}");
            assert!(reason.contains("Operation not permitted"), "{reason}");
            assert_eq!(
                signal_outcome(group, Err(Errno::SRCH)),
                Some(GroupEnd::AlreadyGone),
                "no such group is already gone"
            );
            assert_eq!(
                signal_outcome(group, Ok(())),
                None,
                "a delivered signal is confirmed below"
            );
        }
    }
}
