use crate::types::Task;
use crate::waker::dummy_waker;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// Task Executor.
pub struct Executor {
    queue: Arc<Mutex<VecDeque<Task>>>,
}
impl Executor {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    pub fn add(&mut self, task: Task) {
        self.queue.lock().unwrap().push_back(task);
    }
    pub fn run(&mut self) {
        while let Some(mut task) = {
            let mut t = self.queue.lock().unwrap();
            t.pop_front()
        } {
            // Todo: Implement this when written.
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);
            match task.task.as_mut().poll(&mut context) {
                Poll::Ready(()) => {}
                Poll::Pending => self.add(task),
            }
        }
    }
}
