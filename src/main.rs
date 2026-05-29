use std::{
    future::Future,
    os::fd::RawFd,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    thread,
    time::Duration,
};

use libc::{EAGAIN, EWOULDBLOCK, F_GETFL, F_SETFL, O_NONBLOCK, fcntl, pipe, read, write};

use crate::{executor::Executor, reactor::Reactor, types::Task};

mod executor;
mod reactor;
mod types;
mod waker;

fn main() {
    let mut fds = [0; 2];

    unsafe {
        assert_ne!(pipe(fds.as_mut_ptr()), -1);
    }

    let read_fd = fds[0];
    let write_fd = fds[1];

    unsafe {
        let flags = fcntl(read_fd, F_GETFL);
        assert_ne!(flags, -1);

        assert_ne!(fcntl(read_fd, F_SETFL, flags | O_NONBLOCK), -1);
    }

    let reactor = Arc::new(Mutex::new(Reactor::new()));

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));

        let msg = b"hello from pipe";

        unsafe {
            write(write_fd, msg.as_ptr() as *const libc::c_void, msg.len());
        }

        println!("writer thread wrote");
    });

    let future = AsyncRead::new(read_fd, reactor.clone());

    let mut executor = Executor::new(reactor.clone());

    executor.spawn(Task::new(future));

    executor.run();
}

struct AsyncRead {
    fd: RawFd,
    reactor: Arc<Mutex<Reactor>>,
}

impl AsyncRead {
    fn new(fd: RawFd, reactor: Arc<Mutex<Reactor>>) -> Self {
        Self { fd, reactor }
    }
}

impl Future for AsyncRead {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut buf = [0u8; 1024];

        let ret = unsafe { read(self.fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };

        if ret > 0 {
            let n = ret as usize;

            println!("READ: {}", String::from_utf8_lossy(&buf[..n]));

            return Poll::Ready(());
        }

        let err = std::io::Error::last_os_error();

        match err.raw_os_error() {
            Some(EAGAIN) | Some(EWOULDBLOCK) => {
                println!("pending");

                self.reactor
                    .lock()
                    .unwrap()
                    .register(self.fd, cx.waker().clone());

                Poll::Pending
            }

            _ => {
                panic!("read failed: {err}");
            }
        }
    }
}
