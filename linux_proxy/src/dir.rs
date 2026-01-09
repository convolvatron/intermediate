use crate::{Fd, FileType, AddressSpace, linuxerr, Task, Runtime, Lockable,
};
use protocol::{Buffer,
               DynEntity,
               Error,
               Attribute,
               attribute,
               get_string,
               get_u64};

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

fn write_dirent(dirent: DynEntity, dest: Buffer) -> Result<usize, Error> {
    let name = get_string(dirent, attribute!("name"))?;
    let header_len = core::mem::size_of::<Dirent64Hdr>();

    // Userspace expects dirents to always be 8-byte aligned.
    // are we not allowed to cheat on the last entry?
    let padded_reclen = pad(header_len + name.len() + 1, 8);

    // If the full, padded entry doesn't fit, stop here for this syscall.
    if padded_reclen > dest.len() {
        // isn't this nonmem..nope, someone decided it was the catchall
        return Err(linuxerr!(EINVAL));
    }

    // le should be parameterizable, but i guess that battle was lost 30 years ago
    get_u64(dirent, attribute!("inode"))?.to_le_bytes();
    dest[header_len..name.len()].copy_from_slice(name);
    Ok(padded_reclen)
}

pub async fn sys_getdents64<R:Runtime>(t: Task<R>, fd: Fd, mut ubuf: AddressSpace, size: u32) -> Result<usize, Error> {
    let file = t
        .process
        .fd_table
        .lock()
        .get(fd.0 as usize)
        .ok_or(linuxerr!(EBADF))?;
    let b = Buffer::new();
    let st = get_stream(file.obj, attribute!("children"));
    while let Some(t) = st.next() {
        write_dirent(t, b);
    }

    Ok(0)
}
