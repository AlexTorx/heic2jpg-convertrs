use std::sync::{Arc,Mutex,mpsc};
use std::thread;

use log::info;


type Job = Box<dyn FnOnce() + Send + 'static>;


pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {

        // Open channel
        let (sender,receiver) = mpsc::channel();

        // Prepare receiver to be thread-safe
        let receiver = Arc::new(Mutex::new(receiver));

        // Build the set of threads
        let mut threads = Vec::with_capacity(size);
        for id in 0..size {
            threads.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { threads, sender }
    }

    pub fn spawn<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).expect("The thread pool has no thread");
    }
}



struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                match receiver.lock().unwrap().recv() {
                    Ok(job) => {
                        job();
                    },
                    Err(_) => {
                        info!("Thread fails because the pool is destroyed");
                    }
                };
            }
        });

        Worker { id, thread }
    }
}
