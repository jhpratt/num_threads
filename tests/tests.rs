extern crate num_threads;

use std::io::prelude::*;
use std::num::NonZeroUsize;

/// Run a closure with n threads running in the background
fn with_threads<F>(n: usize, f: F)
where
    F: FnOnce(),
{
    let mut threads = vec![];
    let lock = ::std::sync::Arc::new(::std::sync::Mutex::new(()));
    let _guard = lock.lock().unwrap();

    for _ in 1..n {
        let lock = lock.clone();
        threads.push(::std::thread::spawn(move || {
            drop(lock.lock().unwrap());
        }));
    }

    f();

    drop(_guard);

    for handle in threads {
        handle.join().unwrap();
    }
}

fn test_num_threads() {
    print!("test num_threads: ");
    ::std::io::stdout().flush().unwrap();

    with_threads(1, || {
        assert_eq!(num_threads::num_threads().map(NonZeroUsize::get), Some(1))
    });
    with_threads(2, || {
        assert_eq!(num_threads::num_threads().map(NonZeroUsize::get), Some(2))
    });
    with_threads(3, || {
        assert_eq!(num_threads::num_threads().map(NonZeroUsize::get), Some(3))
    });
    with_threads(4, || {
        assert_eq!(num_threads::num_threads().map(NonZeroUsize::get), Some(4))
    });
    with_threads(5, || {
        assert_eq!(num_threads::num_threads().map(NonZeroUsize::get), Some(5))
    });
    with_threads(6, || {
        assert_eq!(num_threads::num_threads().map(NonZeroUsize::get), Some(6))
    });

    println!("ok");
}

fn test_is_single_threaded() {
    print!("test is_single_threaded: ");
    ::std::io::stdout().flush().unwrap();

    with_threads(1, || {
        assert_eq!(num_threads::is_single_threaded(), Some(true))
    });
    with_threads(2, || {
        assert_eq!(num_threads::is_single_threaded(), Some(false))
    });
    with_threads(3, || {
        assert_eq!(num_threads::is_single_threaded(), Some(false))
    });
    with_threads(4, || {
        assert_eq!(num_threads::is_single_threaded(), Some(false))
    });
    with_threads(5, || {
        assert_eq!(num_threads::is_single_threaded(), Some(false))
    });
    with_threads(6, || {
        assert_eq!(num_threads::is_single_threaded(), Some(false))
    });

    println!("ok");
}

fn main() {
    test_is_single_threaded();
    test_num_threads();
}
