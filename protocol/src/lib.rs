#![no_std]
extern crate alloc;

mod address;

use address::*;

pub type Attribute = alloc::string::String;
pub type Oid = u128;
// is this an oid? a string? a enumeration?
pub type Property = Value;

struct Error {}

enum Value {
    Oid(Oid) = 1,
    String(alloc::string::String),    
}

pub struct Buffer {
}

pub fn new_object() -> Oid {
}

type Changeset = alloc::vec::Vec<(Attribute, Value)>;

// keys?
pub trait Entity {
    // we would like to use an iterator, but there are some fraught lifetime issues
    // for the moment we assume that this set is small
    fn keys(&self) -> alloc::vec::Vec<Attribute>;
    fn get(&self, a:Attribute) -> Result<Value, Error>;
    fn set(&self, s:Changeset) ->Result<(), Error>;
    fn copy(source:Address, dest:Address, length :usize) -> Result<(), Error>;
    fn create() -> Result<(), Error>;
}


// something a little more catchy?
struct MemoryEntity {
    attributes: alloc::collections::BTreeMap<alloc::string::String, Value>
}

impl Entity for MemoryEntity {
    fn keys() -> Iterator<Property> {
    }
    fn get(a:Attribute) -> Value {
    }
    
    fn set(s:ChangeSet) {
    }
    
}


struct Triple {
    entity:Oid,
    attribute:Attribute,
    value:Value,         
}

impl Triple {
    fn new(e:Oid, a:Attribute, v:Value) -> Triple {
    }
}
