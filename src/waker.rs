/// This file is a place holder for now. I have yet to fully understand how
/// to implement this and what underlying technologies need to be implemented
/// first.
use std::task::{RawWaker, RawWakerVTable, Waker};
static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
/// Clone a `RawWaker`
unsafe fn clone(data: *const ()) -> RawWaker {
    RawWaker::new(data, &VTABLE)
}
/// Wake a task
unsafe fn wake(data: *const ()) {
    // todo!()
}
/// wake a task by taking a reference to it.
unsafe fn wake_by_ref(data: *const ()) {
    // todo!()
}
/// Remove a task from the table to prevent it from executing any further.
unsafe fn drop(data: *const ()) {
    // todo!()
}

pub fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}
