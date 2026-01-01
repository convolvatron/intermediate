use crate::{
    memory::uaccess::{UserCopyable, copy_obj_array_from_user},
    linux::Fd,
    current_task,
};
use crate::{
    error::KernelError,
    memory::address::{TUA, UA},
};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct IoVec {
    pub iov_base: UA,
    pub iov_len: usize,
}

// SAFETY: An IoVec is safe to copy to-and-from userspace.
unsafe impl UserCopyable for IoVec {}

pub fn iovec_commands(mut b:Buffer, v:IoVec, u64 offset) {
    for i in v {
        push_command(b,Operation::Copy());
        }
    
}

pub async fn sys_writev(fd: Fd, iov_ptr: TUA<IoVec>, no_iov: usize) -> Result<usize, KernelError> {
    let file = current_task()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;

    let iovs = copy_obj_array_from_user(iov_ptr, no_iov).await?;    
    let (ops, state) = &mut *file.lock().await;
    let mut b = Buffer::new(1024);
    iovec_commands(b, iovs);
    dispatch_block(b);
}

pub async fn sys_readv(fd: Fd, iov_ptr: TUA<IoVec>, no_iov: usize) -> Result<usize, KernelError> {
    let file = current_task()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;

    let iovs = copy_obj_array_from_user(iov_ptr, no_iov).await?;

    let mut b = Buffer::new(1024);
    iovec_commands(b, iovs);
    dispatch_block(b);    
}
