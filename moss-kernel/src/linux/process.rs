pub type ProcVM = ProcessVM<<ArchImpl as VirtualMemory>::ProcessAddressSpace>;

pub struct Process {
    pub tid: Tid,
    pub threads: Arc<ThreadGroup>,
    pub vm: Arc<SpinLock<ProcVM>>,
    pub cwd: Arc<SpinLock<(Oid, PathBuf)>>,
    pub creds: SpinLock<Credentials>,
    pub fd_table: Arc<SpinLock<FileDescriptorTable>>,
    pub ctx: SpinLock<crate::task::Context>,
    pub sig_mask: SpinLock<SigSet>,
    pub pending_signals: SpinLock<SigSet>,
    pub vruntime: SpinLock<u64>,
    pub exec_start: SpinLock<Option<Instant>>,
    pub deadline: SpinLock<Option<Instant>>,
    pub priority: i8,
    pub last_run: SpinLock<Option<Instant>>,
    pub state: Arc<SpinLock<TaskState>>,
    pub robust_list: SpinLock<Option<TUA<crate::linux::RobustListHead>>>,
}

pub struct ProcessTable {
}

impl Entity for ProcessTable {
    fn keys() -> Iterator<Property> {
    }
    fn get(a:Attribute) -> Value {
    }
    fn set(a:Attribute, v:Value) {
    }
}
