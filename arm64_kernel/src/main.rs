#![no_std]
#![no_main]
#![feature(used_with_arg)]
#![feature(likely_unlikely)]

extern crate alloc;

mod arch;
pub mod general;
//mod timespec;
mod interrupts;
mod memory;
mod sync;
mod ctx;

pub use general::*;
pub use sync::*;
pub use memory::*;
pub use ctx::*;

pub use protocol::{Error, err, Buffer};
pub use alloc::format;


pub type ProcVM = ProcessVM<<ArchImpl as VirtualMemory>::ProcessAddressSpace>;


#[macro_export]
macro_rules! out_of_memory {
    () => {{
        use protocol::Oid;
        crate::Error{cause:"out of memory".to_string(), location:Oid(1), syserr: None}
    }}
}

/// Represents a fixed point in monotonic time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instant {
    ticks: u64,
    freq: u64,
}

#[macro_export]
macro_rules! console {
    ($($msg:tt)*) => {
        crate::arch::arm64::boot::console_output(format_args!($($msg)*).as_str().unwrap())
    }
}

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

// find a better home
/// Returns the current instant, if the system timer has been initialised.
/*
pub fn now() -> Instant {
    SYS_TIMER.get().map(|timer| timer.driver.now())
}
 */

// sysproxy object:
//   filesystem root
//   executable oid
pub fn kmain(ctx_frame: *mut UserCtx) {
    //    spawn_kernel_work(launch_init());
    //    dispatch_userspace_task(ctx_frame)
    loop{}
}

pub fn execute(b: Buffer) -> Result<(), Error> {
    Ok(())
    // hsvc!
}
