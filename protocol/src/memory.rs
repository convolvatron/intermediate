use crate::{Address, Attribute, Entity, Error, Value};
use alloc::vec::Vec;

struct Memory {
    attributes: alloc::collections::BTreeMap<Attribute, Value>,
}

impl Entity for Memory {
    fn keys(&self) -> alloc::vec::Vec<Attribute> {
        self.attributes.keys().map(|x| x.clone()).collect()
    }

    fn get(&self, _a: Attribute) -> Result<Value, Error> {
        Ok(Value::Unsigned(1))
    }

    fn set(&self, _s: Vec<(Attribute, Value)>) -> Result<(), Error> {
        Ok(())
    }

    fn copy(_source: Address, _dest: Address, _length: usize) -> Result<(), Error> {
        Ok(())
    }
}
