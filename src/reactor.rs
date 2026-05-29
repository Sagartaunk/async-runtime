use std::{collections::HashMap, os::fd::RawFd, task::Waker};

use libc::{
    EPOLL_CLOEXEC, EPOLL_CTL_ADD, EPOLL_CTL_MOD, EPOLLIN, epoll_create1, epoll_ctl, epoll_wait,
};

pub struct Reactor {
    epoll_fd: RawFd,
    waker: HashMap<RawFd, Waker>,
}
impl Reactor {
    /// Creates a new `epoll_fd` internally and returns a `Reactor`
    /// struct.
    pub fn new() -> Self {
        // SAFETY: The `epoll_fd` automatically closes when a new process is
        // executed due to the `EPOLL_CLOEXEC` flag. Hence, preventing dangling
        // fd's. Also, the program panic's if we failed to create a fd.
        let epoll = unsafe { epoll_create1(EPOLL_CLOEXEC) };
        assert!(epoll != -1, "epoll_create1 failed");
        Reactor {
            epoll_fd: epoll,
            waker: HashMap::new(),
        }
    }
    /// Registers a new process to be watches into the `epoll_fd`.
    /// SAFETY: The caller must ensure that the `fd` being passed is valid.
    pub fn register(&mut self, fd: RawFd, waker: Waker) {
        let mut event = libc::epoll_event {
            events: EPOLLIN as u32,
            u64: fd as u64,
        };
        let op = if self.waker.contains_key(&fd) {
            EPOLL_CTL_MOD
        } else {
            EPOLL_CTL_ADD
        };
        // SAFETY: `epoll_ctl` adds a new fd to be watched. It is the responsibility
        // of the caller to insure that the `fd` being passed is valid. Moreover, the
        // program will panic if the call to add the `fd` failed.
        let epoll = unsafe { epoll_ctl(self.epoll_fd, op, fd, &mut event) };
        assert!(epoll != -1, "epoll_ctl failed");
        self.waker.insert(fd, waker);
    }
    /// Wait for event's to fire a signal to proceed.
    /// SAFETY: Guranteed by the safety contract of `register` method on `Reactor` struct.
    pub fn wait(&mut self) {
        // This vector is a writing space for the `epoll_wait` call on the fd's.
        // SAFETY: `epoll_wait` manages this and no one else accesses it.
        let mut events = vec![unsafe { std::mem::zeroed::<libc::epoll_event>() }; 32];
        // We wait forever until an event fires from the kernel stating that something is
        // ready to proceed or has a result.
        // SAFETY: The fd's we are waiting on are guranteed to be valid because of the
        // security contract of `register` method on `Reactor` struct.
        let wait =
            unsafe { epoll_wait(self.epoll_fd, events.as_mut_ptr(), events.len() as i32, -1) };
        // Panic if wait failed.
        assert!(wait != -1, "epoll_wait failed");
        for event in &events[..wait as usize] {
            let fd = event.u64 as RawFd;

            if let Some(waker) = self.waker.remove(&fd) {
                waker.wake();
            }
        }
    }
}
