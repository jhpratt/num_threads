extern crate libc;

use std::num::NonZeroUsize;
use std::{io, mem, ptr};

use self::libc::{c_int, c_void, kinfo_lwp, sysctl};

pub(crate) fn num_threads() -> Option<NonZeroUsize> {
    // Safety: `sysctl` and `getpid` are both thread-safe.
    // `kip` is only accessed if sysctl() succeeds and sizes match
    unsafe {
        // While kinfo_proc2 does contain a LWP count field, we must enumerate
        // as defunct threads may leave zombie/idle LWP's in their wake.
        let pid = libc::getpid();
        let expected_size = mem::size_of::<kinfo_lwp>();
        let mut size = 0;
        let mut mib: [c_int; 5] = [
            libc::CTL_KERN,
            libc::KERN_LWP,
            pid,
            expected_size as c_int,
            0,
        ];

        loop {
            let ret = libc::sysctl(
                mib.as_ptr(),
                mib.len() as u32,
                ptr::null_mut(),
                &mut size,
                ptr::null(),
                0,
            );

            if ret != 0 {
                return None;
            }

            let mut lwps: Vec<kinfo_lwp> = Vec::with_capacity(size / expected_size as usize);
            mib[4] = lwps.capacity() as c_int;
            size = lwps.capacity() * expected_size;

            let ret = sysctl(
                mib.as_ptr(),
                mib.len() as u32,
                lwps.as_mut_ptr() as *mut _ as *mut c_void,
                &mut size,
                ptr::null_mut() as *mut _ as *mut c_void,
                0,
            );

            if ret != 0 {
                // Thread count changed between calls, retry
                if io::Error::last_os_error().kind() == io::ErrorKind::OutOfMemory {
                    continue;
                }
                return None;
            }

            lwps.set_len(size / expected_size);

            let nthreads = lwps
                .into_iter()
                .filter(|lwp| {
                    lwp.l_pid as i32 == pid
                        && lwp.l_stat as i32 != libc::LSIDL
                        && lwp.l_stat as i32 != libc::LSZOMB
                })
                .count();

            return NonZeroUsize::new(nthreads);
        }
    }
}
