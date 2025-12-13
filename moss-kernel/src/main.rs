#![no_std]
#![no_main]
#![feature(used_with_arg)]
#![feature(likely_unlikely)]

extern crate alloc;

mod arch;
mod inode;
pub mod general;
mod timespec;
mod interrupts;
mod kernel;
mod memory;
mod process;
mod sched;
mod sync;
mod error;
mod proc;
mod fs;

pub use general::*;
pub use sync::*;
pub use memory::*;
pub use error::*;
pub use fs::*;
pub use sched::*;

use crate::process::ctx::UserCtx;
use crate::uspc_ret::dispatch_userspace_task;

async fn launch_init() {
//    let dt = get_fdt();

    let task = current_task();

    // Ensure that the exec() call applies to init.
    assert!(task.process.tgid.is_init());

    // Now that the root fs has been mounted, set the real root inode as the
    // cwd.
//    *task.cwd.lock_save_irq() = (VFS.root_inode(), PathBuf::new());

//    process::exec::kernel_exec(inode, vec![init.as_str().to_string()], vec![])
//        .await
//        .expect("Could not launch init process");
}


pub fn kmain(ctx_frame: *mut UserCtx) {
    sched_init();

//    register_fs_drivers();

//    let kopts = parse_args(&args);

    spawn_kernel_work(launch_init());

    dispatch_userspace_task(ctx_frame)
}
