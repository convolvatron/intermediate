#![no_std]
extern crate alloc;

pub mod attr;
pub mod creds;
pub mod dir;
pub mod fd_table;
pub mod ids;
pub mod kernel;
pub mod path;
pub mod pid;
pub mod process;
//pub mod wait;
pub mod rsrc_lim;
pub mod rw;
pub mod syserr;
pub mod task;
pub mod runtime;

pub use creds::*;
pub use fd_table::*;
pub use ids::*;
pub use kernel::*;
pub use pid::*;
pub use process::*;
pub use rsrc_lim::*;
pub use syserr::*;
pub use task::*;
pub use path::*;
pub use runtime::*;


#[macro_export]
macro_rules! perr {
    ($k:expr, $($arg:tt)*) => {{
        protocol::Error{cause:alloc::format!($($arg)*), location:Some($k.myself), syserr: None}
    }}
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct CharDevDescriptor {
    pub major: u64,
    pub minor: u64,
}

/// Standard POSIX file types. - fuse with the bitmasks
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    BlockDevice(CharDevDescriptor),
    CharDevice(CharDevDescriptor),
    Fifo,
    Socket,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct OpenFlags: u32 {
        const O_RDONLY    = 0b000;
        const O_WRONLY    = 0b001;
        const O_RDWR      = 0b010;
        const O_ACCMODE   = 0b011;
        const O_CREAT     = 0o100;
        const O_EXCL      = 0o200;
        const O_TRUNC     = 0o1000;
        const O_DIRECTORY = 0o200000;
        const O_APPEND    = 0o2000;
        const O_NONBLOCK  = 0o4000;
        const O_CLOEXEC   = 0o2000000;
    }
}

/// Specifies how to seek within a file, mirroring `std::io::SeekFrom`.
#[derive(Debug, Copy, Clone)]
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}
