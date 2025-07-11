
use shell_words;
use tokio::process::{Child, Command};
use tokio::time::Instant;
use std::process::Stdio;
use crate::process::Stat;
use crate::monitor::Monitor;
use crate::process::ProcessInfo;
use crate::linux::PollBased;

pub struct Executor {
    command: String,
}


impl Executor {
    pub fn new(command: String) -> Self {
        Self { command }
    }

    pub fn execute(&self) -> Result<ProcessInfo, Box<dyn std::error::Error>> {
        let parts = shell_words::split(&self.command)?;
        if parts.is_empty() {
            return Err("Empty command".into());
        }
        
        let program = parts[0].clone();
        let args = parts[1..].to_vec();

        let child = Command::new(program.clone())
        .stderr(Stdio::inherit()) 
        .stdin(Stdio::null())  
        .stdout(Stdio::null())
        .args(&args)
        .spawn()
        .map_err(|e| format!("Failed to spawn process: {}", e))?;
        
  
        let mut process_info = ProcessInfo::new();
        self.run(child, &mut process_info);   
        Ok(process_info.clone())
    }

 
     fn run(&self, mut child: Child, process_info: &mut ProcessInfo){
        process_info.pid = child.id().unwrap_or(0);
        let start_time = std::time::Instant::now();
        process_info.start_time = Some(start_time);
       
      
        let status = tokio::spawn(async move {
            child.wait().await
                .expect("child process encountered an error");
        });
     
        let mut monitor = PollBased::new(1);
        loop {
            if status.is_finished() {
                let end_time = std::time::Instant::now();
                process_info.end_time = Some(end_time);
                process_info.duration = Some(end_time.duration_since(start_time));
                break;
            }
            monitor.scan( process_info);
        }
       
    }
    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute() {
        let executor = Executor::new("echo 'Hello, world!'".to_string());
        let process_info = executor.execute().unwrap();
        assert_eq!(process_info.command, Some("echo".to_string()));
        assert_eq!(process_info.args, vec!["Hello, world!"]);
        assert_eq!(process_info.status, Some(0));
        std::process::exit(0);
    }
    
    #[tokio::test]
    async fn test_execute_with_args() {
        let executor = Executor::new("echo 'Hello, world!'".to_string());
        let process_info = executor.execute().unwrap();
        assert_eq!(process_info.command, Some("echo".to_string()));
        assert_eq!(process_info.args, vec!["Hello, world!"]);
        assert_eq!(process_info.status, Some(0));
        
        let start_elapsed = process_info.start_time.unwrap().elapsed().as_micros();
        let end_elapsed = process_info.end_time.unwrap().elapsed().as_micros();
        let duration = process_info.duration.unwrap().as_micros();
        
        assert!(start_elapsed > 0);
        assert!(end_elapsed > 0); 
        assert!(duration > 0);
        std::process::exit(0);
    }

    #[tokio::test]
    async fn test_execute_invalid_command_returns_error() {
        let executor = Executor::new("nonexistent_command_12345".to_string());
        let result = executor.execute();
        assert!(
            result.is_err(),
            "Expected error for invalid command, but got Ok"
        );
        std::process::exit(0);
    }
}