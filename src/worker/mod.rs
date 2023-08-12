use std::{thread, time::Duration};

pub struct Worker<F>
where
    F: Fn() + Send + Clone + 'static,
{
    task: F,
    interval: u64,
}

impl<F> Worker<F>
where
    F: Fn() + Send + Clone + 'static,
{
    pub fn new(interval: u64, task: F) -> Self {
        Worker { task, interval }
    }

    pub fn start(&self) -> thread::JoinHandle<()> {
        let task = self.task.clone();
        let interval = self.interval;

        thread::spawn(move || loop {
            task();
            thread::sleep(Duration::from_millis(interval));
        })
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    use super::*;

    #[test]
    fn test_worker() {
        let x = Arc::new(Mutex::new(0));
        let x_clone = Arc::clone(&x);
        let worker = Worker::new(100, move || {
            let mut x = x_clone.lock().unwrap();
            *x += 1;
        });
        let worker_thread = worker.start();
        thread::sleep(Duration::from_millis(500));
        drop(worker_thread);
        assert_eq!(*x.lock().unwrap(), 5);
    }
}
