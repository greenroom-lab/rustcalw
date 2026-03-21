//! Process liveness check — mirrors src/shared/pid-alive.ts

/// Check if a PID is a valid process identifier.
fn is_valid_pid(pid: u32) -> bool {
    pid > 0
}

/// Check if a process is a zombie on Linux by reading /proc/<pid>/status.
/// Returns false on non-Linux platforms or if the proc file can't be read.
#[cfg(target_os = "linux")]
fn is_zombie_process(pid: u32) -> bool {
    let path = format!("/proc/{}/status", pid);
    let Ok(status) = std::fs::read_to_string(&path) else {
        return false;
    };
    let re = regex::Regex::new(r"(?m)^State:\s+(\S)").unwrap();
    match re.captures(&status) {
        Some(caps) => caps.get(1).map_or(false, |m| m.as_str() == "Z"),
        None => false,
    }
}

#[cfg(not(target_os = "linux"))]
fn is_zombie_process(_pid: u32) -> bool {
    false
}

/// Check if a process with the given PID is alive.
///
/// Uses platform-specific methods:
/// - On Unix: sends signal 0 via `kill(pid, 0)` and checks for zombie status on Linux.
/// - On Windows: uses `OpenProcess` to check if the process handle is valid.
pub fn is_pid_alive(pid: u32) -> bool {
    if !is_valid_pid(pid) {
        return false;
    }
    if !process_exists(pid) {
        return false;
    }
    if is_zombie_process(pid) {
        return false;
    }
    true
}

#[cfg(unix)]
fn process_exists(pid: u32) -> bool {
    // kill(pid, 0) checks if the process exists without sending a signal.
    unsafe {
        libc::kill(pid as i32, 0) == 0
    }
}

#[cfg(windows)]
fn process_exists(pid: u32) -> bool {
    use std::process::Command;
    // Use tasklist to check if process exists — avoids heavy FFI dependencies
    match Command::new("tasklist")
        .args(["/FI", &format!("PID eq {}", pid), "/NH", "/FO", "CSV"])
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.contains(&pid.to_string())
        }
        Err(_) => false,
    }
}

#[cfg(not(any(unix, windows)))]
fn process_exists(_pid: u32) -> bool {
    false
}

/// Read the process start time (field 22 "starttime") from /proc/<pid>/stat.
/// Returns the value in clock ticks since system boot, or None on non-Linux
/// platforms or if the proc file can't be read.
///
/// This is used to detect PID recycling.
#[cfg(target_os = "linux")]
pub fn get_process_start_time(pid: u32) -> Option<u64> {
    if !is_valid_pid(pid) {
        return None;
    }
    let stat = std::fs::read_to_string(format!("/proc/{}/stat", pid)).ok()?;
    let comm_end = stat.rfind(')')?;
    let after_comm = stat[comm_end + 1..].trim_start();
    let fields: Vec<&str> = after_comm.split_whitespace().collect();
    // field 22 (starttime) = index 19 after the comm-split (field 3 is index 0).
    let starttime: u64 = fields.get(19)?.parse().ok()?;
    Some(starttime)
}

#[cfg(not(target_os = "linux"))]
pub fn get_process_start_time(_pid: u32) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_pid_zero() {
        assert!(!is_pid_alive(0));
    }

    #[test]
    fn start_time_on_non_linux() {
        // On Windows/macOS, this should return None
        #[cfg(not(target_os = "linux"))]
        assert!(get_process_start_time(1).is_none());
    }
}
