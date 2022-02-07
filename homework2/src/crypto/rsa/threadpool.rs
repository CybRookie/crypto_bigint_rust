use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

// A pool of threads, stored in a vector of workers,
// thread pool also provides a sender part of the channel,
// so the main thread can send new task to the threads or
// request the graceful termination.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<ThreadTask>,
}

// Job type containing a clojure with the task.
type Job = Box<dyn FnOnce() + Send + 'static>;

// The worker struct containing its ID and a job function.
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

// An enumeration of commands for the RSA bruteforcing threads,
// there are two options: a new job with a function for execution
// and a termination signal for graceful shutdown.
enum ThreadTask {
    NewJob(Job),
    Terminate,
}

// Implement methods on the thread pool struct.
impl ThreadPool {
    // Create/construct a new thread pool, containing the requested amount of workers/threads,
    // and a sender part of the channel for task/signal reception from the main thread.
    pub fn new(size: usize) -> ThreadPool {
        // Assert that the amount of threads requested is more than zero.
        assert!(size > 0);

        // Create a channel, share the receiver with smart reference and a mutex among
        // workers/threads, while the sender part will be utilised by the main thread
        // to send new signals/task to the workers.
        let (main_sender, worker_receiver) = mpsc::channel();
        let worker_receiver = Arc::new(Mutex::new(worker_receiver));

        // Define a new vector with workers and fiil with the amount requested.
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&worker_receiver)));
        }

        ThreadPool {
            workers,
            sender: main_sender,
        }
    }

    // Receive a new requested task and send it to the shared receiver part of the channel.
    pub fn execute<F>(&self, thread_task_function: F)
        where
            F: FnOnce() + Send + 'static,
    {
        // Wrap the received closure, store it on the heap.
        let job = Box::new(thread_task_function);

        // Wrap the box with the clojure into the custom enumeration and
        // send the task across the channel to the shared receiver among the threads.
        // The free thread will pick up the task and execute it.
        self.sender.send(ThreadTask::NewJob(job)).unwrap();
    }
}

// Implement the Drop thread for the thread pool struct,
// defining how the pool will destruct itself, when it goes out the scope.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate ThreadTask to all workers.");

        // Send the termination signal to all the workers in the thread pool.
        for _ in &self.workers {
            self.sender.send(ThreadTask::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        // Wait for the closure of workers/threads.
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // If the thread still exits, wait for its exit.
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap_or(());
            }
        }
    }
}

// Implement methods on the worker struct.
impl Worker {
    // Create/construct a new worker, receiving its own ID and a smart reference to the
    // mutex locking the common receiver part of the channel among the threads.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<ThreadTask>>>) -> Worker {
        // Create the thread worker with a closure,
        // constantly listening for the new task,
        // when a new task is received, act accordingly upon it.
        let thread = thread::spawn(move || loop {
            let thread_task = receiver.lock().unwrap().recv().unwrap();
            match thread_task {
                ThreadTask::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                ThreadTask::Terminate => {
                    println!("Worker {} was told to terminate.", id);
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

#[cfg(test)]
mod tests {
    use std::sync::{Arc, mpsc, Mutex};

    use crate::crypto::rsa::threadpool::{ThreadPool, ThreadTask, Worker};

    // Test the thread pool construction and destruction.
    #[test]
    fn test_rsa_thread_pool_construction_and_destruction() {
        let amount_of_threads = 4;
        let thread_pool = ThreadPool::new(amount_of_threads);

        assert_eq!(thread_pool.workers.len(), amount_of_threads);
    }

    // Test the thread pool task execution.
    #[test]
    fn test_rsa_thread_pool_task_execution() {
        let amount_of_threads = 4;
        let thread_pool = ThreadPool::new(amount_of_threads);

        // Request execution of 16 tasks.
        for iteration in 0..16 {
            thread_pool.execute(move || {
                println!("Executing the task: {}", iteration.clone());
            });
        }

        assert_eq!(thread_pool.workers.len(), amount_of_threads);
    }

    // Test worker construction and operation.
    #[test]
    fn test_rsa_worker_construction_and_operation() {
        let id = 0;
        let (main_sender, worker_receiver) = mpsc::channel();
        let worker_receiver = Arc::new(Mutex::new(worker_receiver));

        // Construct a worker.
        let _worker = Worker::new(id, worker_receiver);

        // Send a task for execution.
        // Wrap the closure, store it on the heap.
        let job = Box::new(|| {
            println!("Executing a requested task!");
        });

        // Wrap the box with the clojure into the custom enumeration and
        // send the task across the channel to the shared receiver among the threads.
        // The free thread will pick up the task and execute it.
        main_sender.send(ThreadTask::NewJob(job)).unwrap();

        // Send the termination signal to the worker.
        main_sender.send(ThreadTask::Terminate).unwrap();
    }
}
