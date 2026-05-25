use std::pin::Pin;

/// Storage for tasks.

pub struct Task {
    pub task: Pin<Box<dyn Future<Output = ()>>>,
}
