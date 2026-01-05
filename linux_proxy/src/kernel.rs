use crate::Pid;
use alloc::vec::Vec;
use core::sync::atomic::Ordering;
use protocol::Command;

// this is a logical kernel instance, really just a place to stash all the
// globals from moss-kernel in someplace that isn't so global

pub struct Kernel {
    pid_count: alloc::sync::Atomic::u64,
}

impl Kernel {
    pub fn next_pid(&self) -> Pid {
        Pid(self.kernel.pid_count, fetch_add(1, Ordering::SeqCst))
    }

    pub fn execute(&self, program: Vec<Command>) -> Result<(), Error> {}
}
