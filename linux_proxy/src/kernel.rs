use crate::{Pid, Process, Runtime};
use alloc::{sync::Arc, vec::Vec};
use core::sync::atomic::Ordering;

// this is a logical kernel instance, really just a place to stash all the
// globals from moss-kernel in someplace that isn't so global

pub struct Kernel<R:Runtime> {
    pub runtime: R,
    processes: Vec<Arc<Process<R>>>,
    pid_count: core::sync::atomic::AtomicU32,
}

impl<R:Runtime> Kernel<R> {
    // might just do these lookups in eav space rather than implement
    // it as DynEntity here
    pub fn get_process(&self, id: Pid) -> Option<Arc<Process<R>>> {
        self.processes.get(id.0 as usize).cloned()
    }
    pub fn next_pid(&self) -> Pid {
        Pid(self.pid_count.fetch_add(1, Ordering::SeqCst))
    }

    pub fn new(runtime:R) -> Kernel<R>{
        Kernel{runtime,
               processes:Vec::new(),
               pid_count: core::sync::atomic::AtomicU32::new(1)}
    }
}
