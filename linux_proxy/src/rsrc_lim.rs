use crate::{Pid, UserAddress};

use protocol::{Error, Oid, err};

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum RlimitId {
    CPU = 0,
    FSIZE = 1,
    DATA = 2,
    STACK = 3,
    CORE = 4,
    RSS = 5,
    NPROC = 6,
    NOFILE = 7,
    MEMLOCK = 8,
    AS = 9,
    LOCKS = 10,
    SIGPENDING = 11,
    MSGQUEUE = 12,
    NICE = 13,
    RPRIO = 14,
    RTTIME = 15,
    NLIMITS = 16,
}

impl RlimitId {
    pub const fn as_usize(self) -> usize {
        self as usize
    }
}

pub const RLIM_INFINITY: u64 = u64::MAX;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RLimit {
    pub rlim_cur: u64, // The current (soft) limit
    pub rlim_max: u64, // The hard limit
}

pub async fn sys_prlimit64(
    pid: Pid,
    resource: u32,
    _new_rlim: *const RLimit,
    _old_rlim: *mut RLimit,
) -> Result<usize, Error> {
    if pid == 0 {
        current_task().process.clone()
    } else {
        return Err(err!(Oid(1), "global pidspace not plumbed"));
    };
    /*
    tree me baby
        let new_limit = if !new_rlim.is_null() {
            Some(copy_from_user(new_rlim).await?)
        } else {
            None
        };

        let old_lim = if let Some(new_limit) = new_limit {
            task.rsrc_lim
                .lock_save_irq()
                .set(resource, new_limit, true)?
        } else {
            task.rsrc_lim.lock_save_irq().get(resource)
        };

        if !old_rlim.is_null() {
            copy_to_user(old_rlim, old_lim).await?
        }
    */
    Ok(0)
}
