use crate::{
    Pid, Process, Runtime, Lockable,
};

use alloc::{sync::Arc};

pub struct Task<R:Runtime> {
    pub tid: Tid,
    pub process: Arc<Process<R>>,
    pub state: Arc<R::Lock<TaskState>>,
}

impl<R:Runtime> Task<R> {
    pub fn new(p: Arc<Process<R>>) -> Self {
        Self {
            tid: Tid(1),
            process: p,
            state: Arc::new(R::Lock::new(TaskState::Runnable)),
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

unsafe impl<R:Runtime> Send for Task<R> {}
unsafe impl<R:Runtime> Sync for Task<R> {}
