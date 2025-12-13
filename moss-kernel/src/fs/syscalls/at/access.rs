use core::ffi::c_char;
use crate::{
    process::fd_table::Fd,
    KernelError,
    memory::address::TUA,
};

pub async fn sys_faccessat(dirfd: Fd, path: TUA<c_char>, mode: i32) -> Result<usize, KernelError> {
    sys_faccessat2(dirfd, path, mode, 0).await
}

pub async fn sys_faccessat2(_dirfd: Fd, _path: TUA<c_char>, _mode: i32, _flags: i32) -> Result<usize, KernelError> {
    Ok(0)
}
