use crate::{OpenFlags, perr, Process, Runtime, runtime::Lockable};
use protocol::{Error, Oid};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fd(pub i32);

pub const AT_FDCWD: i32 = -100; // Standard value for "current working directory"

impl Fd {
    pub fn as_raw(self) -> i32 {
        self.0
    }

    pub fn is_atcwd(self) -> bool {
        self.0 == AT_FDCWD
    }
}

impl From<u64> for Fd {
    // Conveience implemtnation for syscalls.
    fn from(value: u64) -> Self {
        Self(value.cast_signed() as _)
    }
}

bitflags::bitflags! {
    #[derive(Clone, Default)]
    pub struct FdFlags: u32 {
        /// The close-on-exec flag.
        const CLOEXEC = 1;
    }
}

#[derive(Clone)]
pub struct FileDescriptorEntry {
    pub obj: Oid,
    pub oflags: crate::OpenFlags,
    pub pos: u64,
    pub flags: FdFlags,
}

const MAX_FDS: usize = 8192;

impl<R:Runtime> Process<R> {
    /// Inserts a new file into the table, returning the new file descriptor.
    pub fn insert_fd(&mut self, obj: Oid, oflags: OpenFlags) -> Result<Fd, Error> {
        let fd = self.find_free_fd()?;

        let entry = FileDescriptorEntry {
            obj,
            oflags,
            pos: 0,
            flags: FdFlags::default(),
        };

        self.insert_at(fd, entry);

        Ok(fd)
    }

    // Insert the given etnry at the specified index. If there was an entry at
    // that index `Some(entry)` is returned. Otherwise, `None` is returned.
    fn insert_at(&mut self, fd: Fd, entry: FileDescriptorEntry) -> Option<FileDescriptorEntry> {
        let fd_idx = fd.0 as usize;

        if fd_idx >= self.fd_table.lock().len() {
            // We need to resize the vector to accommodate the new FD.
            self.fd_table.lock().resize_with(fd_idx + 1, || None);
        }

        self.fd_table[fd_idx].replace(entry)
    }

    /// Removes a file descriptor from the table, returning the file if it
    /// existed.
    pub fn remove_fd(&mut self, fd: Fd) {
        let fd_idx = fd.0 as usize;
        
        if let Some(entry) = self.fd_table.lock().get(fd_idx)
            && let Some(old_entry) = entry.take()
        {
            // Update the hint to speed up the next search.
            self.next_fd_hint = self.next_fd_hint.min(fd_idx);
        }
        
    }


    //    pub fn clone_for_exec(&self) -> Self {

    /// Finds the lowest-numbered available file descriptor.
    fn find_free_fd(&mut self) -> Result<Fd, Error> {
        // Start searching from our hint.
        for i in self.next_fd_hint..self.fd_table.len() {
            if self.fd_table[i].is_none() {
                self.next_fd_hint = i + 1;
                return Ok(Fd(i as i32));
            }
        }

        // We didn't find a free slot in the existing capacity
        let next = self.fd_table.len();

        if next >= MAX_FDS {
            // this should be in the process context
            Err(perr!(self.k, "too many files"))
        } else {
            self.next_fd_hint = next + 1;
            Ok(Fd(next as i32))
        }
    }
}
