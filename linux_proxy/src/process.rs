use crate::{
    Runtime,
    Credentials, Kernel, linuxerr, Lockable,
    FileDescriptorEntry,
    Fd,
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

pub struct Process<R:Runtime> {
    pub kernel: Arc<Kernel<R>>,
    pub myself: Oid,
    pub pid: Pid,
    pub pgid: R::Lock<Pgid>,
    pub sid: R::Lock<Sid>,
    pub state: R::Lock<ProcessState>,
    pub umask: R::Lock<u32>,
    pub parent: R::Lock<Pid>,
    pub children: R::Lock<BTreeMap<Pid, Arc<Process<R>>>>,
    pub threads: R::Lock<BTreeMap<Tid, Weak<Task<R>>>>,
    pub fd_table: R::Lock<Vec<Option<FileDescriptorEntry>>>,
    pub robust_list: R::Lock<Option<*mut RobustListHead>>,
    pub creds: R::Lock<Credentials>,
    // we keep the path used to traverse to this objet since its
    // not unique, valuable user context, and very costly to ennumerate
    pub cwd: R::Lock<(Oid, Path)>,
    pub next_fd_hint: usize,
    pub next_tid: AtomicU32,
}

unsafe impl<R:Runtime> Send for Process<R> {}

impl<R:Runtime> Process<R> {
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
    
    pub fn get_fd(&self, fd: Fd) -> Result<FileDescriptorEntry, Error> {
        Ok(self.fd_table.lock().get(fd.0 as usize))
    }
}

impl<R:Runtime> Drop for Process<R> {
    fn drop(&mut self) {
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

pub async fn sys_set_robust_list<R:Runtime>(t:Task<R>, head: *mut RobustListHead, len: usize) -> Result<usize, Error> {
    if len != size_of::<RobustListHead>() {
        return Err(linuxerr!(EINVAL));
    }

    t.process.robust_list.lock().replace(head);

    Ok(0)
}
