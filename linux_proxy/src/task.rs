use crate::{
    Lock, Pid, Process, 
};

use alloc::{sync::Arc};

pub struct Task {
    pub tid: Tid,
    pub process: Arc<Process>,
    pub state: Arc<Lock<TaskState>>,
}

impl Task {
    pub fn new(p: Process) -> Self {
        Self {
            tid: Tid(1),
            process: p,
            state: Arc::new(Lock::new(TaskState::Runnable)),
        }
    }

    pub fn pgid(&self) -> Pid {
        self.process.pid
    }

    pub fn tid(&self) -> Tid {
        self.tid
    }

    //    pub fn raise_task_signal(&self, signal: SigId) {
    //        self.pending_signals.lock_save_irq().insert(signal.into());
    //    }
}

// Thread Id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tid(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Running,
    Runnable,
    Sleeping,
    Finished,
}

impl TaskState {
    pub fn is_finished(self) -> bool {
        matches!(self, Self::Finished)
    }
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}
