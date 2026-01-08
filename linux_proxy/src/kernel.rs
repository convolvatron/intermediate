use crate::{Pid, Process, Runtime};
use alloc::{sync::Arc};
use core::sync::atomic::Ordering;

// this is a logical kernel instance, really just a place to stash all the
// globals from moss-kernel in someplace that isn't so global

pub struct Kernel<R:Runtime> {
    runtime: R,
    pid_count: core::sync::atomic::AtomicU64,
}

impl<R:Runtime> Kernel<R> {
    // might just do these lookups in eav space rather than implement
    // it as DynEntity here
    pub fn get_process(&self, id: Pid) -> Option<Arc<Process<R>>> {
        None
    }
    pub fn next_pid(&self) -> Pid {
        Pid(self.kernel.pid_count, fetch_add(1, Ordering::SeqCst))
    }

    pub fn new(runtime:R) -> Kernel<R>{
        Kernel{runtime, pid_count:1}
    }
}
