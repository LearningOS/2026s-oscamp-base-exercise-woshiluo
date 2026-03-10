//! # Mutex Shared State
//!
//! In this exercise, you will use `Arc<Mutex<T>>` to safely share and modify data between multiple threads.
//!
//! ## Concepts
//! - `Mutex<T>` mutex protects shared data
//! - `Arc<T>` atomic reference counting enables cross-thread sharing
//! - `lock()` acquires the lock and accesses data

use std::sync::{Arc, Mutex};
use std::thread;

/// Increment a counter concurrently using `n_threads` threads.
/// Each thread increments the counter `count_per_thread` times.
/// Returns the final counter value.
///
/// Hint: Use `Arc<Mutex<usize>>` as the shared counter.
pub fn concurrent_counter(n_threads: usize, count_per_thread: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let handles = (0..n_threads)
        .into_iter()
        .map(|_| {
            let counter = counter.clone();
            thread::spawn(move || {
                let counter = counter.clone();
                for _ in 0..count_per_thread {
                    let mut counter = counter.lock().unwrap();
                    *counter += 1;
                }
            })
        })
        .collect::<Vec<_>>();
    for handle in handles {
        handle.join().unwrap();
    }
    let result: usize = *counter.lock().unwrap();
    result
}

/// Add elements to a shared vector concurrently using multiple threads.
/// Each thread pushes its own id (0..n_threads) to the vector.
/// Returns the sorted vector.
///
/// Hint: Use `Arc<Mutex<Vec<usize>>>`.
pub fn concurrent_collect(n_threads: usize) -> Vec<usize> {
    // TODO: Create Arc<Mutex<Vec<usize>>>
    // TODO: Each thread pushes its own id
    // TODO: After joining all threads, sort the result and return
    let list = Arc::new(Mutex::new(Vec::new()));
    let handles = (0..n_threads)
        .into_iter()
        .map(|x| {
            let list = list.clone();
            thread::spawn(move || {
                let mut list = list.lock().unwrap();
                list.push(x)
            })
        })
        .collect::<Vec<_>>();
    for handle in handles {
        handle.join().unwrap();
    }
    let mut list = list.lock().unwrap().clone();
    list.sort();
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_single_thread() {
        assert_eq!(concurrent_counter(1, 100), 100);
    }

    #[test]
    fn test_counter_multi_thread() {
        assert_eq!(concurrent_counter(10, 100), 1000);
    }

    #[test]
    fn test_counter_zero() {
        assert_eq!(concurrent_counter(5, 0), 0);
    }

    #[test]
    fn test_collect() {
        let result = concurrent_collect(5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_collect_single() {
        assert_eq!(concurrent_collect(1), vec![0]);
    }
}
