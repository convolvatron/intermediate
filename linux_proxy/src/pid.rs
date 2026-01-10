use protocol::Error;
use core::convert::Infallible;
use crate::{Task, Runtime, linuxerr, Lockable};
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
    Ok(t.process.parent.lock().0 as usize)
}

pub fn sys_getpgid<R:Runtime>(t: Task<R>, pid: Pid) -> Result<usize, Error> {
    let pgid = if pid.0 == 0 {
        t.process.pgid.lock().0
    } else if let Some(tg) = t.process.kernel.get_process(pid) {
        tg.pgid.lock().0
    } else {
        return Err(linuxerr!(ESRCH))
    };

    Ok(pgid as _)
}

pub fn sys_setpgid<R:Runtime>(t: Task<R>, pid: Pid, pgid: Pgid) -> Result<usize, Error> {
    if pid.0 == 0 {
        **t.process.pgid.lock() = pgid;
    } else if let Some(p) = t.process.kernel.get_process(pid) {
        **p.pgid.lock() = pgid;
    } else {
        return Err(linuxerr!(ESRCH));
    };

    Ok(0)
}
