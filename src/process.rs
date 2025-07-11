use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub command: Option<String>,
    pub args: Vec<String>,
    pub status: Option<i32>,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub duration: Option<Duration>,
    pub(crate) stat: Stat,
}
impl ProcessInfo {
    pub(crate) fn new() -> Self {
        ProcessInfo {
            pid: 0,
            command: None,
            args: vec![],
            status: Some(0),
            start_time: None,
            end_time: None,
            duration: None, //end_time.duration_since(start_time),
            stat: Stat::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub(crate) struct Stat {
    pub(crate) read_bytes: Vec<u64>,
    pub(crate) write_bytes: Vec<u64>,
    pub(crate) received: Vec<u64>,
    pub(crate) transmitted: Vec<u64>,
    pub(crate) utime: u64,
    pub(crate) stime: u64,
    pub(crate) memory_kb: Vec<u64>,
    pub(crate) total_time: u64,
}

impl Stat {
    pub(crate) fn new() -> Self {
        Stat {
            read_bytes: vec![],
            write_bytes: vec![],
            received: vec![],
            transmitted: vec![],
            utime: 0,
            stime: 0,
            memory_kb: vec![],
            total_time: 0,
        }
    }
}
