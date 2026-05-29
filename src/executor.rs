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
    pub fn new(reactor: Arc<Mutex<Reactor>>) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            reactor: reactor,
        }
    }
    pub fn add(&mut self, task: Arc<Mutex<Task>>) {
        self.queue.lock().unwrap().push_back(task);
    }
    pub fn spawn(&mut self, task: Task) {
        self.queue
            .lock()
            .unwrap()
            .push_back(Arc::new(Mutex::new(task)));
    }
    pub fn run(&mut self) {
        loop {
            let task = {
                let mut t = self.queue.lock().unwrap();
                t.pop_front()
            }; // lock dropped here
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
                        Poll::Pending => self.add(task),
                    }
                }
                None => {
                    self.reactor.lock().unwrap().wait();
                }
            }
        }
    }
}
