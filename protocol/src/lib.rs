#![no_std]
#![allow(dead_code)]
extern crate alloc;
pub use alloc::{format, string::String, sync::Arc};

mod address;
mod buffer;
mod command;
mod memory;
mod value;

pub use address::*;
pub use buffer::*;
pub use command::*;
pub use value::*;
//pub use memory::*;

pub struct Error {
    location: Oid,
    cause: String,
    // file and line can we do?
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        crate::Error{cause:crate::format!($($arg)*), location:Oid(1)}
    }
}

type DynResolver = Arc<dyn Resolver>;
pub trait Resolver {
    fn resolve(&self, o: Oid) -> Result<DynEntity, Error>;
}

// this needs to be parameterized by instance
pub fn new_object() -> Oid {
    Oid(1)
}

type ChangeSet = alloc::vec::Vec<(Attribute, Value)>;

type DynEntity = Arc<dyn Entity>;
pub trait Entity {
    fn keys(&self) -> alloc::vec::Vec<Attribute>;
    fn get(&self, a: Attribute) -> Result<Value, Error>;
    fn set(&self, s: ChangeSet) -> Result<(), Error>;
    fn copy(source: Address, dest: Address, length: usize) -> Result<(), Error>
    where
        Self: Sized;
}

// there a whole .. allocation and routing thing that would have to be built. this is our
// silly placeholder

const TARGET: Oid = Oid(1000);
const SYSPROXY: Oid = Oid(1000);
const MONITOR: Oid = Oid(1000);
