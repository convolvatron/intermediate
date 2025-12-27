use crate::{
    memory::uaccess::{copy_to_user_slice},
    current_task,
};
use alloc::{ffi::CString, string::ToString};
use core::{ffi::c_char, str::FromStr};
use crate::{
    error::KernelError,
    memory::address::{TUA, UA},
};

pub async fn sys_getcwd(buf: UA, len: usize) -> Result<usize, KernelError> {
    let task = current_task();
    let path = task.cwd.lock_save_irq().1.as_str().to_string();
    let cstr = CString::from_str(&path).map_err(|_| KernelError::InvalidValue)?;
    let slice = cstr.as_bytes_with_nul();

    if slice.len() > len {
        return Err(KernelError::TooLarge);
    }

    copy_to_user_slice(slice, buf).await?;

    Ok(buf.value())
}

pub async fn sys_chdir(_path: TUA<c_char>) -> Result<usize, KernelError> {
    Ok(0)
}
