use async_trait::async_trait;
use crate::error::KernelError;
use crate::fs::attr::FileAttr;
use alloc::boxed::Box;
use alloc::sync::Arc;
use crate::fs::DirStream;
use crate::error::FsError;
use crate::fs::FileType;

/// A stateless representation of a filesystem object.
///
/// This trait represents an object on the disk (a file, a directory, etc.). All
/// operations are stateless from the VFS's perspective; for instance, `read_at`
/// takes an explicit offset instead of using a hidden cursor.
#[async_trait]
pub trait Inode: Send + Sync {
    /// Get the unique ID for this inode.
//    fn id(&self) -> InodeId;

    /// Reads data from the inode at a specific `offset`.
    /// Returns the number of bytes read.
    async fn read_at(&self, _offset: u64, _buf: &mut [u8]) -> Result<usize, KernelError> {
        Err(KernelError::NotSupported)
    }

    /// Writes data to the inode at a specific `offset`.
    /// Returns the number of bytes written.
    async fn write_at(&self, _offset: u64, _buf: &[u8]) -> Result<usize, KernelError> {
        Err(KernelError::NotSupported)
    }

    /// Truncates the inode to a specific `size`.
    async fn truncate(&self, _size: u64) -> Result<(), KernelError> {
        Err(KernelError::NotSupported)
    }

    /// Gets the metadata for this inode.
    async fn getattr(&self) -> Result<FileAttr, KernelError> {
        Err(KernelError::NotSupported)
    }

    /// Looks up a name within a directory, returning the corresponding inode.
    async fn lookup(&self, _name: &str) -> Result<Arc<dyn Inode>, KernelError> {
        Err(KernelError::NotSupported)
    }

    /// Creates a new object within a directory.
    async fn create(
        &self,
        _name: &str,
        _file_type: FileType,
        _permissions: u16,
    ) -> Result<Arc<dyn Inode>, KernelError> {
        Err(KernelError::NotSupported)
    }

    /// Removes a link to an inode from a directory.
    async fn unlink(&self, _name: &str) -> Result<(), KernelError> {
        Err(KernelError::NotSupported)
    }

    /// Reads the contents of a directory.
    async fn readdir(&self, _start_offset: u64) -> Result<Box<dyn DirStream>, KernelError> {
        Err(FsError::NotADirectory.into())
    }
}
