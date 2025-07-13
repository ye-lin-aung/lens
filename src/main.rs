mod benchmark;
mod executor;
mod linux;
mod monitor;
mod process;

use crate::process::ProcessInfo;
use clap::Parser;
use executor::Executor;

use crate::benchmark::Benchmark;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    warm: u8,

    #[arg(short, long, default_value_t = 5)]
    iter: u8,

    #[arg(required = true)]
    commands: Vec<String>,
}
/// Command to use lens -w 3 "ruby a.rb" "another command to compare"
/// lens -w 3 "ruby"
/// CPU, Memory, Network, Disk
/// curl -s "asd | bash
///
/// lens rails server   
///
/// lens rails runner app/jobs/something.rb
/// lens gzip file.txt
/// CPU, Memory, Network, Disk  
fn show_sys_info() {
    println!("\n=== System Information ===");

    // CPU Info
    // TODO: Support for physical core
    let cpu = sys_info::cpu_num().unwrap_or(0);
    let cpu_speed = sys_info::cpu_speed().unwrap_or(0);
    println!("CPUs: {}  cores, {} at MHz", cpu, cpu_speed);
    const GB_CONVERSION: f64 = 1024.0 * 1024.0;

    // Memory Info
    if let Ok(mem) = sys_info::mem_info() {
        println!(
            "Memory: {:.1} GB total, {:.1} GB free",
            mem.total as f64 / GB_CONVERSION,
            mem.free as f64 / GB_CONVERSION
        );
    }

    // Disk Info
    if let Ok(disk) = sys_info::disk_info() {
        println!(
            "Disk: {:.1} GB total, {:.1} GB free",
            disk.total as f64 / GB_CONVERSION,
            disk.free as f64 / GB_CONVERSION
        );
    }

    // OS Info
    if let Ok(os) = sys_info::os_type() {
        if let Ok(release) = sys_info::os_release() {
            println!("\nOS: {} {}", os, release);
        }
    }

    // Process Info
    if let Ok(proc_total) = sys_info::proc_total() {
        println!("Total Processes: {}", proc_total);
    }

    println!("=====================\n");
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut processes = Vec::new();

    show_sys_info();

    for command in args.commands {
        let mut command_processes = Vec::new();
        for _ in 0..args.warm {
            let _ = Executor::new(command.clone()).execute();
        }
        for _ in 0..args.iter {
            command_processes.push(Executor::new(command.clone()).execute());
        }
        processes.push(command_processes);
    }

    for command_processes in processes {
        let processes: Vec<ProcessInfo> = command_processes
            .into_iter()
            .filter_map(|p| p.ok())
            .collect();
        // Need to fix this to calculate each process
        let mut benchmarks = Vec::new();
        for process in processes {
            let command = process.command.clone();
            let args = process.args.join(" ");
            println!("\nCommand: {}", command);
            println!("Arguments: {}", args);
            benchmarks.push(Benchmark::calculate(process));
            println!("\nBenchmark Statistics:");
            println!("---------------------");
            println!("CPU Usage:");
            println!(
                "  User Time:   {:.1}% (min: {:.2}ms, avg: {:.2}ms, max: {:.2}ms)",
                benchmarks.last().unwrap().utime_percentage,
                benchmarks.last().unwrap().min_utime,
                benchmarks.last().unwrap().average_utime,
                benchmarks.last().unwrap().max_utime
            );
            println!(
                "  System Time: {:.1}% (min: {:.2}ms, avg: {:.2}ms, max: {:.2}ms)",
                benchmarks.last().unwrap().stime_percentage,
                benchmarks.last().unwrap().min_stime,
                benchmarks.last().unwrap().average_stime,
                benchmarks.last().unwrap().max_stime
            );
            println!("\nMemory Usage:");
            println!(
                "  Min:     {:.1} MB",
                benchmarks.last().unwrap().min_memory / 1024.0
            );
            println!(
                "  Average: {:.1} MB",
                benchmarks.last().unwrap().average_memory / 1024.0
            );
            println!(
                "  Max:     {:.1} MB",
                benchmarks.last().unwrap().max_memory / 1024.0
            );
            println!("\nExecution Time:");
            println!(
                "  Min:     {:.3} sec",
                benchmarks.last().unwrap().min_duration
            );
            println!(
                "  Average: {:.3} sec",
                benchmarks.last().unwrap().average_duration
            );
            println!(
                "  Max:     {:.3} sec",
                benchmarks.last().unwrap().max_duration
            );
            println!("---------------------\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_default() {
        let args = Args::try_parse_from(["test", "ruby a.rb"]).unwrap();
        assert_eq!(args.warm, 0);
    }

    #[test]
    fn test_args_custom_warm() {
        let args = Args::try_parse_from(["test", "-w", "3", "ruby a.rb"]).unwrap();
        assert_eq!(args.warm, 3);
    }

    #[test]
    fn test_args_custom_iter() {
        let args = Args::try_parse_from(["test", "-i", "5", "ruby a.rb"]).unwrap();
        assert_eq!(args.iter, 5);
    }

    #[test]
    fn test_args_giving_multiple_commands() {
        let args =
            Args::try_parse_from(["test", "ruby a.rb", "another command to compare"]).unwrap();
        assert_eq!(args.commands.len(), 2);
        assert_eq!(args.commands[0], "ruby a.rb");
        assert_eq!(args.commands[1], "another command to compare");
    }
}
