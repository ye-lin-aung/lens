use crate::monitor::Monitor;
use crate::process::ProcessInfo;
use crate::process::Stat;

use std::fs::File;
use std::io::Read;

const CGROUP_UNIFIED: &str = "/sys/fs/cgroup/unified";
const CGROUP_V2: &str = "/sys/fs/cgroup/cgroup";
const CGROUP_CONTROLLER_PATH: &str = "cgroup.controllers";
const MAX_PATH: usize = 256;
const MAX_BUFFER: usize = 4096;

struct CgroupV2 {
    pid: u32,
}

pub(crate) struct PollBased {
    pid: u32,
}

impl Monitor for PollBased {
    fn new(pid: u32) -> Self {
        Self { pid }
    }

    fn read_cpu_usage(&mut self, process_info: &mut ProcessInfo) {
        let mut stat = String::new();
        if let Ok(mut f) = File::open(format!("/proc/{}/stat", self.pid)) {
            if f.read_to_string(&mut stat).is_ok() {
                // Split the stat content and get CPU usage fields
                let fields: Vec<&str> = stat.split_whitespace().collect();
                if fields.len() >= 15 {
                    // Fields 14 and 15 contain user and system CPU time
                    let utime = fields[13].parse::<u64>().unwrap_or(0);
                    let stime = fields[14].parse::<u64>().unwrap_or(0);
                    let total_time = utime + stime;

                    // Store individual readings rather than accumulating
                    process_info.stat.utime = utime;
                    process_info.stat.stime = stime;
                    process_info.stat.total_time = total_time;
                }
            }
        }
    }
    fn read_memory_usage(&mut self, process_info: &mut ProcessInfo) {
        let mut status = String::new();
        if let Ok(mut f) = File::open(format!("/proc/{}/status", self.pid)) {
            if f.read_to_string(&mut status).is_ok() {
                // Find VmRSS line which shows actual physical memory usage
                if let Some(line) = status.lines().find(|l| l.starts_with("VmRSS:")) {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    if fields.len() >= 2 {
                        let memory_kb = fields[1].parse::<u64>().unwrap_or(0);
                        process_info.stat.memory_kb.push(memory_kb);
                    }
                }
            }
        }
    }
    fn read_network_usage(&mut self, process_info: &mut ProcessInfo) {
        let mut net = String::new();
        if let Ok(mut f) = File::open(format!("/proc/{}/net/dev", self.pid)) {
            if f.read_to_string(&mut net).is_ok() {
                // Skip header lines and process network interface statistics
                for line in net.lines().skip(2) {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    if fields.len() >= 10 {
                        // let interface = fields[0].trim_end_matches(':');
                        let bytes_received = fields[1].parse::<u64>().unwrap_or(0);
                        let bytes_transmitted = fields[9].parse::<u64>().unwrap_or(0);
                        process_info.stat.read_bytes.push(bytes_received);
                        process_info.stat.transmitted.push(bytes_transmitted);
                    }
                }
            }
        }
    }
    fn read_disk_usage(&mut self, process_info: &mut ProcessInfo) {
        let mut io = String::new();
        if let Ok(mut f) = File::open(format!("/proc/{}/io", self.pid)) {
            if f.read_to_string(&mut io).is_ok() {
                // Process IO statistics
                for line in io.lines() {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    if fields.len() >= 2 {
                        match fields[0] {
                            "read_bytes:" => {
                                let read_bytes = fields[1].parse::<u64>().unwrap_or(0);
                                process_info.stat.read_bytes.push(read_bytes);
                            }
                            "write_bytes:" => {
                                let write_bytes = fields[1].parse::<u64>().unwrap_or(0);
                                process_info.stat.write_bytes.push(write_bytes);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_bytes_parsing() {
        let mut monitor = PollBased::new(std::process::id());
        let mut process_info = ProcessInfo::new();
        monitor.read_disk_usage(&mut process_info);
        // Verify read_bytes vector is populated
        assert!(!process_info.stat.read_bytes.is_empty());
    }

    #[test]
    fn test_write_bytes_parsing() {
        let mut monitor = PollBased::new(std::process::id());
        let mut process_info = ProcessInfo::new();
        monitor.read_disk_usage(&mut process_info);
        // Verify write_bytes vector is populated
        assert!(!process_info.stat.write_bytes.is_empty());
    }

    #[test]
    fn test_network_stats_parsing() {
        let mut monitor = PollBased::new(std::process::id());
        let mut process_info = ProcessInfo::new();
        monitor.read_network_usage(&mut process_info);
        // Verify network stats vectors are populated
        assert!(!process_info.stat.received.is_empty());
        assert!(!process_info.stat.transmitted.is_empty());
    }

    #[test]
    fn test_multiple_readings() {
        let mut monitor = PollBased::new(std::process::id());
        let mut process_info = ProcessInfo::new();
        // Take multiple readings
        for _ in 0..3 {
            monitor.read_cpu_usage(&mut process_info);
            monitor.read_memory_usage(&mut process_info);
            monitor.read_network_usage(&mut process_info);
            monitor.read_disk_usage(&mut process_info);
        }

        assert!(process_info.stat.read_bytes.len() >= 3);
        assert!(process_info.stat.transmitted.len() >= 3);
    }

    #[test]
    fn test_nonexistent_pid() {
        let mut monitor = PollBased::new(u32::MAX);
        let mut process_info = ProcessInfo {
            pid: 0,
            command: None,
            args: vec![],
            status: None,
            start_time: None,
            end_time: None,
            duration: None,
            stat: Stat::new(),
        };
        monitor.read_cpu_usage(&mut process_info);
        monitor.read_memory_usage(&mut process_info);
        monitor.read_network_usage(&mut process_info);
        monitor.read_disk_usage(&mut process_info);
        // Should handle nonexistent PID gracefully without panicking

        assert!(process_info.stat.memory_kb.is_empty());
        assert!(process_info.stat.total_time > 0);
    }

    #[test]
    fn test_monitor_creation() {
        let monitor = PollBased::new(1);
        assert_eq!(monitor.pid, 1);
    }

    #[test]
    fn test_read_cpu_usage() {
        let mut monitor = PollBased::new(std::process::id());
        let mut process_info = ProcessInfo::new();
        monitor.read_cpu_usage(&mut process_info);
        // Since we're reading our own process, this should execute without panicking
    }

    #[test]
    fn test_read_memory_usage() {
        let mut monitor = PollBased::new(std::process::id());
        let mut process_info = ProcessInfo::new();
        monitor.read_memory_usage(&mut process_info);
        // Since we're reading our own process, this should execute without panicking
    }

    #[test]
    fn test_read_network_usage() {
        let mut monitor = PollBased::new(std::process::id());
        let mut process_info = ProcessInfo::new();
        monitor.read_network_usage(&mut process_info);
        // Since we're reading our own process, this should execute without panicking
    }

    #[test]
    fn test_read_disk_usage() {
        let mut monitor = PollBased::new(std::process::id());
        let mut process_info = ProcessInfo::new();
        monitor.read_disk_usage(&mut process_info);
        // Since we're reading our own process, this should execute without panicking
    }

    #[test]
    fn test_invalid_pid() {
        let mut monitor = PollBased::new(0);
        let mut process_info = ProcessInfo::new();
        monitor.read_cpu_usage(&mut process_info); // Should handle invalid PID gracefully
        monitor.read_memory_usage(&mut process_info);
        monitor.read_network_usage(&mut process_info);
        monitor.read_disk_usage(&mut process_info);
    }
}
