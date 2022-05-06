extern crate memoffset;
extern crate winapi;

use std::mem;
use std::num::NonZeroUsize;

use self::winapi::shared::minwindef::{FALSE, TRUE};
use self::winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use self::winapi::um::processthreadsapi::GetCurrentProcessId;
use self::winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
};

pub(crate) fn num_threads() -> Option<NonZeroUsize> {
    unsafe {
        // This approach is described in Raymond Chen's Old New Thing blog:
        // https://devblogs.microsoft.com/oldnewthing/20060223-14/?p=32173
        // Also documented here, without the dwSize check:
        // https://docs.microsoft.com/en-us/windows/win32/toolhelp/traversing-the-thread-list
        let handle = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);

        if handle == INVALID_HANDLE_VALUE {
            return None;
        }

        let pid = GetCurrentProcessId();
        let mut count = 0;

        // THREADENTRY32 consists entirely of DWORD and LONG values
        // (This is exactly what winapi impl-default does)
        let mut te: THREADENTRY32 = mem::zeroed();
        te.dwSize = mem::size_of_val(&te) as u32;
        if Thread32First(handle, &mut te) == TRUE {
            loop {
                // Ensure the retrieved data encompasses the field we want
                if te.dwSize
                    >= (memoffset::offset_of!(THREADENTRY32, th32OwnerProcessID)
                        + mem::size_of_val(&te.th32OwnerProcessID)) as u32
                    && te.th32OwnerProcessID == pid
                {
                    count += 1;
                }
                te.dwSize = mem::size_of_val(&te) as u32;
                if Thread32Next(handle, &mut te) == FALSE {
                    break;
                }
            }
        }
        CloseHandle(handle);
        NonZeroUsize::new(count)
    }
}
