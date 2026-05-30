# async-runtime

A small async runtime written in Rust as a learning project.

The runtime consists of three primary components:

* An `Executor` that polls runnable futures.
* A `Reactor` built on Linux `epoll`.
* A custom `Waker` implementation that requeues tasks when they become runnable again.

The goal of the project is to learn the basic relationship between futures, executors, wakers, and an event-driven reactor.

## Architecture

### Executor

The executor owns a queue of runnable tasks.

Each task is stored as:

```rust
Arc<Mutex<Task>>
```

The executor repeatedly:

1. Removes a task from the front of the queue.
2. Creates a custom waker for that task.
3. Polls the future.
4. Continues if the future returns `Poll::Pending`.
5. Drops the task if it returns `Poll::Ready(())`.

If no runnable tasks exist, the executor blocks inside the reactor until an I/O event occurs.

The executor terminates when:

* The runnable task queue is empty.
* The reactor has no registered pending I/O operations.

---

### Reactor

The reactor is responsible for waiting on operating system events.

Internally it maintains:

```rust
HashMap<RawFd, Waker>
```

and an `epoll` instance.

When a future cannot make progress because it is waiting on I/O, it registers:

* The file descriptor to monitor.
* The task's waker.

The reactor then:

1. Waits inside `epoll_wait`.
2. Receives readiness notifications from the kernel.
3. Looks up the corresponding waker.
4. Wakes the associated task.
5. Removes the registration from its internal table.

Currently the reactor watches for readable events using `EPOLLIN`.

---

### Waker

The runtime provides a custom implementation of Rust's `RawWaker`.

Each waker stores:

```rust
struct WakerData {
    queue: Arc<Mutex<VecDeque<Arc<Mutex<Task>>>>>,
    task: Arc<Mutex<Task>>,
}
```

When a task is woken:

* The task is pushed back into the executor queue.
* The executor will poll it again during a later iteration.

The implementation provides:

* `clone`
* `wake`
* `wake_by_ref`
* `drop`

through a custom `RawWakerVTable`.

## Task Model

A task is simply a boxed future:

```rust
pub struct Task {
    pub task: Pin<Box<dyn Future<Output = ()>>>,
}
```

The runtime only supports futures with:

```rust
Output = ()
```

## Platform Support

This runtime currently targets Linux.

The reactor depends on:

* `epoll_create1`
* `epoll_ctl`
* `epoll_wait`

and therefore is not portable to non-Linux platforms in its current form.

## Limitations

This project intentionally implements only a minimal subset of functionality.

Current limitations include:

* Linux-only implementation.
* No timer support.
* No sleep future.
* Single-threaded executor.
* No task cancellation.
* No task joining or result retrieval.
* No work-stealing scheduler.
* No support for futures returning values.
* Reactor only handles readiness notifications and does not provide higher-level async abstractions.
* Heavy use of `Mutex` even though execution is single-threaded.
* Panic-based error handling for system call failures.
* No graceful handling of closed or invalid file descriptors.
* No executor metrics, tracing, or debugging facilities.

## TODO

* Implement timer future.

## Project Structure

```text
src/
├── executor.rs
├── reactor.rs
├── types.rs
└── waker.rs
```

## Note

This async-runtime was supposed to be a learning project and will not be recieving any further feature updates.
