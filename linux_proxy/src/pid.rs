use protocol::Error;

use core::convert::Infallible;

use crate::{Pgid, Process, Task};

/// Userspace `pid_t` type.
pub type Pid = i32;

pub fn sys_getpid(t:Task) -> core::result::Result<usize, Infallible> {
    Ok(t.process.tgid.value() as _)
}

pub fn sys_getppid(t:Task) -> core::result::Result<usize, Infallible> {
    Ok(t.process
       .parent
       .lock_save_irq()
       .as_ref()
       .and_then(|x| x.upgrade())
       .map(|x| x.tgid.value())
       .unwrap_or(0) as _)
}

pub fn sys_getpgid(t:Task, pid: Pid) -> Result<usize, Error> {
    let pgid = if pid == 0 {
        *t.process.pgid.lock_save_irq()
    } else if let Some(tg) = ThreadGroup::get(Tgid::from_pid_t(pid)) {
        *tg.pgid.lock_save_irq()
    } else {
        return Err(Error::NoProcess);
    };

    Ok(pgid.value() as _)
}

pub fn sys_setpgid(t:Task, pid: Pid, pgid: Pgid) -> Result<usize, Error> {
    if pid == 0 {
        *t.process.pgid.lock_save_irq() = pgid;
    } else if let Some(tg) = ThreadGroup::get(Tgid::from_pid_t(pid)) {
        *tg.pgid.lock_save_irq() = pgid;
    } else {
        return Err(Error::NoProcess);
    };

    Ok(0)
}
