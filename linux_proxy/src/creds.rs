use core::convert::Infallible;
use protocol::Error;

use crate::{Gid, Uid, Task, Runtime, Lockable};

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

pub fn sys_getuid<R:Runtime>(mut t: Task<R>) -> core::result::Result<usize, Infallible> {
    let uid: u32 = t.process.creds.lock().uid().into();
    Ok(uid as _)
}

pub fn sys_geteuid<R:Runtime>(t:Task<R>) -> core::result::Result<usize, Infallible> {
    let uid: u32 = t.process.creds.lock().euid().into();
    Ok(uid as _)
}

pub fn sys_getgid<R:Runtime>(t:Task<R>) -> core::result::Result<usize, Infallible> {
    let gid: u32 = t.process.creds.lock().gid().into();
    Ok(gid as _)
}

pub fn sys_getegid<R:Runtime>(t:Task<R>) -> core::result::Result<usize, Infallible> {
    let gid: u32 = t.process.creds.lock().egid().into();
    Ok(gid as _)
}

pub fn sys_gettid<R:Runtime>(t:Task<R>) -> core::result::Result<usize, Infallible> {
    let tid: u32 = t.tid.0;
    Ok(tid as _)
}

pub async fn sys_getresuid<R:Runtime>(
    t: Task<R>,
    ruid: *mut Uid,
    euid: *mut Uid,
    suid: *mut Uid,
) -> Result<usize, Error> {
    let creds = t.process.creds.lock().clone();

    *ruid = creds.uid;
    *euid = creds.euid;
    *suid = creds.suid;        
    
    Ok(0)
}

pub async fn sys_getresgid<R:Runtime>(
    t:Task<R>,
    rgid: *mut Gid,
    egid: *mut Gid,
    sgid: *mut Gid
) -> Result<usize, Error> {
    let creds = t.process.creds.lock().clone();
    *rgid = creds.gid;
    *egid = creds.egid;
    *sgid = creds.sgid;        
    Ok(0)
}
