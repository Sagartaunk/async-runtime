use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
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
/// Stores a copy of the `Executor` queue and a `Task`.
/// Acts as a storage struct fot the `Waker`.
struct WakerData {
    queue: Arc<Mutex<VecDeque<Arc<Mutex<Task>>>>>,
    task: Arc<Mutex<Task>>,
}
