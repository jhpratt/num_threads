use std::fs;
use std::num::NonZeroUsize;

pub(crate) fn num_threads() -> Option<NonZeroUsize> {
    fs::read_dir("/proc/self/task")
        // If we can't read the directory, return `None`.
        .ok()
        // The number of files in the directory is the number of threads.
        .and_then(|tasks| NonZeroUsize::new(tasks.count()))
}
