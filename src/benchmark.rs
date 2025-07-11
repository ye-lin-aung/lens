use crate::process::ProcessInfo;
pub(crate) struct Benchmark;

#[derive(Debug)]
pub(crate) struct BenchmarkStat {
    min_t: f64,
    max_t: f64,
    average_t: f64,
    average_duration: f64,
    max_duration: f64,
    min_duration: f64,
    min_cpu: f64,
    max_cpu: f64,
    average_cpu: f64,
    min_memory: f64,
    max_memory: f64,
    average_memory: f64,
}

impl BenchmarkStat {
    fn new() -> Self {
        BenchmarkStat {
            min_t: 0.0,
            max_t: 0.0,
            average_t: 0.0,
            average_duration: 0.0,
            max_duration: 0.0,
            min_duration: 0.0,
            min_cpu: 0.0,
            max_cpu: 0.0,
            average_cpu: 0.0,
            min_memory: 0.0,
            max_memory: 0.0,
            average_memory: 0.0,
        }
    }
}

// pub(crate) read_bytes: Vec<u64>,
// pub(crate) write_bytes: Vec<u64>,
// pub(crate) received: Vec<u64>,
// pub(crate) transmitted: Vec<u64>,
// pub(crate) utime: u64,
// pub(crate) stime: u64,
// pub(crate) memory_kb: Vec<u64>,
// pub(crate) total_time: u64,
// }
impl Benchmark {
    pub(crate) fn calculate(processes: Vec<ProcessInfo>) -> BenchmarkStat {
        let mut benchmark = BenchmarkStat::new();

        let mut memory_values = Vec::new();
        let mut utimes = Vec::new();
        let mut stimes = Vec::new();
        let mut total_times = Vec::new();
        for process in processes {
            memory_values.extend(process.stat.memory_kb);
            utimes.push(process.stat.utime);
            stimes.push(process.stat.utime);
            total_times.push(process.stat.total_time);
        }
        benchmark.average_memory = Self::average(memory_values.clone());
        benchmark.max_memory = Self::max(memory_values.clone());
        benchmark.min_memory = Self::min(memory_values);

        benchmark.average_cpu = Self::average(utimes.clone()) + Self::average(stimes.clone());
        benchmark.max_cpu = Self::max(utimes.clone()) + Self::max(stimes.clone());
        benchmark.min_cpu = Self::min(utimes.clone()) + Self::min(stimes.clone());

        benchmark.average_t = Self::average(total_times.clone());
        benchmark.max_t = Self::max(total_times.clone());
        benchmark.min_t = Self::min(total_times);

        benchmark
    }

    fn average(nums: Vec<u64>) -> f64 {
        nums.iter().sum::<u64>() as f64 / nums.len() as f64
    }

    fn max(nums: Vec<u64>) -> f64 {
        *nums.iter().max().unwrap() as f64
    }

    fn min(nums: Vec<u64>) -> f64 {
        *nums.iter().min().unwrap() as f64
    }
}
