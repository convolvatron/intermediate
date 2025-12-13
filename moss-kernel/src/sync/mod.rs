pub mod condvar;
pub mod mpsc;
pub mod mutex;
pub mod once_lock;
pub mod per_cpu;
pub mod spinlock;
pub mod waker_set;

use crate::arch::ArchImpl;
pub use per_cpu::*;

pub type SpinLock<T> = spinlock::SpinLockIrq<T, ArchImpl>;
pub type Mutex<T> = mutex::Mutex<T, ArchImpl>;
pub type AsyncMutexGuard<'a, T> = mutex::AsyncMutexGuard<'a, T, ArchImpl>;
pub type OnceLock<T> = once_lock::OnceLock<T, ArchImpl>;
pub type CondVar<T> = condvar::CondVar<T, ArchImpl>;
