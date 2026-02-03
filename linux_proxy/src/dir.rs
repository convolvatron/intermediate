use alloc::{vec};
use crate::{Fd, FileType, AddressSpace, linuxerr, Task, Runtime};
use protocol::{Buffer,
               Command,
               Value,
               Error,
               Attribute,
               attribute};

// merge with file type
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum DirentFileType {
    _Unknown = 0,
    Fifo = 1,
    Char = 2,
    Dir = 4,
    Block = 6,
    Reg = 8,
    Link = 10,
    Socket = 12,
    _Wht = 14,
}

impl From<FileType> for DirentFileType {
    fn from(value: FileType) -> Self {
        match value {
            FileType::File => Self::Reg,
            FileType::Directory => Self::Dir,
            FileType::Symlink => Self::Link,
            FileType::BlockDevice(_) => Self::Block,
            FileType::CharDevice(_) => Self::Char,
            FileType::Fifo => Self::Fifo,
            FileType::Socket => Self::Socket,
        }
    }
}

/// The header of a `linux_dirent64` struct.
///
/// This must match the C layout precisely. `#[repr(packed)]` is essential to
/// remove Rust's default padding and make `size_of` report the correct unpadded
/// header size (19 bytes).
/// not sure this is carrying any weight . total length is 19 which pads to 24
#[repr(C, packed)]
struct Dirent64Hdr {
    _ino: u64,
    _off: u64,
    _reclen: u16,
    _kind: DirentFileType,
}

fn pad(x: usize, to: usize) -> usize {
    (((x - 1) / to) + 1) * to
}

macro_rules! write {
    ($dest:expr, $source:expr) => {
        $dest.write($source).map_err(|_| linuxerr!(EINVAL))
    }
}

macro_rules! var {
    ($num:expr) => {
        Value::Variable($num)
    }
}

macro_rules! union {
    ($num:expr) => {
        Value::Union($num)
    }
}

// i would like to check if it has a contents, but not
// to pull the whole thing. we also need to demux special files and links

pub async fn sys_getdents64<R:Runtime>(t: Task<R>, fd: Fd, mut ubuf: AddressSpace, size: u32) -> Result<usize, Error> {
    let file = t.process.get_fd(fd)?;
    let b = Buffer::new();
    let st = t.process.kernel.runtime.execute(
        vec!(Command::Get(file.obj, attribute!("children"), var!(0), union!(4)),
             Command::Get(var!(0), var!(1), var!(2), union!(4)),
             Command::Get(var!(2), attribute!("children"), union!(3), union!(4))))?;

    while let Some(v) = st.next() {
        if let Value::String(name) = v[1] && 
            let Value::Oid(oid) = v[2] &&
            let Value::Set(children) = v[3] {
                let header_len = core::mem::size_of::<Dirent64Hdr>();
                let reclen : u16 = header_len + name.len() + 1;
                write!(b, &oid.to_le_64())?;
                write!(b, &(pad(reclen as u64, 8).to_le_u64()))?;
                write!(b, &reclen.to_le_64())?;
                let kind = if children.len() > 0 {
                    &[DirentFileType::Dir as u8]
                } else {
                    &[DirentFileType::Reg as u8]
                };
                write!(b, kind)?;
                write!(b, &name.into_bytes())?;
            }
    }
    Ok(b.len())
}
