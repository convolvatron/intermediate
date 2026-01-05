use crate::{
    Credentials, Kernel, Lock, linuxerr,
    FileDescriptorEntry,
    Pid,
    Pgid,
    Path,
    Task,
    Tid,
};
use core::ffi::c_long;
use protocol::{Error, Oid};

use core::sync::atomic::AtomicU32;

use alloc::{
    vec::Vec,    
    collections::btree_map::BTreeMap,
    sync::{Arc, Weak},
};

use core::{sync::atomic::Ordering};


/// Session ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sid(pub u32);

impl Sid {
    pub fn value(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running, // Actively running
    Exiting, // In the middle of being torn down
}

pub struct Process {
    pub kernel: Arc<Kernel>,
    pub pid: Pid,
    pub pgid: Lock<Pgid>,
    pub sid: Lock<Sid>,
    pub state: Lock<ProcessState>,
    pub umask: Lock<u32>,
    pub parent: Lock<Option<Weak<Process>>>,
    pub children: Lock<BTreeMap<Pid, Arc<Process>>>,
    pub threads: Lock<BTreeMap<Tid, Weak<Task>>>,

    fd_table: Arc<Vec<FileDescriptorEntry>>,
    pub robust_list: Lock<Option<*mut RobustListHead>>,
    pub creds: Lock<Credentials>,
    // we keep the path used to traverse to this objet since its
    // not unique, valuable user context, and very costly to ennumerate
    pub cwd: Arc<Lock<(Oid, Path)>>,
    next_fd_hint: usize,
    next_tid: AtomicU32,
}

unsafe impl Send for Process {}

impl Process {
    // Return the next avilable thread id. Will never return a thread who's ID
    // == PID, since that is defined as the main, root thread.
    pub fn next_tid(&self) -> Tid {
        let mut v = self.next_tid.fetch_add(1, Ordering::SeqCst);

        // Skip the PID.
        if v == self.pid.value() {
            v = self.next_tid.fetch_add(1, Ordering::SeqCst)
        }

        Tid(v)
    }

    pub fn get(&self, id: Pid) -> Option<Arc<Self>> {
        self.kernel
            .tg_list
            .lock_save_irq()
            .get(&id)
            .and_then(|x| x.upgrade())
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        self.kernel.TG_LIST.lock()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RobustList {
    next: *mut RobustList,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RobustListHead {
    list: RobustList,
    futex_offset: c_long,
    list_op_pending: RobustList,
}

pub async fn sys_set_robust_list(head: *const RobustListHead, len: usize) -> Result<usize, Error> {
    if len != size_of::<RobustListHead>() {
        return Err(linuxerr!(EINVAL));
    }

    let task = current_task();
    task.robust_list.lock_save_irq().replace(head);

    Ok(0)
}
