use std::mem;

use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, PROCESSENTRY32, Process32First,
                           Process32Next, TH32CS_SNAPPROCESS};
use winapi::um::winnt::HANDLE;

use utils::nul_terminated_i8_arr_to_string;

pub struct ProcessEnumerator<'a> {
    toolhelp_handle: &'a ToolHelp32Snapshot,
    current: PROCESSENTRY32,
}

impl<'a> ProcessEnumerator<'a> {
    pub fn new(snapshot_handle: &ToolHelp32Snapshot) -> Result<ProcessEnumerator, &'static str> {
        // Initialize memory for the first process entry
        // We rely on WinAPI to initialize all most members.
        let mut pe: PROCESSENTRY32;
        unsafe {
            // this will initialize <sizeof> PROCESSENTRY32 zeroed memory
            pe = mem::zeroed();
        }

        // Documentation states we have to manually set this member of the struct.
        pe.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;

        let res;

        unsafe {
            res = Process32First(snapshot_handle.handle, &mut pe);
        }

        if res == 0 {
            return Err("Call to Process32First failed");
        }

        Ok(ProcessEnumerator {
            toolhelp_handle: snapshot_handle,
            // moves res to be owned by the enumerator
            current: pe,
        })
    }
}

impl<'a> Iterator for ProcessEnumerator<'a> {
    type Item = ProcessInformation;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let res: i32;
        unsafe {
            res = Process32Next(self.toolhelp_handle.handle, &mut self.current);
        }

        if res == 1 {
            return Some(ProcessInformation::from(&self.current));
        }
        None
    }
}

#[derive(Debug)]
pub struct ProcessInformation {
    pub pid: u32,
    pub number_of_threads: u32,
    pub name: String,
}

impl<'a> From<&'a PROCESSENTRY32> for ProcessInformation {
    fn from(pe: &PROCESSENTRY32) -> Self {
        ProcessInformation {
            pid: pe.th32ProcessID,
            number_of_threads: pe.cntThreads,
            name: nul_terminated_i8_arr_to_string(&pe.szExeFile),
        }
    }
}

pub struct ToolHelp32Snapshot {
    pub handle: HANDLE,
}

impl ToolHelp32Snapshot {
    pub fn new() -> Result<ToolHelp32Snapshot, &'static str> {
        let handle: HANDLE;
        unsafe {
            handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        }
        if handle == INVALID_HANDLE_VALUE {
            Err("Could not create ToolHelp32Snapshot.")
        } else {
            Ok(ToolHelp32Snapshot { handle })
        }
    }
}

impl Drop for ToolHelp32Snapshot {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}
