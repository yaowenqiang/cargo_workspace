use std::thread;
pub struct ThreadPool{
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// create a threadpool
    /// the threadpool size must gt 0
    /// # Panics
    ///

    pub fn new (size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {

        }
        ThreadPool {
            threads
        }
    }

    pub fn execute<F>(&self, f:F)  
        where F:FnOnce() + Send  + 'static {

    }
}
