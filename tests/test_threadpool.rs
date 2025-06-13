use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use heic2jpeg::ThreadPool;

#[test]
fn test_threadpool_creation() {
    let mut pool = ThreadPool::new(4);
}

#[test]
fn test_threadpool_execution() {
    let mut pool = ThreadPool::new(2);
    let counter = Arc::new(Mutex::new(0));
    
    // Spawn two tasks that increment the counter
    for _ in 0..2 {
        let counter = Arc::clone(&counter);
        pool.spawn(move || {
            thread::sleep(Duration::from_millis(100));
            let mut count = counter.lock().unwrap();
            *count += 1;
        });
    }
    
    // Wait for all tasks to complete
    pool.join();
    
    // Check if both tasks were executed
    let count = *counter.lock().unwrap();
    assert_eq!(count, 2);
}

#[test]
fn test_threadpool_parallel_execution() {
    let mut pool = ThreadPool::new(4);
    let results = Arc::new(Mutex::new(Vec::new()));
    
    // Spawn tasks that take different amounts of time
    for i in 0..4 {
        let results = Arc::clone(&results);
        pool.spawn(move || {
            thread::sleep(Duration::from_millis(100 * (4 - i)));
            let mut vec = results.lock().unwrap();
            vec.push(i);
        });
    }
    
    // Wait for all tasks to complete
    pool.join();
    
    // Check if all tasks were executed
    let vec = results.lock().unwrap();
    assert_eq!(vec.len(), 4);
}

#[test]
fn test_threadpool_termination() {
    let mut pool = ThreadPool::new(2);
    let counter = Arc::new(Mutex::new(0));
    
    // Spawn a task
    let counter_clone = Arc::clone(&counter);
    pool.spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let mut count = counter_clone.lock().unwrap();
        *count += 1;
    });
    
    // Join should wait for the task to complete
    pool.join();
    
    // Check if the task was executed
    let count = *counter.lock().unwrap();
    assert_eq!(count, 1);
}

#[test]
#[ignore]
fn test_threadpool_error_handling() {
    let mut pool = ThreadPool::new(2);
    let error_occurred = Arc::new(Mutex::new(false));
    
    // Spawn a task that panics
    let error_occurred_clone = Arc::clone(&error_occurred);
    pool.spawn(move || {
        let mut error = error_occurred_clone.lock().unwrap();
        *error = true;
        panic!("Test panic");
    });
    
    // Spawn a normal task
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = Arc::clone(&counter);
    pool.spawn(move || {
        let mut count = counter_clone.lock().unwrap();
        *count += 1;
    });
    
    // Join should not be affected by the panic
    pool.join();
    
    // Check if the error was recorded and the normal task completed
    let error = *error_occurred.lock().unwrap();
    let count = *counter.lock().unwrap();
    assert!(error);
    assert_eq!(count, 1);
} 