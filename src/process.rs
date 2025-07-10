use std::time::{Instant, Duration};

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

#[derive(Debug, Clone)]
pub(crate) struct Stat {
   pub(crate) read_bytes: Vec<u64>,
   pub(crate) write_bytes: Vec<u64>,
   pub(crate) received: Vec<u64>,
   pub(crate) transmitted: Vec<u64>,
   pub(crate) utime: Vec<u64>,
   pub(crate) stime: Vec<u64>,
   pub(crate) memory_kb: Vec<u64>,
   pub(crate) total_time: Vec<u64>,
}