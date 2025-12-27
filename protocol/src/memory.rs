use crate::{Entity, Attribute, Value, Error, Address};
use alloc::vec::Vec;

struct Memory {
    attributes: alloc::collections::BTreeMap<alloc::string::String, Value>
}

impl Entity for Memory {
    fn keys(&self) -> alloc::vec::Vec<Attribute> {
        self.keys()
    }
    
    fn get(&self, _a:Attribute) -> Result<Value, Error> {
        Ok(Value::Unsigned(1))
    }
    
    fn set(&self, _s:Vec<(Attribute, Value)>) ->Result<(), Error> {
        Ok(())
    }
    
    fn copy(_source:Address, _dest:Address, _length :usize) -> Result<(), Error> {
        Ok(())
    }
}
