use alloc::{boxed::Box, string::ToString};
use protocol::Error;

use crate::{Fd, UserAddress, linuxerr};

use protocol::{Address, Attribute, Buffer, Command, attribute};

pub async fn sys_write(fd: Fd, _user_buf: UA, count: usize) -> Result<usize, Error> {
    let file = current_task()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(linuxerr!(EBADF))?;

    let mut b = Buffer::new();
    Command::copy(
        &mut b,
        Address::Entity(current_task().myself, Attribute("vma".to_string())),
        Address::Offset(
            Box::new(Address::Entity(file.obj, Attribute("contents".to_string()))),
            file.pos,
        ),
        count,
        0,
    );
    execute(b);
    // partial writes?
    // update pos
    Ok(count)
}

pub async fn sys_read(fd: Fd, user_buf: UA, count: usize) -> Result<usize, Error> {
    let file = current_task()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(linuxerr!(EBADFD))?;

    // translate user_buf to 'physical'
    let mut b = Buffer::new();
    Command::copy(
        &mut b,
        Address::Offset(
            Box::new(Address::Entity(file.obj, attribute!("contents"))),
            file.pos,
        ),
        Address::Entity(current_task().myself, attribute!("vma")),
        count,
        0,
    );
    // update pos
    // partial reads?
    Ok(count)
}
