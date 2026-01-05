use protocol::{DynEntity, Oid};

use crate::{
    Lock, Pid, Process, RobustListHead, UserAddress, creds::Credentials,
    fd_table::FileDescriptorTable,
};

use alloc::{
    collections::btree_map::BTreeMap,
    string::String,
    sync::{Arc, Weak},
};

pub struct Task {
    pub tid: Tid,
    pub process: Arc<Process>,
    pub state: Arc<Lock<TaskState>>,
}

impl Task {
    pub fn new(cwd: Oid) -> Self {
        Self {
            tid: Tid(1),
            process: ThreadGroupBuilder::new(Tgid::init()).build(),
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

impl Tid {
    pub fn value(self) -> u32 {
        self.0
    }

    pub fn from_tgid(tgid: Tgid) -> Self {
        Self(tgid.0)
    }
}

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
