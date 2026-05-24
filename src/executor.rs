use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::future::Future;
use std::pin::Pin;

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
            }{
            // Todo: Implement this when written.
            let waker = todo!();
            let mut context = todo!();
            match task.poll(&mut context) {
                Poll::Ready(()) => {// Task Done.},
                Poll::Pending => self.add(task),
            }
        }
    }
}
