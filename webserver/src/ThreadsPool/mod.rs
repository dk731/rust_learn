use flume;
use std::thread;

use std::sync::{Arc, Condvar, Mutex};
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Pool {
    workers: Vec<Worker>,
    jobs: Arc<Mutex<Vec<Job>>>,
    jobs_notif: Arc<Condvar>,
    jobs_finished: Arc<Mutex<bool>>,
    pool_size: usize, // Amount of parallel threads
}

struct Worker {
    thread_handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(
        available_jobs: Arc<Mutex<Vec<Job>>>,
        job_notify: Arc<Condvar>,
        job_finished: Arc<Mutex<bool>>,
    ) -> Self {
        let thread_handle = thread::spawn(move || {
            println!("Staring worker thread");

            loop {
                let mut jobs = available_jobs.lock().unwrap();
                while *job_finished.lock().unwrap() && jobs.len() == 0 {
                    jobs = job_notify.wait(jobs).unwrap();
                }

                jobs.pop().unwrap()();
            }
        });

        Worker {
            thread_handle: thread_handle,
        }
    }
}

impl Pool {
    pub fn new(pool_size: usize) -> Self {
        assert!(pool_size > 0);

        let (finish_send, finish_recv): (flume::Sender<i32>, flume::Receiver<i32>) =
            flume::unbounded();
        let jobs = Arc::new(Mutex::new(Vec::new()));
        let jobs_notif = Arc::new(Condvar::new());
        let jobs_finished = Arc::new(Mutex::new(false));

        Pool {
            workers: (0..pool_size)
                .map(|_| Worker::new(jobs.clone(), jobs_notif.clone(), jobs_finished.clone()))
                .collect(),
            jobs: jobs,
            jobs_notif,
            pool_size,
            jobs_finished,
        }
    }

    pub fn excecute<F>(&mut self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.jobs.lock().unwrap().push(Box::new(job));
        self.jobs_notif.notify_one();
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        *self.jobs_finished.lock().unwrap() = false;
        self.jobs_notif.notify_all();
    }
}
