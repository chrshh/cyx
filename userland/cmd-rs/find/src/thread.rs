use std::{
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle, available_parallelism},
};

use crate::error::CFindError;

#[derive(Debug)]
pub struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl std::ops::Deref for Worker {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Option<mpsc::Sender<Job>>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let msg = rx.lock().unwrap().recv();
                match msg {
                    Ok(job) => {
                        // println!("Worker {id} got a job... executing");
                        job()
                    }
                    Err(_) => {
                        // println!("Worker {id} disconnected... shutting down");
                        break;
                    }
                }
            }
        });
        Worker { id, thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new() -> Result<Self, CFindError> {
        let thread_count: usize = available_parallelism().map(|n| n.get()).unwrap_or(1);
        if thread_count == 0 {
            return Err(CFindError::PoolInitError);
        }

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(thread_count);

        for id in 0..thread_count {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        Ok(Self {
            workers,
            tx: Some(tx),
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.tx.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.tx.take());
        for worker in self.workers.drain(..) {
            // println!("Shutting down worker #{}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}
