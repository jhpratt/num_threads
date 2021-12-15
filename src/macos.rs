extern crate libc;

use std::mem;
use std::num::NonZeroUsize;

pub(crate) fn num_threads() -> Option<NonZeroUsize> {
    unsafe {
        let size = mem::size_of::<libc::proc_taskinfo>();
        let buffer = libc::malloc(size);

        let c_size = size as libc::c_int;
        let result = libc::proc_pidinfo(libc::getpid(), libc::PROC_PIDTASKINFO, 0, buffer, c_size);
        if result != c_size {
            return None;
        }

        let pti = buffer as *mut libc::proc_taskinfo;
        // Safety: `malloc`ed memory is aligned for repr(C) structs, so dereference is safe.
        let num_threads = NonZeroUsize::new(pti.as_ref()?.pti_threadnum as usize);

        libc::free(pti as *mut libc::c_void);
        num_threads
    }
}
