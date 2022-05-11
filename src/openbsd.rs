extern crate libc;

use std::num::NonZeroUsize;
use std::{mem, ptr};

use self::libc::{c_int, c_void, kinfo_proc, sysctl};

pub(crate) fn num_threads() -> Option<NonZeroUsize> {
    // Safety: `sysctl` and `getpid` are both thread-safe.
    // Struct sizes are checked on both our side and kernel side
    unsafe {
        // OpenBSD differs from the other BSDs in not having a dedicated thread/lwp field,
        // instead we have to enumerate individual threads.
        //
        // We must call sysctl() twice, once to get the size of the buffer we'll need
        // and once to actually retrieve the list.

        let pid = libc::getpid();
        let expected_kip_len = mem::size_of::<kinfo_proc>();

        // Unlike The other BSDs, OpenBSD has us pass in the size of the struct
        // we expect and how many items we believe fits in our buffer.
        //
        // This should make it quite robust in face of ABI changes - new fields
        // should be appended to the existing structure, with older software
        // getting a truncated version matching what they expect.
        let mut mib: [c_int; 6] = [
            libc::CTL_KERN,
            libc::KERN_PROC,
            libc::KERN_PROC_PID | libc::KERN_PROC_SHOW_THREADS,
            pid,
            expected_kip_len as c_int,
            0,
        ];
        let mut kip_len = 0;

        let ret = sysctl(
            mib.as_ptr(),
            mib.len() as u32,
            ptr::null_mut() as *mut _ as *mut c_void,
            &mut kip_len,
            ptr::null_mut() as *mut _ as *mut c_void,
            0,
        );

        if ret < 0 {
            return None;
        }

        // libkvm's kvm_getprocs() rounds up its buffer by 1/8th.
        // Instead we round up the number of items.
        let mut nitems = kip_len / expected_kip_len;
        nitems += nitems / 8;
        mib[5] = nitems as c_int;
        kip_len = expected_kip_len * nitems;

        let mut kips: Vec<kinfo_proc> = Vec::with_capacity(nitems);

        let ret = sysctl(
            mib.as_ptr(),
            mib.len() as u32,
            kips.as_mut_ptr() as *mut _ as *mut c_void,
            &mut kip_len,
            ptr::null_mut() as *mut _ as *mut c_void,
            0,
        );

        if ret < 0 {
            return None;
        }

        // A final sanity check before we blindly accept what we've been given.
        // Failure here would indicate a serious ABI breakage.
        assert!(kip_len % expected_kip_len == 0);
        assert!(kip_len <= expected_kip_len * nitems);
        kips.set_len(kip_len / expected_kip_len);

        // In principle we could instead just return kip_len / expected_kip_len - 1
        // as we have a list of threads plus a p_tid == -1 entry for the process.
        NonZeroUsize::new(
            kips.into_iter()
                .filter(|kip| kip.p_pid == pid && kip.p_tid != -1)
                .count(),
        )
    }
}
