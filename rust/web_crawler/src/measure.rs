use sysinfo::{Pid, ProcessExt, System, SystemExt};

pub fn print_mem_usage() {
    let system = System::new_all();
    let pid = std::process::id();
    if let Some(process) = system.process(Pid::from(pid as usize)) {
        println!("Alloc = {} MiB", format_byte_to_mb(process.memory()));
        println!("name {}", process.name());
    } else {
        println!("Could not find current process in the system");
    }
}

pub fn format_byte_to_mb(b: u64) -> u64 {
    b / 1024 / 1024
}
