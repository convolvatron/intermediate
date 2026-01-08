use alloc::{boxed::Box, string::ToString, vec};
use crate::{Fd, AddressSpace, Task, Runtime};
use protocol::{Address, Attribute, Buffer, Command, attribute, Error, Value};

// partial writes?
// update pos

pub async fn sys_write<R:Runtime>(t: Task<R>, fd: Fd, user_buf: AddressSpace, count: usize) -> Result<usize, Error> {
    let file = t.process.get_fd(fd)?;
    if let AddressSpace::User(addr) = user_buf {
        let program = vec!(Command::copy(
            Address::Offset(
                Box::new(Address::Entity(t.process.myself, attribute!("vma")), addr)
            ),
            
            Address::Offset(
                Box::new(Address::Entity(file.obj, attribute!("contents"))),
                file.pos,
            ),
            count,
            Value::Variable(0),
        ));
        t.process.kernel.execute(program);
    }

    Ok(count)
}

pub async fn sys_read<R:Runtime>(t:Task<R>, fd: Fd, user_buf: AddressSpace, count: usize) -> Result<usize, Error> {
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
