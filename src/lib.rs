//! Minimum supported Rust version: 1.36

use std::num::NonZeroUsize;

#[cfg_attr(any(target_os = "linux", target_os = "android"), path = "linux.rs")]
#[cfg_attr(any(target_os = "macos", target_os = "ios"), path = "apple.rs")]
#[cfg_attr(target_os = "freebsd", path = "freebsd.rs")]
#[cfg_attr(target_os = "dragonfly", path = "dragonfly.rs")]
mod imp;

/// Obtain the number of threads currently part of the active process. Returns `None` if the number
/// of threads cannot be determined.
pub fn num_threads() -> Option<NonZeroUsize> {
    imp::num_threads()
}

/// Determine if the current process is single-threaded. Returns `None` if the number of threads
/// cannot be determined.
pub fn is_single_threaded() -> Option<bool> {
    num_threads().map(|n| n.get() == 1)
}

#[cfg(test)]
mod test {
    use std::num::NonZeroUsize;
    use std::thread::{sleep, spawn};
    use std::time::Duration;

    // Run each expression in its own thread.
    macro_rules! threaded {
        ($first:expr;) => {
            $first;
        };
        ($first:expr; $($rest:expr;)*) => {
            $first;
            spawn(|| {
                threaded!($($rest;)*);
            })
            .join()
            .unwrap();
        };
    }

    #[test]
    fn num_threads() {
        await_single_threaded();
        threaded! {
            assert_eq!(super::num_threads().map(NonZeroUsize::get), Some(1));
            assert_eq!(super::num_threads().map(NonZeroUsize::get), Some(2));
            assert_eq!(super::num_threads().map(NonZeroUsize::get), Some(3));
            assert_eq!(super::num_threads().map(NonZeroUsize::get), Some(4));
            assert_eq!(super::num_threads().map(NonZeroUsize::get), Some(5));
            assert_eq!(super::num_threads().map(NonZeroUsize::get), Some(6));
        }
    }

    #[test]
    fn is_single_threaded() {
        await_single_threaded();
        threaded! {
            assert_eq!(super::is_single_threaded(), Some(true));
            assert_eq!(super::is_single_threaded(), Some(false));
            assert_eq!(super::is_single_threaded(), Some(false));
            assert_eq!(super::is_single_threaded(), Some(false));
            assert_eq!(super::is_single_threaded(), Some(false));
            assert_eq!(super::is_single_threaded(), Some(false));
        }
    }

    // Wait a few moments for the thread count to reach one.
    //
    // DragonFly reaps LWP's async with thread exits, so it can take a few ms
    // to return to single-threaded mode after a test.
    // This also gives us a handy place to bail early on null implementations
    // and warn about the need for --test-threads=1
    fn await_single_threaded() {
        for _ in 0..50 {
            match super::is_single_threaded() {
                Some(true) => return,
                None => panic!("Thread counts unavailable in this environment"),
                _ => sleep(Duration::from_millis(1)),
            }
        }
        panic!("Not single threaded: did you run with `cargo test -- --test-threads=1`?");
    }
}
