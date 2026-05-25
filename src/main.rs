use crate::{executor::Executor, types::Task};

mod executor;
mod types;
mod waker;
fn main() {
    let future = async {};
    let task = Task::new(future);
    let mut exec = Executor::new();
    exec.add(task);
    exec.run();
}
