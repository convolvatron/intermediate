use alloc::{string::ToString, boxed::Box};

use crate::{
    Error,
    execute,
    linux::Fd,
    current_task,
    memory::address::UA,
};

use protocol::{Buffer, Command, Address, Attribute, attr, linuxerr};

pub async fn sys_write(fd: Fd, user_buf: UA, count: usize) -> Result<usize, Error> {
    let file = current_task()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(linuxerr!(LinuxError::BADFD))?;

    let mut b = Buffer::new();
    Command::copy(&mut b,
                  Address::Entity(current_task().myself, Attribute("vma".to_string())),
                  Address::Offset(Box::new(Address::Entity(file.obj, Attribute("contents".to_string()))), file.pos),
                  count,
                  0);
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
        .ok_or(linuxerr!(LinuxError::BADFD))?;

    let mut b = Buffer::new();
    Command::copy(&mut b,
                  Address::Offset(Box::new(Address::Entity(file.obj, attr!("contents"))), file.pos),
                  Address::Entity(current_task().myself, attr!("vma")),
                  count,
                  0);
    // update pos
    // partial reads?    
    Ok(count)
}
