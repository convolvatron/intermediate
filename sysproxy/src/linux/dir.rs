use crate::{
    error::{KernelError},
    linux::{FileType},
    memory::address::UA,
    linux::Fd,
    current_task,
};
use alloc::{ffi::CString, string::String, layout::Layout};
use protocol::{Buffer, linuxerr, DynEntity, Error};

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

fn pad(x:usize, to:usize) {
    (((x-1)/to)+1)*to;
}

fn write_dirent(dirent: DynEntity, dest:&[u8]) -> Result<usize, Error> {
    let name = get_string(dirent, "name");
    let header_len = core::mem::size_of::<Dirent64Hdr>();
    let unpadded_len = header_len + name.len();
    
    // Userspace expects dirents to always be 8-byte aligned.
    let padded_reclen = pad(unpadded_len, 8);
    
    // If the full, padded entry doesn't fit, stop here for this syscall.
    if padded_reclen > (size as usize).saturating_sub(bytes_written) {
        return linuxerr!(LinuxError::EINVAL);        
    }        

    // le should be parameterizable, but i guess that battle was lost 30 years ago
    get_u64(dirent, "inode")?.to_le_bytes();
    kernel_entry_buf[header_len..unpadded_len].copy_from_slice(name);
    let entry_slice = &kernel_entry_buf[..padded_reclen];
    // do this incrementally
    copy_to_user_slice(entry_slice, ubuf).await?;
    
    ubuf = ubuf.add_bytes(padded_reclen);
    bytes_written += padded_reclen;
}

pub async fn sys_getdents64(fd: Fd, mut ubuf: UA, size: u32) -> Result<usize, Error> {
    let task = current_task();
    let file = task
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;
    let b = Buffer::new();
    Ok(0)
}
