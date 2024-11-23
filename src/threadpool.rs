use std::thread;

use crossbeam::channel::{self,Receiver,Sender};
use log::{debug,error};


type Job = Box<dyn FnOnce() + Send + 'static>;


pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {

        // Open channel
        let (sender,receiver) = channel::unbounded::<Job>();

        // Build the set of threads
        let mut threads = Vec::with_capacity(size);
        for id in 0..size {
            threads.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool { threads, sender }
    }

    pub fn spawn<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).expect("The thread pool has no thread");
    }

    pub fn join(&self) {
        loop {
            match self.sender.is_empty() {
                true => {
                    break;
                },
                false => {},
            };
        }
    }
}



struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Receiver<Job>) -> Self {
        debug!("Starting thread worker {}", id);
        let thread = thread::spawn(move || {
            loop {
                match receiver.recv() {
                    Ok(job) => {
                        job();
                    },
                    Err(_) => {
                        error!("Thread fails because the pool is destroyed");
                    }
                };
            }
        });

        Worker { id, thread }
    }
}
