#![cfg(unix)]
//! Helpers shared by the process-group tests: the process table, read the
//! way an operator would, and a script that starts a long-running child.

use std::path::Path;

/// The processes that are running and whose command line holds `marker`.
/// A zombie is not running: nothing is left of it to end. The table is
/// `/proc` where the host has one and `ps` otherwise (macOS), so the case is
/// not vacuous on a host without `/proc`; a host with neither is an error,
/// because an empty answer there would read as "nothing is left".
pub fn running(marker: &str) -> Vec<(i32, String)> {
    if Path::new("/proc/self").exists() {
        return std::fs::read_dir("/proc")
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|entry| {
                let pid = entry.file_name().to_str()?.parse::<i32>().ok()?;
                let command = std::fs::read(format!("/proc/{pid}/cmdline")).ok()?;
                let command = String::from_utf8_lossy(&command).replace('\0', " ");
                let status = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
                let zombie = status.rsplit(") ").next()?.starts_with('Z');
                (command.contains(marker) && !zombie).then_some((pid, command))
            })
            .collect();
    }
    // An absolute path: a test may have replaced `PATH`.
    let ps = ["/bin/ps", "/usr/bin/ps"]
        .into_iter()
        .find(|path| Path::new(path).is_file())
        .expect("the host has neither /proc nor ps, so the process table cannot be read");
    let output = std::process::Command::new(ps)
        .args(["-axo", "pid=,stat=,command="])
        .output()
        .expect("ps runs");
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let pid = fields.next()?.parse::<i32>().ok()?;
            let state = fields.next()?;
            let command = fields.collect::<Vec<_>>().join(" ");
            (command.contains(marker) && !state.starts_with('Z')).then_some((pid, command))
        })
        .collect()
}

/// End what a failed assertion would otherwise leave behind.
pub fn end_all(left: &[(i32, String)]) {
    for (pid, _) in left {
        if let Some(pid) = rustix::process::Pid::from_raw(*pid) {
            let _ = rustix::process::kill_process(pid, rustix::process::Signal::KILL);
        }
    }
}

/// An executable script.
pub fn script(path: &Path, body: &str) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::write(path, body).expect("write");
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).expect("chmod");
}
