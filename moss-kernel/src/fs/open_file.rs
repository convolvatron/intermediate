use crate::{
    fops::FileOps,    
//    process::fd_table::select::PollFlags,
    sync::{AsyncMutexGuard, Mutex},
    fs::{Inode, OpenFlags},    
};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::{future};

pub struct FileCtx {
    pub flags: OpenFlags,
    pub pos: u64,
}

impl FileCtx {
    pub fn new(flags: OpenFlags) -> Self {
        Self { flags, pos: 0 }
    }
}

pub struct OpenFile {
    inode: Option<Arc<dyn Inode>>,
    state: Mutex<(Box<dyn FileOps>, FileCtx)>,
}

impl OpenFile {
    pub fn new(ops: Box<dyn FileOps>, flags: OpenFlags) -> Self {
        Self {
            state: Mutex::new((ops, FileCtx::new(flags))),
            inode: None,
        }
    }

    pub fn set_inode(&mut self, inode: Arc<dyn Inode>) {
        self.inode = Some(inode)
    }

    pub fn inode(&self) -> Option<Arc<dyn Inode>> {
        self.inode.clone()
    }

    pub async fn flags(&self) -> OpenFlags {
        self.state.lock().await.1.flags
    }

    pub async fn lock(&self) -> AsyncMutexGuard<'_, (Box<dyn FileOps>, FileCtx)> {
        self.state.lock().await
    }
/*
    pub async fn poll(
        &self,
        flags: PollFlags,
    ) -> impl Future<Output = Result<PollFlags, KernelError>> + Send + use<> {
        let mut futs = Vec::new();

        {

        }

        future::poll_fn(move |cx| {
})
}
    */
}
