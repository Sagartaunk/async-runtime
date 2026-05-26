use crate::types::Task;
use crate::waker::dummy_waker;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// Task Executor.
pub struct Executor {
    queue: Arc<Mutex<VecDeque<Arc<Mutex<Task>>>>>,
}
impl Executor {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
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
        while let Some(task) = {
            let mut t = self.queue.lock().unwrap();
            t.pop_front()
        } {
            let waker = dummy_waker();
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
    }
}
