use crate::{linux::Fd, current_task};
use alloc::sync::Arc;
use crate::error::KernelError;

pub async fn sys_close(fd: Fd) -> Result<usize, KernelError> {
    let file = current_task()
        .fd_table
        .lock_save_irq()
        .remove(fd)
        .ok_or(KernelError::BadFd)?;
}
