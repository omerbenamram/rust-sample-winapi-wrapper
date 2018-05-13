extern crate proc_list;
use proc_list::process_enumerator::{ProcessEnumerator, ToolHelp32Snapshot};

#[cfg(windows)]
fn print_running_processes() -> Result<(), &'static str> {
    let snapshot_handle = ToolHelp32Snapshot::new()?;
    let process_list = ProcessEnumerator::new(&snapshot_handle)?;

    for process in process_list {
        println!("{:?}", process)
    }
    Ok(())
}

#[cfg(not(windows))]
fn print_running_processes() -> Result<(), &'static str> {
    Err("Only Windows is supported.")
}

fn main() {
    print_running_processes().unwrap();
}
