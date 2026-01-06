use crate::{Pid, Process};
use alloc::{vec::Vec, sync::Arc};
use core::sync::atomic::Ordering;
use protocol::{Command, Error};

// this is a logical kernel instance, really just a place to stash all the
// globals from moss-kernel in someplace that isn't so global

pub struct Kernel {
    pid_count: core::sync::atomic::AtomicU64,
}

impl Kernel {
    // might just do these lookups in eav space rather than implement
    // it as DynEntity here
    pub fn get_process(&self, id: Pid) -> Option<Arc<Process>> {
        None
    }
    pub fn next_pid(&self) -> Pid {
        Pid(self.kernel.pid_count, fetch_add(1, Ordering::SeqCst))
    }

    pub fn execute(&self, program: Vec<Command>) -> Result<(), Error> {}
}
