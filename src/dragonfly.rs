extern crate libc;

use std::num::NonZeroUsize;
use std::{mem, ptr};

pub(crate) fn num_threads() -> Option<NonZeroUsize> {
    // Safety: `sysctl` and `getpid` are both thread-safe.
    // `kip` is only accessed if sysctl() succeeds and agrees with the expected size,
    // and the data only trusted if pid matches expectations
    unsafe {
        let pid = libc::getpid();
        let mib: [libc::c_int; 4] = [libc::CTL_KERN, libc::KERN_PROC, libc::KERN_PROC_PID, pid];
        let mut kip = mem::MaybeUninit::<libc::kinfo_proc>::uninit();
        let expected_kip_len = mem::size_of_val(&kip);
        let mut kip_len = expected_kip_len;

        let ret = libc::sysctl(
            mib.as_ptr(),
            mib.len() as u32,
            kip.as_mut_ptr() as *mut libc::c_void,
            &mut kip_len,
            ptr::null(),
            0,
        );

        if ret != 0 || kip_len != expected_kip_len {
            None
        } else {
            let kip = kip.assume_init();
            if kip.kp_pid == pid {
                NonZeroUsize::new(kip.kp_nthreads as usize)
            } else {
                None
            }
        }
    }
}
