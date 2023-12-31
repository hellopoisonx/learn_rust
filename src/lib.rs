use std::thread;
pub struct ThreadPool {
    workers: Vec<Worker>,
}
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Self {
        let thread = thread::spawn(|| {});
        Self { id, thread }
    }
}
impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i))
        }
        Self { workers }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        f();
    }
}
