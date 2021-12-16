extern crate libc;

use std::mem;
use std::num::NonZeroUsize;

const PROC_TASKINFO_SIZE: usize = mem::size_of::<libc::proc_taskinfo>();

pub(crate) fn num_threads() -> Option<NonZeroUsize> {
    let buffer = unsafe { libc::malloc(PROC_TASKINFO_SIZE) };
    if buffer.is_null() {
        return None;
    }

    let result = unsafe {
        libc::proc_pidinfo(
            libc::getpid(),
            libc::PROC_PIDTASKINFO,
            0,
            buffer,
            PROC_TASKINFO_SIZE as libc::c_int,
        )
    };
    if result != PROC_TASKINFO_SIZE as libc::c_int {
        return None;
    }

    let pti = buffer as *mut libc::proc_taskinfo;
    // Safety: `malloc`ed memory is aligned for repr(C) structs, so dereference is safe.
    let num_threads = NonZeroUsize::new(unsafe { pti.as_ref() }?.pti_threadnum as usize);

    unsafe { libc::free(pti as *mut libc::c_void) };
    num_threads
}
