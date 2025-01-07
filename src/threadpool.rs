use std::thread;
use std::process::exit;

use crossbeam::channel::{self,Receiver,Sender};
use log::{debug,error};


type Job = Box<dyn FnOnce() + Send + 'static>;


enum WorkerMessage {
    Terminate,
    Task(Job),
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<WorkerMessage>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {

        // Open channel
        let (sender,receiver) = channel::unbounded::<WorkerMessage>();

        // Build the set of threads
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool { workers, sender }
    }

    pub fn spawn<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(WorkerMessage::Task(job)).expect("The thread pool has no thread");
    }

    pub fn join(&mut self) {
        // Send termination message to all workers
        for _ in &mut self.workers {
            self.sender.send(WorkerMessage::Terminate).unwrap();
        }

        debug!("All workers are being shutdown");

        // Wait until all workers join
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                debug!("Shutting down worker {}", worker.id);
                thread.join().unwrap();
            }
        }
    }
}



struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Receiver<WorkerMessage>) -> Self {
        debug!("Starting thread worker {}", id);
        let thread = thread::spawn(move || {
            loop {
                match receiver.recv() {
                    Ok(msg) => {
                        match msg {
                            WorkerMessage::Terminate => {
                                debug!("Terminating thread worker {} as end request was received", id);
                                exit(0);
                            },
                            WorkerMessage::Task(job) => {
                                debug!("Worker {} starts running a new job", id);
                                job();
                            }
                        }
                    },
                    Err(_) => {
                        error!("Thread fails because the pool is destroyed");
                    }
                };
            }
        });

        Worker { id, thread: Some(thread) }
    }
}
