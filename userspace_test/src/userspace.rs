use std::sync::Mutex;

struct Thread {
}

struct Userspace  {
}


impl protocol::Runtime for Userspace {
    type Lock<A> = Mutex<A>;
    type Thread:Runnable;

    fn create_thread(&self, entry:u64, arg:u64, syscalls:DynSyscall) -> Result<Self::Thread, Error>{
    }
    
    fn console(&self, s:String){
        print!(s);
    }

    // permissions..think about address spaces a little more
    // return a mapping with an unmap?
    fn map(&self, from:AddressSpace, to:AddressSpace, length:usize) {
    }
    fn unmap(&self, at:AddressSpace, length:usize) {
    }
    fn copy(&self, to:AddressSpace, from:AddressSpace, length:usize){
    }
    
    fn execute(&self, block:Vec<Command>) -> Result<Vec<Value>, Error>{
    }
}
