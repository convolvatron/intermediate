#![no_std]
#![allow(dead_code)]
extern crate alloc;
pub use alloc::{
    sync::Arc,
    string::String,
    format,
};

mod address;
mod command;
mod buffer;
mod value;
mod memory;

pub use address::*;
pub use command::*;
pub use value::*;
pub use buffer::*;
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
    fn resolve(&self, o:Oid) -> Result<DynEntity, Error>;
}

// this needs to be parameterized by instance
pub fn new_object() -> Oid {
    Oid(1)
}

type ChangeSet = alloc::vec::Vec<(Attribute, Value)>;

type DynEntity = Arc<dyn Entity>;
pub trait Entity {
    fn keys(&self) -> alloc::vec::Vec<Attribute>;
    fn get(&self, a:Attribute) -> Result<Value, Error>;
    fn set(&self, s:ChangeSet) ->Result<(), Error>;
    fn copy(source:Address, dest:Address, length :usize) -> Result<(), Error> where Self: Sized;
}
