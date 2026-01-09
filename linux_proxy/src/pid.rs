use protocol::Error;
use core::convert::Infallible;
use crate::{Task, Runtime, Process, linuxerr, Lockable};
use alloc::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pid(pub u32);

impl Pid {
    pub fn value(self) -> u32 {
        self.0
    }

    pub fn is_idle(self) -> bool {
        self.0 == 0
    }
}

impl Display for Pid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// Process Group ID.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pgid(pub u32);

impl Pgid {
    pub fn value(self) -> u32 {
        self.0
    }
}

pub fn sys_getpid<R:Runtime>(t: Task<R>) -> Result<usize, Infallible> {
    Ok(t.process.pgid.lock().0 as usize)
}

pub fn sys_getppid<R:Runtime>(t: Task<R>) -> Result<usize, Infallible> {
    Ok(t.process
        .parent
        .lock()
        .as_ref()
        .and_then(|x| x.upgrade())
        .map(|x| x.pgid.0)
        .unwrap_or(0) as _)
}

pub fn sys_getpgid<R:Runtime>(t: Task<R>, pid: Pid) -> Result<usize, Error> {
    let pgid = if pid.0 == 0 {
        *t.process.pgid.lock()
    } else if let Some(tg) = Process::get(Pgid::from_pid_t(pid)) {
        *tg.pgid.lock()
    } else {
        return Err(Error::NoProcess);
    };

    Ok(pgid.value() as _)
}

pub fn sys_setpgid<R:Runtime>(t: Task<R>, pid: Pid, pgid: Pgid) -> Result<usize, Error> {
    if pid == 0 {
        *t.process.pgid.lock_save_irq() = pgid;
    } else if let Some(tg) = Process::get(Tgid::from_pid_t(pid)) {
        *tg.pgid.lock_save_irq() = pgid;
    } else {
        return Err(linuxerr!(ESRCH));
    };

    Ok(0)
}
