extern crate libc;

use std::num::NonZeroUsize;
use std::{mem, ptr};

const KI_NGROUPS: usize = 16;
const KI_MAXCOMLEN: usize = 24;
const KI_WMESGLEN: usize = 8;
const KI_MAXLOGNAME: usize = 32;
const KI_EMULNAMELEN: usize = 8;

#[derive(Debug)]
#[repr(C)]
struct kinfo_proc {
    p_forw: u64, /* PTR: linked run/sleep queue. */
    p_back: u64,
    p_paddr: u64, /* PTR: address of proc */

    p_addr: u64,    /* PTR: Kernel virtual addr of u-area */
    p_fd: u64,      /* PTR: Ptr to open files structure. */
    p_stats: u64,   /* unused, always zero. */
    p_limit: u64,   /* PTR: Process limits. */
    p_vmspace: u64, /* PTR: Address space. */
    p_sigacts: u64, /* PTR: Signal actions, state */
    p_sess: u64,    /* PTR: session pointer */
    p_tsess: u64,   /* PTR: tty session pointer */
    p_ru: u64,      /* PTR: Exit information. XXX */

    p_eflag: i32, /* LONG: extra kinfo_proc flags */
    // #define	EPROC_CTTY	0x01	/* controlling tty vnode active */
    // #define	EPROC_SLEADER	0x02	/* session leader */
    // #define	EPROC_UNVEIL	0x04	/* has unveil settings */
    // #define	EPROC_LKUNVEIL	0x08	/* unveil is locked */
    p_exitsig: i32, /* unused, always zero. */
    p_flag: i32,    /* INT: P_* flags. */

    p_pid: i32,   /* PID_T: Process identifier. */
    p_ppid: i32,  /* PID_T: Parent process id */
    p_sid: i32,   /* PID_T: session id */
    p__pgid: i32, /* PID_T: process group id */
    /* XXX: <sys/proc.h> hijacks p_pgid */
    p_tpgid: i32, /* PID_T: tty process group id */

    p_uid: u32,  /* UID_T: effective user id */
    p_ruid: u32, /* UID_T: real user id */
    p_gid: u32,  /* GID_T: effective group id */
    p_rgid: u32, /* GID_T: real group id */

    p_groups: [u32; KI_NGROUPS], /* GID_T: groups */
    p_ngroups: i16,              /* SHORT: number of groups */

    p_jobc: i16, /* SHORT: job control counter */
    p_tdev: u32, /* DEV_T: controlling tty dev */

    p_estcpu: u32,     /* U_INT: Time averaged value of p_cpticks. */
    p_rtime_sec: u32,  /* STRUCT TIMEVAL: Real time. */
    p_rtime_usec: u32, /* STRUCT TIMEVAL: Real time. */
    p_cpticks: i32,    /* INT: Ticks of cpu time. */
    p_pctcpu: u32,     /* FIXPT_T: %cpu for this process */
    p_swtime: u32,     /* unused, always zero */
    p_slptime: u32,    /* U_INT: Time since last blocked. */
    p_schedflags: i32, /* INT: PSCHED_* flags */

    p_uticks: u64, /* U_QUAD_T: Statclock hits in user mode. */
    p_sticks: u64, /* U_QUAD_T: Statclock hits in system mode. */
    p_iticks: u64, /* U_QUAD_T: Statclock hits processing intr. */

    p_tracep: u64,    /* PTR: Trace to vnode or file */
    p_traceflag: i32, /* INT: Kernel trace points. */

    p_holdcnt: i32, /* INT: If non-zero, don't swap. */

    p_siglist: i32,   /* INT: Signals arrived but not delivered. */
    p_sigmask: u32,   /* SIGSET_T: Current signal mask. */
    p_sigignore: u32, /* SIGSET_T: Signals being ignored. */
    p_sigcatch: u32,  /* SIGSET_T: Signals being caught by user. */

    p_stat: i8,     /* CHAR: S* process status (from LWP). */
    p_priority: u8, /* U_CHAR: Process priority. */
    p_usrpri: u8,   /* U_CHAR: User-priority based on p_estcpu and ps_nice. */
    p_nice: u8,     /* U_CHAR: Process "nice" value. */

    p_xstat: u16, /* U_SHORT: Exit status for wait; also stop signal. */
    p_spare: u16, /* U_SHORT: unused */

    p_comm: [libc::c_char; KI_MAXCOMLEN],

    p_wmesg: [libc::c_char; KI_WMESGLEN], /* wchan message */
    p_wchan: u64,                         /* PTR: sleep address. */

    p_login: [libc::c_char; KI_MAXLOGNAME], /* setlogin() name */

    p_vm_rssize: i32, /* SEGSZ_T: current resident set size in pages */
    p_vm_tsize: i32,  /* SEGSZ_T: text size (pages) */
    p_vm_dsize: i32,  /* SEGSZ_T: data size (pages) */
    p_vm_ssize: i32,  /* SEGSZ_T: stack size (pages) */

