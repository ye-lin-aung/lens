use crate::process::ProcessInfo;
pub(crate) struct Benchmark;

#[derive(Debug)]
pub(crate) struct BenchmarkStat {
    min_ttime: f64,
    max_ttime: f64,
    average_ttime: f64,
    average_duration: f64,
    max_duration: f64,
    min_duration: f64,
    min_stime: f64,
    max_stime: f64,
    average_stime: f64,
    min_utime: f64,
    max_utime: f64,
    average_utime: f64,
    min_memory: f64,
    max_memory: f64,
    average_memory: f64,
    utime_percentage: f64,
    stime_percentage: f64,
    ttime_percentage: f64,
}

impl BenchmarkStat {
    fn new() -> Self {
        BenchmarkStat {
            min_ttime: 0.0,
            max_ttime: 0.0,
            average_ttime: 0.0,
            average_duration: 0.0,
            max_duration: 0.0,
            min_duration: 0.0,
            min_stime: 0.0,
            max_stime: 0.0,
            average_stime: 0.0,
            min_utime: 0.0,
            max_utime: 0.0,
            average_utime: 0.0,
            min_memory: 0.0,
            max_memory: 0.0,
            average_memory: 0.0,
            utime_percentage: 0.0,
            stime_percentage: 0.0,
            ttime_percentage: 0.0,
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
        let mut durations = Vec::new();

        for process in processes {
            memory_values.extend(process.stat.memory_kb);
            utimes.push(process.stat.utime);
            stimes.push(process.stat.stime);
            total_times.push(process.stat.total_time);
            if let Some(duration) = process.duration {
                durations.push(duration.as_secs_f64());
            }
        }
        benchmark.average_memory = Self::average(memory_values.clone());
        benchmark.max_memory = Self::max(memory_values.clone());
        benchmark.min_memory = Self::min(memory_values);

        benchmark.average_utime = Self::average(utimes.clone());
        benchmark.max_utime = Self::max(utimes.clone());
        benchmark.min_utime = Self::min(utimes);

        benchmark.average_stime = Self::average(stimes.clone());
        benchmark.max_stime = Self::max(stimes.clone());
        benchmark.min_stime = Self::min(stimes);

        benchmark.average_ttime = Self::average(total_times.clone());
        benchmark.max_ttime = Self::max(total_times.clone());
        benchmark.min_ttime = Self::min(total_times);

        benchmark.average_duration = durations.iter().sum::<f64>() / durations.len() as f64;
        benchmark.max_duration = *durations
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        benchmark.min_duration = *durations
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        // Calculate percentages
        let total_cpu_time = benchmark.average_utime + benchmark.average_stime;
        if total_cpu_time > 0.0 {
            benchmark.utime_percentage = (benchmark.average_utime / total_cpu_time) * 100.0;
            benchmark.stime_percentage = (benchmark.average_stime / total_cpu_time) * 100.0;
            benchmark.ttime_percentage = 100.0; // Total percentage is always 100%
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn create_test_process(
        utime: u64,
        stime: u64,
        memory: Vec<u64>,
        duration_secs: u64,
    ) -> ProcessInfo {
        let mut process = ProcessInfo::new();
        process.stat.utime = utime;
        process.stat.stime = stime;
        process.stat.memory_kb = memory;
        process.stat.total_time = utime + stime;
        process.duration = Some(Duration::from_secs(duration_secs));
        process.start_time = Some(Instant::now());
        process.end_time = Some(Instant::now() + Duration::from_secs(duration_secs));
        process
    }

    #[test]
    fn test_calculate_single_process() {
        let process = create_test_process(100, 50, vec![1000, 2000, 3000], 5);
        let processes = vec![process];

        let stats = Benchmark::calculate(processes);

        assert_eq!(stats.average_utime, 100.0);
        assert_eq!(stats.average_stime, 50.0);
        assert_eq!(stats.average_memory, 2000.0);
        assert_eq!(stats.min_memory, 1000.0);
        assert_eq!(stats.max_memory, 3000.0);
        assert_eq!(stats.average_duration, 5.0);
    }

    #[test]
    fn test_calculate_multiple_processes() {
        let process1 = create_test_process(100, 50, vec![1000], 5);
        let process2 = create_test_process(200, 100, vec![2000], 10);
        let processes = vec![process1, process2];

        let stats = Benchmark::calculate(processes);

        assert_eq!(stats.average_utime, 150.0);
        assert_eq!(stats.average_stime, 75.0);
        assert_eq!(stats.min_utime, 100.0);
        assert_eq!(stats.max_utime, 200.0);
        assert_eq!(stats.average_duration, 7.5);
    }

    #[test]
    fn test_cpu_percentages() {
        let process = create_test_process(75, 25, vec![1000], 5);
        let processes = vec![process];

        let stats = Benchmark::calculate(processes);

        assert_eq!(stats.utime_percentage, 75.0);
        assert_eq!(stats.stime_percentage, 25.0);
        assert_eq!(stats.ttime_percentage, 100.0);
    }

    #[test]
    fn test_helper_functions() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(Benchmark::average(nums.clone()), 3.0);
        assert_eq!(Benchmark::max(nums.clone()), 5.0);
        assert_eq!(Benchmark::min(nums), 1.0);
    }
}
