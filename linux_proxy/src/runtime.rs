use alloc::{sync::Arc, string::String, vec::Vec};
use protocol::{Error, Command, Value};
use core::ops::{Deref, DerefMut};


type DynSyscall = Arc<dyn Syscall>;
pub trait Syscall {
    fn on_syscall(&self, num:u32, args:[u64;6]);
}

pub trait Lockable<A> {
    type Lock<C>;
    type Guard<'a, B:'a> : Deref<Target = &'a B> + DerefMut<Target = &'a mut B> where Self: 'a;
    fn new(item:A) -> Self::Lock<A>;
    fn lock(&self) ->  Self::Guard<'_, A>;
}

pub trait Runnable {
    fn run(&self) -> !; // this will have to change once we support multiple contexts
}

// AddressSpace looks alot like protocol::Address?
pub enum AddressSpace {
    User(u64),
    Kernel(u64),
    Physical(u64),
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct AccessMode: i32 {
        /// Execution is permitted
        const X_OK = 1;
        /// Writing is permitted
        const W_OK = 2;
        /// Reading is permitted
        const R_OK = 4;
    }
}


pub trait Runtime {
    type Lock<A>:Lockable<A>;
    type Thread:Runnable;

    fn create_thread(&self, entry:u64, arg:u64, syscalls:DynSyscall) -> Result<Self::Thread, Error>;
    fn console(&self, s:String);
    fn map(&self, from:AddressSpace, to:AddressSpace, a:AccessMode, length:usize) -> Result<(), Error>;
    fn unmap(&self, at:AddressSpace, length:usize) -> Result<(), Error>;
    fn copy(&self, to:AddressSpace, from:AddressSpace, length:usize);
    fn execute(&self, block:Vec<Command>) -> Result<Vec<Value>, Error>;
}