    ip_uvalid: i64, /* CHAR: following p_u* members from struct user are valid */
    /* XXX 64 bits for alignment */
    p_ustart_sec: u64,  /* STRUCT TIMEVAL: starting time. */
    p_ustart_usec: u32, /* STRUCT TIMEVAL: starting time. */

    p_uutime_sec: u32,  /* STRUCT TIMEVAL: user time. */
    p_uutime_usec: u32, /* STRUCT TIMEVAL: user time. */
    p_ustime_sec: u32,  /* STRUCT TIMEVAL: system time. */
    p_ustime_usec: u32, /* STRUCT TIMEVAL: system time. */

    p_uru_maxrss: u64,   /* LONG: max resident set size. */
    p_uru_ixrss: u64,    /* LONG: integral shared memory size. */
    p_uru_idrss: u64,    /* LONG: integral unshared data ". */
    p_uru_isrss: u64,    /* LONG: integral unshared stack ". */
    p_uru_minflt: u64,   /* LONG: page reclaims. */
    p_uru_majflt: u64,   /* LONG: page faults. */
    p_uru_nswap: u64,    /* LONG: swaps. */
    p_uru_inblock: u64,  /* LONG: block input operations. */
    p_uru_oublock: u64,  /* LONG: block output operations. */
    p_uru_msgsnd: u64,   /* LONG: messages sent. */
    p_uru_msgrcv: u64,   /* LONG: messages received. */
    p_uru_nsignals: u64, /* LONG: signals received. */
    p_uru_nvcsw: u64,    /* LONG: voluntary context switches. */
    p_uru_nivcsw: u64,   /* LONG: involuntary ". */

    p_uctime_sec: u32,                      /* STRUCT TIMEVAL: child u+s time. */
    p_uctime_usec: u32,                     /* STRUCT TIMEVAL: child u+s time. */
    p_psflags: u32,                         /* UINT: PS_* flags on the process. */
    p_acflag: u32,                          /* UINT: Accounting flags. */
    p_svuid: u32,                           /* UID_T: saved user id */
    p_svgid: u32,                           /* GID_T: saved group id */
    p_emul: [libc::c_char; KI_EMULNAMELEN], /* syscall emulation name */
    p_rlim_rss_cur: u64,                    /* RLIM_T: soft limit for rss */
    p_cpuid: u64,                           /* LONG: CPU id */
    p_vm_map_size: u64,                     /* VSIZE_T: virtual size */
    p_tid: i32,                             /* PID_T: Thread identifier. */
    p_rtableid: u32,                        /* U_INT: Routing table identifier. */

    p_pledge: u64, /* U_INT64_T: Pledge flags. */
}

pub(crate) fn num_threads() -> Option<NonZeroUsize> {
    // Safety: `sysctl` and `getpid` are both thread-safe.
    // `kip` is only accessed if sysctl() succeeds and agrees with the expected size,
    // and the data only trusted if both its embedded size and pid match expectations
    unsafe {
        // OpenBSD differs from the other BSDs in not having a dedicated thread/lwp field
        // We must call sysctl() twice, once to get the size of the buffer we'll need
        // (which may over-estimate), and once to actually retrieve the list
        let pid = libc::getpid();
        let expected_kip_len = mem::size_of::<kinfo_proc>();

        let mut mib: [libc::c_int; 6] = [
            libc::CTL_KERN,
            libc::KERN_PROC,
            libc::KERN_PROC_PID | libc::KERN_PROC_SHOW_THREADS,
            pid,
            expected_kip_len as libc::c_int,
            0,
        ];
        let mut kip_len = 0;

        let ret = libc::sysctl(
            mib.as_ptr(),
            mib.len() as u32,
            ptr::null_mut() as *mut _ as *mut libc::c_void,
            &mut kip_len,
            ptr::null_mut() as *mut _ as *mut libc::c_void,
            0,
        );

        if ret < 0 || kip_len % expected_kip_len != 0 {
            return None;
        }

        let nitems = kip_len / expected_kip_len;
        let mut kips: Vec<kinfo_proc> = Vec::with_capacity(nitems);
        kip_len = nitems * expected_kip_len;
        mib[5] = nitems as libc::c_int;

        let ret = libc::sysctl(
            mib.as_ptr(),
            mib.len() as u32,
            // Vec::as_mut_ptr added in 1.37.0
            kips.as_mut_ptr() as *mut _ as *mut libc::c_void,
            &mut kip_len,
            ptr::null_mut() as *mut _ as *mut libc::c_void,
            0,
        );

        if ret == 0 && kip_len % expected_kip_len == 0 {
            let nitems = kip_len / expected_kip_len;
            kips.set_len(nitems);
            NonZeroUsize::new(
                kips.into_iter()
                    .filter(|kip| kip.p_pid == pid && kip.p_tid != -1)
                    .count(),
            )
        } else {
            dbg!(ret, kip_len);
            None
        }
    }
}
