use crate::{
    //kernel::kpipe::KPipe,
    linux::Fd,
    error::KernelError,
    memory::address::{TUA},

};
use alloc::{boxed::Box, sync::Arc};
use async_trait::async_trait;
use core::{future, pin::pin, task::Poll};

pub async fn sys_pipe2(fds: TUA<[Fd; 2]>, flags: u32) -> Result<usize, KernelError> {
    Ok(0)
}
