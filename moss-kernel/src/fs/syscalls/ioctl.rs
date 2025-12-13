use crate::{process::fd_table::Fd, sched::current_task};
use crate::error::KernelError;

pub async fn sys_ioctl(fd: Fd, request: usize, arg: usize) -> Result<usize, KernelError> {
    let fd = current_task()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;

    let (ops, ctx) = &mut *fd.lock().await;
    ops.ioctl(ctx, request, arg).await
}
