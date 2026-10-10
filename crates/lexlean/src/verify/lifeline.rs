//! Children that lead a process group of their own, and the thread that ends
//! them when this process is interrupted.
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
//! owns its own signals and its own termination, and is never ended by this
//! crate (the children it starts through the library are its to end). And it
//! watches only the signals that were not ignored when the process began, so
//! a `nohup lexlean verify` that was told to ignore SIGHUP is not ended by one.

use std::process::{Child, Command};
#[cfg(unix)]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(unix)]
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

#[cfg(unix)]
use rustix::process::{kill_process_group, Pid, Signal};
#[cfg(unix)]
use signal_hook::consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
#[cfg(unix)]
use signal_hook::iterator::Signals;

/// The process groups of the children that are running now.
#[cfg(unix)]
static LIVE: Mutex<Vec<Pid>> = Mutex::new(Vec::new());
#[cfg(unix)]
static INSTALLED: OnceLock<Result<(), String>> = OnceLock::new();
/// Set by the executable's `main` before anything runs.
#[cfg(unix)]
static ARMED: AtomicBool = AtomicBool::new(false);
/// The signals that were ignored when the process began, as the kernel's
/// mask (bit `n - 1` for signal `n`); `None` when the host cannot say.
#[cfg(unix)]
static IGNORED: OnceLock<Option<u64>> = OnceLock::new();

#[cfg(unix)]
fn live() -> MutexGuard<'static, Vec<Pid>> {
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
    // An absolute path: nothing is searched for on `PATH`.
    let ps = ["/bin/ps", "/usr/bin/ps"]
        .into_iter()
        .find(|path| std::path::Path::new(path).is_file())?;
    let output = Command::new(ps)
        .args(["-o", "ignored=", "-p", &std::process::id().to_string()])
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    u64::from_str_radix(String::from_utf8_lossy(&output.stdout).trim(), 16).ok()
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
                        for group in live().iter() {
                            let _ = kill_process_group(*group, Signal::KILL);
                        }
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

/// A registered group, forgotten when the child has been waited for (a
/// group number that is no longer in use may be given to someone else).
#[cfg(unix)]
pub struct Guard(Pid);

#[cfg(unix)]
impl Drop for Guard {
    fn drop(&mut self) {
        live().retain(|group| *group != self.0);
    }
}

/// Start the child and register its group in one step under the lock the
/// handler takes: an interrupt that arrives between the two waits for the
/// registration, and so ends the child.
///
/// # Errors
///
/// The reason it could not be started or watched.
#[cfg(unix)]
pub fn spawn(command: &mut Command) -> Result<(Child, Guard), String> {
    install()?;
    let mut groups = live();
    let mut child = command
        .spawn()
        .map_err(|io_error| format!("cannot start: {io_error}"))?;
    let Some(group) = i32::try_from(child.id()).ok().and_then(Pid::from_raw) else {
        let _ = child.kill();
        let _ = child.wait();
        return Err(format!("process {} has no usable group number", child.id()));
    };
    groups.push(group);
    Ok((child, Guard(group)))
}

/// Windows children share the console, which delivers Ctrl-C to them.
#[cfg(not(unix))]
pub struct Guard;

/// Start the child.
///
/// # Errors
///
/// The reason it could not be started.
#[cfg(not(unix))]
pub fn spawn(command: &mut Command) -> Result<(Child, Guard), String> {
    command
        .spawn()
        .map(|child| (child, Guard))
        .map_err(|io_error| format!("cannot start: {io_error}"))
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn an_ignored_signal_is_not_watched() {
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

    #[test]
    fn the_mask_of_this_process_is_read() {
        // Not vacuous: on a host with `/proc` or `ps` the mask is known, and
        // a test process does not ignore SIGTERM.
        let mask = ignored_at_start().expect("the host reports the ignored signals");
        assert_eq!(mask & (1_u64 << (SIGTERM - 1)), 0, "{mask:x}");
    }
}
