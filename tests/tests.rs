
extern crate num_threads;

use std::num::NonZeroUsize;

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

fn test_num_threads() {
    threaded! {
        assert_eq!(num_threads::num_threads().map(NonZeroUsize::get), Some(1));
        assert_eq!(num_threads::num_threads().map(NonZeroUsize::get), Some(2));
        assert_eq!(num_threads::num_threads().map(NonZeroUsize::get), Some(3));
        assert_eq!(num_threads::num_threads().map(NonZeroUsize::get), Some(4));
        assert_eq!(num_threads::num_threads().map(NonZeroUsize::get), Some(5));
        assert_eq!(num_threads::num_threads().map(NonZeroUsize::get), Some(6));
    }
}

fn test_is_single_threaded() {
    threaded! {
        assert_eq!(num_threads::is_single_threaded(), Some(true));
        assert_eq!(num_threads::is_single_threaded(), Some(false));
        assert_eq!(num_threads::is_single_threaded(), Some(false));
        assert_eq!(num_threads::is_single_threaded(), Some(false));
        assert_eq!(num_threads::is_single_threaded(), Some(false));
        assert_eq!(num_threads::is_single_threaded(), Some(false));
    }
}

fn main() {
    test_is_single_threaded();
    test_num_threads();
}

