/// Storage for tasks.
pub struct Task {
    task: Pin<Box<dyn Future<Output = ()>>>,
}
