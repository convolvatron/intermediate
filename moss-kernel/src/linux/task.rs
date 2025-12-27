
use protocol::{Oid, Entity, Attribute, Value};

use crate::{
    Context,
    per_cpu,
    Instant,
    PathBuf,
    RobustListHead,
    arch::{Arch, ArchImpl},
    sync::SpinLock,
    memory::address::TUA,
    VirtualMemory,
    memory::{
        address::VA,
        proc_vm::ProcessVM,
    },
    creds::Credentials,
    fd_table::FileDescriptorTable,
    
    linux::{
        Tgid, ThreadGroup,
        ThreadGroupBuilder,
    },
};

use alloc::{
    collections::btree_map::BTreeMap,
    sync::{Arc, Weak},
};

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

/// A unqiue identifier for any task in the current system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskDescriptor {
    tid: Tid,
    tgid: Tgid,
}

impl TaskDescriptor {
    pub fn from_tgid_tid(tgid: Tgid, tid: Tid) -> Self {
        Self { tid, tgid }
    }

    /// Returns a descriptor for the idle task.
    pub fn this_cpus_idle() -> Self {
        Self {
            tgid: Tgid(0),
            tid: Tid(0),
        }
    }

    /// Returns a representation of a descriptor encoded in a single pointer
    /// value.
    #[cfg(target_pointer_width = "64")]
    pub fn to_ptr(self) -> *const () {
        let mut value: u64 = self.tgid.value() as _;

        value |= (self.tid.value() as u64) << 32;

        value as _
    }

    /// Returns a descriptor decoded from a single pointer value. This is the
    /// inverse of `to_ptr`.
    #[cfg(target_pointer_width = "64")]
    pub fn from_ptr(ptr: *const ()) -> Self {
        let value = ptr as u64;

        let tgid = value & 0xffffffff;
        let tid = value >> 32;

        Self {
            tgid: Tgid(tgid as _),
            tid: Tid(tid as _),
        }
    }

    pub fn is_idle(&self) -> bool {
        self.tgid.is_idle()
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
pub type ProcVM = ProcessVM<<ArchImpl as VirtualMemory>::ProcessAddressSpace>;

pub struct Task {
    pub tid: Tid,
    pub process: Arc<ThreadGroup>,
    pub vm: Arc<SpinLock<ProcVM>>,
    pub cwd: Arc<SpinLock<Oid>>,
    pub creds: SpinLock<Credentials>,
    pub fd_table: Arc<SpinLock<FileDescriptorTable>>,
    pub ctx: SpinLock<Context>,
//    pub sig_mask: SpinLock<SigSet>,
//    pub pending_signals: SpinLock<SigSet>,
    pub vruntime: SpinLock<u64>,
    pub exec_start: SpinLock<Option<Instant>>,
    pub deadline: SpinLock<Option<Instant>>,
    pub priority: i8,
    pub last_run: SpinLock<Option<Instant>>,
    pub state: Arc<SpinLock<TaskState>>,
    pub robust_list: SpinLock<Option<TUA<RobustListHead>>>,
}

impl Task {
    pub fn new(working: Oid) -> Self {
        Self {
            tid: Tid(1),
            process: ThreadGroupBuilder::new(Tgid::init()).build(),
            state: Arc::new(SpinLock::new(TaskState::Runnable)),
            cwd: Arc::new(SpinLock::new((working, PathBuf::new()))),
            creds: SpinLock::new(Credentials::new_root()),
            vm: Arc::new(SpinLock::new(
                ProcessVM::empty().expect("Could not create init process's VM"),
            )),
            fd_table: Arc::new(SpinLock::new(FileDescriptorTable::new())),
            pending_signals: SpinLock::new(SigSet::empty()),
            vruntime: SpinLock::new(0),
            exec_start: SpinLock::new(None),
            deadline: SpinLock::new(None),
            sig_mask: SpinLock::new(SigSet::empty()),
            priority: 0,
            ctx: SpinLock::new(Context::from_user_ctx(
                <ArchImpl as Arch>::new_user_context(VA::null(), VA::null()),
            )),
            last_run: SpinLock::new(None),
            robust_list: SpinLock::new(None),
        }
    }

    pub fn priority(&self) -> i8 {
        self.priority
    }

    pub fn set_priority(&mut self, priority: i8) {
        self.priority = priority;
    }

    pub fn pgid(&self) -> Tgid {
        self.process.tgid
    }

    pub fn tid(&self) -> Tid {
        self.tid
    }

    /// Return a new desctiptor that uniquely represents this task in the
    /// system.
    pub fn descriptor(&self) -> TaskDescriptor {
        TaskDescriptor::from_tgid_tid(self.process.tgid, self.tid)
    }

//    pub fn raise_task_signal(&self, signal: SigId) {
//        self.pending_signals.lock_save_irq().insert(signal.into());
//    }
}

pub static TASK_LIST: SpinLock<BTreeMap<TaskDescriptor, Weak<SpinLock<TaskState>>>> =
    SpinLock::new(BTreeMap::new());

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

per_cpu! {
    static CURRENT_TASK: Option<Task> = None;
}

pub fn current_task() -> Task {
    CURRENT_TASK.clone()
}


pub struct TaskTable {
}

impl Entity for TaskTable {
    fn keys() -> alloc::vec::Vec<Attribute> {
    }
    fn get(a:Attribute) -> Value {
    }
    fn set(a:Attribute, v:Value) {
    }
}
