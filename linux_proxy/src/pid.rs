use protocol::Error;
use core::convert::Infallible;
use crate::{Task};
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

    pub fn from_pid_t(pid: Pid) -> Pid {
        Self(pid as _)
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

pub fn sys_getpid(t: Task) -> Result<usize, Infallible> {
    Ok(t.process.tgid.value() as _)
}

pub fn sys_getppid(t: Task) -> Result<usize, Infallible> {
    Ok(t.process
        .parent
        .lock_save_irq()
        .as_ref()
        .and_then(|x| x.upgrade())
        .map(|x| x.tgid.value())
        .unwrap_or(0) as _)
}

pub fn sys_getpgid(t: Task, pid: Pid) -> Result<usize, Error> {
    let pgid = if pid == 0 {
        *t.process.pgid.lock_save_irq()
    } else if let Some(tg) = ThreadGroup::get(Tgid::from_pid_t(pid)) {
        *tg.pgid.lock_save_irq()
    } else {
        return Err(Error::NoProcess);
    };

    Ok(pgid.value() as _)
}

pub fn sys_setpgid(t: Task, pid: Pid, pgid: Pgid) -> Result<usize, Error> {
    if pid == 0 {
        *t.process.pgid.lock_save_irq() = pgid;
    } else if let Some(tg) = ThreadGroup::get(Tgid::from_pid_t(pid)) {
        *tg.pgid.lock_save_irq() = pgid;
    } else {
        return Err(Error::NoProcess);
    };

    Ok(0)
}
