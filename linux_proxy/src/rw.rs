use alloc::{boxed::Box, vec, vec::Vec};
use crate::{Fd, AddressSpace, Task, Runtime, linuxerr};
use protocol::{Address, Attribute, Command, attribute, Error};

// partial writes?
// update pos

pub async fn sys_write<R:Runtime>(t: Task<R>, fd: Fd, user_buf: AddressSpace, count: usize) -> Result<usize, Error> {
    let file = t.process.get_fd(fd)?;
    if let AddressSpace::User(addr) = user_buf {
        let program = vec!(Command::Copy(
            Address::Offset(
                Box::new(Address::Entity(t.process.myself, attribute!("vma"))),
                addr),
            Address::Offset(
                Box::new(Address::Entity(file.obj, attribute!("contents"))),
                file.pos,
            ),
            count,
            0
        ));
        t.process.kernel.runtime.execute(program);
    }

    Ok(count)
}

pub async fn sys_read<R:Runtime>(t:Task<R>, fd: Fd, user_buf: AddressSpace, count: usize) -> Result<usize, Error> {
    let file = t.process.get_fd(fd)?;

    // translate user_buf to 'physical'
    let mut block = Vec::new();
    if let AddressSpace::User(addr) = user_buf {
        block.push(Command::Copy(
            Address::Offset(
                Box::new(Address::Entity(file.obj, attribute!("contents"))),
                file.pos),
            Address::Offset(
                Box::new(Address::Entity(t.process.myself, attribute!("vma"))),
                addr as u64),
            count,
            0,
        ));
        // update pos
        // partial reads?
        Ok(count)
    } else {
        Err(linuxerr!(EFAULT))
    }
}
