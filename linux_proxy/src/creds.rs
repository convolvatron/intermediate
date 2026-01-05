use core::convert::Infallible;
use protocol::Error;

use crate::{Gid, Uid, UserAddress, Task};

#[derive(Clone, PartialEq, Eq)]
pub struct Credentials {
    uid: Uid,
    euid: Uid,
    suid: Uid,
    gid: Gid,
    egid: Gid,
    sgid: Gid,
}

impl Credentials {
    pub fn new_root() -> Self {
        Self {
            uid: Uid::new_root(),
            euid: Uid::new_root(),
            suid: Uid::new_root(),
            gid: Gid::new_root_group(),
            egid: Gid::new_root_group(),
            sgid: Gid::new_root_group(),
        }
    }

    pub fn uid(&self) -> Uid {
        self.uid
    }

    pub fn euid(&self) -> Uid {
        self.euid
    }

    pub fn suid(&self) -> Uid {
        self.suid
    }

    pub fn gid(&self) -> Gid {
        self.gid
    }

    pub fn egid(&self) -> Gid {
        self.egid
    }

    pub fn sgid(&self) -> Gid {
        self.sgid
    }
}

pub fn sys_getuid() -> core::result::Result<usize, Infallible> {
    let uid: u32 = current_task().creds.lock_save_irq().uid().into();

    Ok(uid as _)
}

pub fn sys_geteuid() -> core::result::Result<usize, Infallible> {
    let uid: u32 = current_task().creds.lock_save_irq().euid().into();

    Ok(uid as _)
}

pub fn sys_getgid() -> core::result::Result<usize, Infallible> {
    let gid: u32 = current_task().creds.lock_save_irq().gid().into();

    Ok(gid as _)
}

pub fn sys_getegid() -> core::result::Result<usize, Infallible> {
    let gid: u32 = current_task().creds.lock_save_irq().egid().into();

    Ok(gid as _)
}

pub fn sys_gettid() -> core::result::Result<usize, Infallible> {
    let tid: u32 = current_task().tid.0;

    Ok(tid as _)
}

pub async fn sys_getresuid(
    t:Task,
    _ruid: *mut Uid,
    _euid: *mut Uid,
    _suid: *mut Uid,
) -> Result<usize, Error> {
    let creds = t.process.creds.lock_save_irq().clone();

    /*    copy_to_user(ruid, creds.uid).await?;
        copy_to_user(euid, creds.euid).await?;
        copy_to_user(suid, creds.suid).await?;
    */
    Ok(0)
}

pub async fn sys_getresgid(
    rgid: UserAddress,
    egid: UserAddress,
    sgid: UserAddress,
) -> Result<usize, Error> {
    let task = current_task();
    let _creds = task.creds.lock_save_irq().clone();

    /*    copy_to_user(rgid, creds.gid).await?;
        copy_to_user(egid, creds.egid).await?;
        copy_to_user(sgid, creds.sgid).await?;
    */
    Ok(0)
}
