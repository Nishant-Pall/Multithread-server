use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,

    // sending jobs across threads
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    ///	The size is the number of threads in the pool
    ///
    ///	# Panics
    ///
    ///	The new function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        // assert pool size to be greater than 0
        assert!(size > 0);

        // create a new channel
        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        // To resolve the issue of passing ownership during loop iteration
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            // create threads
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // get job from receiver
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job executing.", id);

            job();
        });

        Worker { id, thread }
    }
}
