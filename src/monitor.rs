use crate::process::ProcessInfo;


pub trait Monitor {
    fn new(pid: u32) -> Self;
    fn read_cpu_usage(&mut self,  proces_info: &mut ProcessInfo);
    fn read_memory_usage(&mut self,  proces_info: &mut ProcessInfo);
    fn read_network_usage(&mut self,  proces_info: &mut ProcessInfo);
    fn read_disk_usage(&mut self, proces_info: &mut ProcessInfo);
    fn scan(&mut self, proces_info: &mut ProcessInfo) {
        self.read_cpu_usage(proces_info);
        self.read_memory_usage(proces_info);
        self.read_network_usage(proces_info);
        self.read_disk_usage(proces_info);
    }
}
