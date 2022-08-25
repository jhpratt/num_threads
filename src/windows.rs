extern crate winapi;

use std::mem::size_of;
use std::num::NonZeroUsize;
use std::ptr::addr_of_mut;

use self::winapi::shared::minwindef::TRUE;
use self::winapi::um::handleapi::CloseHandle;
use self::winapi::um::processthreadsapi::GetCurrentProcessId;
use self::winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPALL,
};

pub(crate) fn num_threads() -> Option<NonZeroUsize> {
    unsafe {
        let pid = GetCurrentProcessId();
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPALL, 0);
        let mut entry = PROCESSENTRY32 {
            dwSize: size_of::<PROCESSENTRY32>() as u32,
            ..Default::default()
        };

        let mut ret = Process32First(snapshot, addr_of_mut!(entry));

        while ret == TRUE && entry.th32ProcessID != pid {
            ret = Process32Next(snapshot, addr_of_mut!(entry));
        }

        CloseHandle(snapshot);

        if ret == TRUE {
            NonZeroUsize::new(entry.cntThreads as usize)
        } else {
            None
        }
    }
}
