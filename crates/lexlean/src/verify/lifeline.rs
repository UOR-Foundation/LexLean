//! Children that lead a process group of their own, and the thread that ends
//! them when this process is interrupted.
//!
//! A child is started in a group of its own so that a timeout can end what it
//! started (`lake` starts `lean`). The cost is that the terminal's Ctrl-C,
//! which reaches the foreground group only, no longer reaches the child, and
//! neither does the SIGTERM that `timeout(1)` or a service manager sends to
//! this process: the children would be left running at full speed with
//! nothing to end them. The thread below receives SIGINT, SIGTERM and SIGHUP,
//! ends every live group, and then lets the signal do what it would have done
//! (the process is terminated by it, so the exit status is the usual one).
//! SIGKILL cannot be caught by any process; a `lexlean` killed that way
//! leaves its children, which is the one case this does not cover.

use std::process::{Child, Command};
#[cfg(unix)]
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

#[cfg(unix)]
use rustix::process::{kill_process_group, Pid, Signal};
#[cfg(unix)]
use signal_hook::consts::{SIGHUP, SIGINT, SIGTERM};
#[cfg(unix)]
use signal_hook::iterator::Signals;

/// The process groups of the children that are running now.
#[cfg(unix)]
static LIVE: Mutex<Vec<Pid>> = Mutex::new(Vec::new());
#[cfg(unix)]
static INSTALLED: OnceLock<Result<(), String>> = OnceLock::new();

#[cfg(unix)]
fn live() -> MutexGuard<'static, Vec<Pid>> {
    // A poisoned list is still the list: ending the children is the one
    // thing that must not depend on some other thread having finished.
    LIVE.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Install the handler once per process.
#[cfg(unix)]
fn install() -> Result<(), String> {
    INSTALLED
        .get_or_init(|| {
            let mut signals = Signals::new([SIGINT, SIGTERM, SIGHUP])
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
