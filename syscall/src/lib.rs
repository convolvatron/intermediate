#![no_std]

pub type Oid = u128;
    
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

pub enum Value {
    Address,
    String,
    Unsigned,
    Signed,
}

pub fn new_object() {
}




