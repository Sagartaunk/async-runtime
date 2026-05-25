use std::future::Future;
use std::pin::Pin;
/// Storage for tasks.

pub struct Task {
    pub task: Pin<Box<dyn Future<Output = ()>>>,
}
impl Task {
    pub fn new(fut: impl Future<Output = ()> + 'static) -> Self {
        Self {
            task: Box::pin(fut),
        }
    }
}
