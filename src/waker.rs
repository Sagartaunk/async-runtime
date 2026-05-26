/// This file is a place holder for now. I have yet to fully understand how
/// to implement this and what underlying technologies need to be implemented
/// first.
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    task::{RawWaker, RawWakerVTable, Waker},
};

use crate::types::{Task, WakerData};
static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
/// Clone a `RawWaker`
/// SAFETY: the caller gurantees that `data` is a valid
/// non-null pointer to a live `WakerData` which was created
/// through `Box::into_raw` and has not yet been freed.
unsafe fn clone(data: *const ()) -> RawWaker {
    let data = data as *const WakerData;
    // SAFETY: The pointer was created by `Box::into_raw()` and is still valid
    // and exclusively accessible at this point.
    let queue = unsafe { Arc::clone(&(*data).queue) };
    let task = unsafe { Arc::clone(&(*data).task) };
    let new_waker = Box::new(WakerData {
        queue: queue,
        task: task,
    });
    RawWaker::new(Box::into_raw(new_waker) as *const (), &VTABLE)
}
/// Wake a task
/// SAFETY: the caller gurantees that `data` is a valid
/// non-null pointer to a live `WakerData` which was created
/// through `Box::into_raw` and has not yet been freed.
unsafe fn wake(data: *const ()) {
    let data = data as *mut WakerData;
    // reclaim ownership of data.
    // SAFETY: No one else is using this except us.
    let owned = unsafe { Box::from_raw(data) };
    // In it's own block, so we drop the lock as soon as we have pushed
    // `task` into `queue`.
    {
        owned.queue.lock().unwrap().push_back(owned.task)
    };
}
/// wake a task by taking a reference to it.
/// The `data` pointer remains valid after running `wake_by_ref`.
/// SAFETY: the caller gurantees that `data` is a valid
/// non-null pointer to a live `WakerData` which was created
/// through `Box::into_raw` and has not yet been freed.
unsafe fn wake_by_ref(data: *const ()) {
    let data = data as *const WakerData;
    // SAFETY: the caller gurantees that `data` is a valid
    // non-null pointer to a live `WakerData` which was created
    // through `Box::into_raw` and has not yet been freed.
    let task = Arc::clone(unsafe { &(*data).task });
    unsafe { (*data).queue.lock().unwrap().push_back(task) };
}
/// Remove a task from the table to prevent it from executing any further.
/// SAFETY: the caller gurantees that `data` is a valid
/// non-null pointer to a live `WakerData` which was created
/// through `Box::into_raw` and has not yet been freed.
unsafe fn drop(data: *const ()) {
    let data = data as *mut WakerData;
    // SAFETY : We drop data so no one else can use it.
    // Safety of this function is to be ensured by the caller. Moreover,
    // the caller must ensure that the pointer `data` was created by
    // `Box::into_raw()`.
    let _ = unsafe { Box::from_raw(data) };
}

pub fn task_waker(task: Arc<Mutex<Task>>, queue: Arc<Mutex<VecDeque<Arc<Mutex<Task>>>>>) -> Waker {
    let waker_data = Box::new(WakerData {
        queue: queue,
        task: task,
    });
    let boxed_data = Box::into_raw(waker_data) as *const ();
    unsafe { Waker::from_raw(RawWaker::new(boxed_data, &VTABLE)) }
}
