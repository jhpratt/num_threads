extern crate libc;

use std::num::NonZeroUsize;

extern "C" {
    fn kinfo_getproc(pid: libc::pid_t) -> *mut libc::kinfo_proc;
}

pub(crate) fn num_threads() -> Option<NonZeroUsize> {
    // Safety: `kinfo_getproc` and `getpid` are both thread-safe. All invariants of `as_ref` are
    // upheld.
    NonZeroUsize::new(unsafe { kinfo_getproc(libc::getpid()).as_ref() }?.ki_numthreads as usize)
}
