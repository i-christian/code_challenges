use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Creates a new Worker.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the worker.
    /// * `receiver` - The receiver end of a channel to get jobs.
    ///
    /// # Returns
    ///
    /// A new Worker instance.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

#[derive(Debug)]
pub enum PoolError {
    PoolCreationError,
}

use PoolError::PoolCreationError;

impl ThreadPool {
    /// Creates a new ThreadPool.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of workers in the pool.
    ///
    /// # Returns
    ///
    /// A Result containing either the ThreadPool or a PoolError.
    pub fn new(size: usize) -> Result<ThreadPool, PoolError> {
        if size < 1 {
            return Err(PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    /// Executes a function using one of the workers in the pool.
    ///
    /// # Arguments
    ///
    /// * `f` - The function to be executed.
    ///
    /// The function must implement the `FnOnce` trait, be `Send`, and have a static lifetime.
    pub fn execute<F>(&self, f: F)
    where
        F: Send + FnOnce() + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    /// Drops the ThreadPool, gracefully shutting down all workers.
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
