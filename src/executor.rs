use crate::reactor::Reactor;
use crate::types::Task;
use crate::waker::task_waker;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

/// Task Executor.
pub struct Executor {
    queue: Arc<Mutex<VecDeque<Arc<Mutex<Task>>>>>,
    reactor: Arc<Mutex<Reactor>>,
}
impl Executor {
    /// Create a new `Executor` by creating an empty queue internally
    /// and taking a `Arc<Mutex<Reactor>>`.
    pub fn new(reactor: Arc<Mutex<Reactor>>) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            reactor: reactor,
        }
    }
    /// Adds an existing `Arc<Mutex<Task>>` to the executor queue.
    pub fn add(&mut self, task: Arc<Mutex<Task>>) {
        self.queue.lock().unwrap().push_back(task);
    }
    /// Wraps a `Task` in `Arc<Mutex<_>>` and adds it to the executor queue.
    ///
    /// Unlike `add`, this method takes ownership of a plain `Task`.
    pub fn spawn(&mut self, task: Task) {
        self.queue
            .lock()
            .unwrap()
            .push_back(Arc::new(Mutex::new(task)));
    }
    /// Runs the executor event loop until there are no runnable tasks
    /// in the executor queue and no pending I/O registrations in the reactor.
    pub fn run(&mut self) {
        loop {
            let task = {
                let mut t = self.queue.lock().unwrap();
                t.pop_front()
            }; // lock dropped here
            // Check if there are no pending tasks left in reactor queue and
            // executor queue and exit the executor cleanely.
            {
                if self.reactor.lock().unwrap().is_empty() && task.is_none() {
                    break;
                }
            }
            // If a runnable task exists, poll it. Otherwise, block in the
            // reactor until an I/O event wakes a task and requeues it.
            match task {
                Some(task) => {
                    let waker = task_waker(Arc::clone(&task), Arc::clone(&self.queue));
                    let mut context = Context::from_waker(&waker);
                    let result = {
                        let mut t = task.lock().unwrap();
                        t.task.as_mut().poll(&mut context)
                    };
                    match result {
                        Poll::Ready(()) => {}
                        Poll::Pending => {}
                    }
                }
                None => {
                    self.reactor.lock().unwrap().wait();
                }
            }
        }
    }
}
