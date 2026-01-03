use protocol::Error;

use crate::current_task;
use core::convert::Infallible;

use super::{Pgid, Tgid, ThreadGroup};

/// Userspace `pid_t` type.
pub type Pid = i32;

pub fn sys_getpid() -> core::result::Result<usize, Infallible> {
    Ok(current_task().process.tgid.value() as _)
}

pub fn sys_getppid() -> core::result::Result<usize, Infallible> {
    Ok(current_task()
        .process
        .parent
        .lock_save_irq()
        .as_ref()
        .and_then(|x| x.upgrade())
        .map(|x| x.tgid.value())
        .unwrap_or(0) as _)
}

pub fn sys_getpgid(pid: Pid) -> Result<usize, Error> {
    let pgid = if pid == 0 {
        *current_task().process.pgid.lock_save_irq()
    } else if let Some(tg) = ThreadGroup::get(Tgid::from_pid_t(pid)) {
        *tg.pgid.lock_save_irq()
    } else {
        return Err(Error::NoProcess);
    };

    Ok(pgid.value() as _)
}

pub fn sys_setpgid(pid: Pid, pgid: Pgid) -> Result<usize, Error> {
    if pid == 0 {
        *current_task().process.pgid.lock_save_irq() = pgid;
    } else if let Some(tg) = ThreadGroup::get(Tgid::from_pid_t(pid)) {
        *tg.pgid.lock_save_irq() = pgid;
    } else {
        return Err(Error::NoProcess);
    };

    Ok(0)
}
