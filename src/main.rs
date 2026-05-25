use std::{
    future::Pending,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{executor::Executor, types::Task};

mod executor;
mod types;
mod waker;
fn main() {
    let task = Task::new(PendingOnce::new());
    let mut exec = Executor::new();
    exec.add(task);
    exec.run();
}

struct PendingOnce {
    polled: bool,
}
impl PendingOnce {
    fn new() -> Self {
        Self { polled: false }
    }
}
impl Future for PendingOnce {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(())
        } else {
            self.polled = true;
            Poll::Pending
        }
    }
}
