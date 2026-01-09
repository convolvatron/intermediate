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
pub enum AddressSpace {
    User(u64),
    Kernel(u64),
    Physical(u64),
}

pub trait Runtime {
    type Lock<A>:Lockable<A>;

    type Thread:Runnable;

    fn create_thread(&self, entry:u64, arg:u64, syscalls:DynSyscall) -> Result<Self::Thread, Error>;
    fn create_lock<A>(&self) -> Self::Lock<A>;
    fn console(&self, s:String);

    // permissions..think about address spaces a little more
    // return a mapping with an unmap?
    fn map(&self, from:AddressSpace, to:AddressSpace, length:usize);
    fn unmap(&self, at:AddressSpace, length:usize);
    fn copy(&self, to:AddressSpace, from:AddressSpace, length:usize);
    fn execute(&self, block:Vec<Command>) -> Result<Vec<Value>, Error>;
}
