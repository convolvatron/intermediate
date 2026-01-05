#![no_std]
#![allow(dead_code)]
extern crate alloc;
pub use alloc::{format, string::String, sync::Arc, boxed::Box};
use async_trait::async_trait;

mod address;
mod buffer;
mod command;
mod memory;
mod value;
mod error;

pub use address::*;
pub use buffer::*;
pub use command::*;
pub use value::*;
pub use error::*;
//pub use memory::*;

#[macro_export]
macro_rules! attribute {
    ($sattr:expr) => {
        Attribute($sattr.to_string())
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

pub type DynEntity = Arc<dyn Entity>;
pub trait Entity {
    fn keys(&self) -> alloc::vec::Vec<Attribute>;
    fn get(&self, a: Attribute) -> Result<Value, Error>;
    fn set(&self, s: ChangeSet) -> Result<(), Error>;
    fn copy(source: Address, dest: Address, length: usize) -> Result<(), Error>
    where
        Self: Sized;
}

// there a whole .. allocation and routing thing that would have to be built. this is our
// silly placeholder. 
const TARGET: Oid = Oid(0x10000000000000000000000000000000);
const SYSPROXY: Oid = Oid(0x20000000000000000000000000000000);
const MONITOR: Oid = Oid(0x30000000000000000000000000000000);


pub type DynStream<A> = Arc<dyn Stream<A>>;
#[async_trait]
pub trait Stream<A> {
    async fn next(&self) -> Option<A>;
}

