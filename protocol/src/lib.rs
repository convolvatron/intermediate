#![no_std]
#![allow(dead_code)]
extern crate alloc;
pub use alloc::{boxed::Box, format, string::String, sync::Arc, collections::BTreeMap, vec::Vec};
use core::sync::atomic::{AtomicU64, Ordering};
use async_trait::async_trait;

mod buffer;
mod command;
mod error;
mod memory;
mod value;
pub mod interpreter;

pub use buffer::*;
pub use command::*;
pub use error::*;
pub use value::*;
pub use memory::*;
pub use interpreter::*;

#[macro_export]
macro_rules! attribute {
    ($sattr:expr) => {{
        use alloc::string::ToString;
        Attribute($sattr.to_string())
    }};
}

type DynResolver = Arc<dyn Resolver + Sync + Send>;
pub trait Resolver {
    fn resolve(&self, v: Oid) -> Option<DynEntityHandler>;
}

// this needs to be parameterized by instance
pub fn new_object() -> Oid {
    Oid(1)
}

type ChangeSet = alloc::vec::Vec<(Attribute, Value)>;

// we may need to add a method to sort of the target of a copy operation (in or out)
pub type DynEntityHandler = Arc<dyn EntityHandler>;
pub trait EntityHandler  {
    fn keys(&self) -> DynStream<Attribute>;
    fn get(&self, a: Attribute) -> Result<Option<Value>, Error>;
    fn commit(&self, s: Vec<Command>) -> Result<(), Error>;
    fn copyout(&self,
               source_attribute:Attribute,
               source_offset:usize,
               dest:&[u8]) -> Result<(), Error>;    
}


struct Bindings {
    b:BTreeMap<Variable, Value>,
}

impl Bindings {
    fn get(&self, key:Value) -> Option<Value> {
        match key {
            Value::Variable(k) => self.b.get(&k).map(|x|x.clone()),
            _ => Some(key)
        }
    }
    
    fn assert(&mut self, key:Value, value:Value) -> bool {
        if let Value::Variable(v) = key {
            match self.get(key) {
                Some(x) => x == value,
                None => {
                    self.b.insert(v, value);
                    true
                }
            }
        } else {
            key == value
        }
    }
}

#[macro_export]
macro_rules! read_stream_with_err {
    ($stream:expr, $bindings:ident, $body:expr) => {{
        let x = $stream.next().await;
        match x {
            Err(e) => x,
            Ok(None) => x,
            Ok(Some($bindings)) => $body,            
        }
    }}
}

// I would have like result to be inside option since it belongs in the
// contained and not the container, but that eliminates our ability
// to use '?', which is the only thing really keeping going here. I hate
// the asymmetry of <A> and the return type of next
pub type DynStream<A> = Arc<dyn Stream<A> + Send + Sync>;
#[async_trait]
pub trait Stream<A> {
    async fn next(&mut self) -> Result<Option<A>, Error>;
}


pub type DynAllocator = Arc<dyn Allocator + Send + Sync>;
trait Allocator {
    fn new(&self) -> Oid;
}

struct SimpleAllocator {
    base:Oid,
    count: AtomicU64,
}

impl SimpleAllocator {
    fn new(base:Oid) -> DynAllocator {
        Arc::new(SimpleAllocator{base, count:AtomicU64::new(0)})
    }
}

impl Allocator for SimpleAllocator {
    // we're going to pass allocation errors to the runtime for treatment (down instead of up) 
    fn new(&self) -> Oid {
        let offset = self.count.fetch_add(1, Ordering::Relaxed);
        Oid(self.base.0 + offset as u128)
    }
}

