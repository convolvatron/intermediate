//! The architectural abstraction layer.
//!
//! This module defines the `Arch` trait, which encapsulates all
//! architecture-specific functionality required by the kernel. To port the
//! kernel to a new architecture, a new submodule must be created here which
//! implements this trait.
//!
//! The rest of the kernel should use the `ArchImpl` type alias to access
//! architecture-specific functions and types.

use crate::{
    CpuOps, VirtualMemory,
    memory::address::{VA},
};

pub trait Arch: CpuOps + VirtualMemory {
    /// The type representing the state saved to the stack on an exception or
    /// context switch. The kernel's scheduler and exception handlers will work
    /// with this type.
    type UserContext: Sized + Send + Sync + Clone;

    fn name() -> &'static str;

    /// Prepares the initial context for a new user-space thread. This sets up
    /// the stack frame so that when we context-switch to it, it will begin
    /// execution at the specified `entry_point`.
    fn new_user_context(entry_point: VA, stack_top: VA) -> Self::UserContext;
}

#[cfg(target_arch = "aarch64")]
pub mod arm64;

#[cfg(target_arch = "aarch64")]
pub use self::arm64::Aarch64 as ArchImpl;
