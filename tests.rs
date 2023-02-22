extern crate num_threads;

use std::num::NonZeroUsize;

use num_threads::*;

// Run each expression in its own thread.
macro_rules! threaded {
    ($first:expr;) => {
        $first;
    };
    ($first:expr; $($rest:expr;)*) => {
        $first;
        ::std::thread::spawn(|| {
            threaded!($($rest;)*);
        })
        .join()
        .unwrap();
    };
}

fn test_single_threaded_by_default() {
    if is_single_threaded() != Some(true) {
        panic!("Process is not single-threaded, but it should be.");
    }
}

fn test_num_threads() {
    threaded! {
        assert_eq!(num_threads().map(NonZeroUsize::get), Some(1));
        assert_eq!(num_threads().map(NonZeroUsize::get), Some(2));
        assert_eq!(num_threads().map(NonZeroUsize::get), Some(3));
        assert_eq!(num_threads().map(NonZeroUsize::get), Some(4));
        assert_eq!(num_threads().map(NonZeroUsize::get), Some(5));
        assert_eq!(num_threads().map(NonZeroUsize::get), Some(6));
    }
}

fn test_is_single_threaded() {
    threaded! {
        assert_eq!(is_single_threaded(), Some(true));
        assert_eq!(is_single_threaded(), Some(false));
        assert_eq!(is_single_threaded(), Some(false));
        assert_eq!(is_single_threaded(), Some(false));
        assert_eq!(is_single_threaded(), Some(false));
        assert_eq!(is_single_threaded(), Some(false));
    }
}

fn main() {
    test_single_threaded_by_default();
    std::thread::sleep(std::time::Duration::new(0, 100));
    test_num_threads();
    std::thread::sleep(std::time::Duration::new(0, 100));
    test_is_single_threaded();
    eprintln!("All tests passed!");
}
