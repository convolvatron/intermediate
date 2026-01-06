use alloc::{boxed::Box, string::ToString, vec};
use crate::{Fd, UserAddress, Task};
use protocol::{Address, Attribute, Buffer, Command, attribute, Error};

// partial writes?
// update pos

pub async fn sys_write(t: Task, fd: Fd, user_buf: UserAddress, count: usize) -> Result<usize, Error> {
    let file = t.process.get_fd(fd)?;
    let program = vec!(Command::copy(
        
        Address::Offset(
            Box::new(Address::Entity(t.process.myself, Attribute("vma".to_string()))),
            user_buf as u64,
        ),
        
        Address::Offset(
            Box::new(Address::Entity(file.obj, Attribute("contents".to_string()))),
            file.pos,
        ),
        count,
        0,
    ));
    t.process.kernel.execute(program);
    Ok(count)
}

pub async fn sys_read(t:Task, fd: Fd, user_buf: UserAddress, count: usize) -> Result<usize, Error> {
    let file = t.process.get_fd(fd)?;

    // translate user_buf to 'physical'
    let mut b = Buffer::new();
    Command::copy(
        &mut b,
        Address::Offset(
            Box::new(Address::Entity(file.obj, attribute!("contents"))),
            file.pos,
        ),
        Address::Entity(t.process.myself, attribute!("vma")),
        count,
        0,
    );
    // update pos
    // partial reads?
    Ok(count)
}
