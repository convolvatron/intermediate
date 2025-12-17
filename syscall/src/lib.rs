#![no_std]

pub type Oid = u128;
// is this an oid? a string? a enumeration?
pub type Property = Value;

enum Value {
    Oid(Oid),
    String(String),    
}

pub enum Operation  {
    Copy = 1,
    Resolve = 2,
//    open = 3,
}

pub enum Address {
    Virtual,
    Object,
    Instance,
    BitRange,
    Locale,
}

pub struct Buffer {
}

pub fn new_object() -> Oid {
}

struct Object {};
impl Object {
    pub fn keys() -> Iterator<Property> {
    }
    pub fn get(p:Property) -> Value {
    }    
}





