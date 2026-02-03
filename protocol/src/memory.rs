use crate::{ Attribute,
 Error,
 Value,
 EntityHandler,
 DynStream,
 Stream,
 locerr,
 Command,
 Scope};
use alloc::{vec::Vec, collections::BTreeMap, boxed::Box, sync::Arc, vec};
use async_trait::async_trait;

// we keep a separate set of keys so that we can do async iteration without plumbing a runtime or a lifetime
// through the entire codebase. maybe there is a better answer? its not clear
#[derive(Clone)]
pub struct Memory {
    attributes: Vec<Attribute>,
    values: BTreeMap<Attribute, Value>,
}

impl Memory {
    pub fn new()->Self {
        Memory{
            attributes:Vec::new(),
            values:BTreeMap::new(),
        }
    }
    
    fn copyin(&self,
              scope: Scope,
              dest_attribute:Attribute, 
              dest_offset: usize,
              source: &[u8]) -> Result<(), Error> {
        if let Some(x) = self.get(dest_attribute)? {
            if let Value::Bytes(v) = x {
                let end = dest_offset + source.len();
                let first_end = core::cmp::min(end, v.len() as usize);
                v.as_slice()[dest_offset..].copy_from_slice(&source[..first_end]);
                if first_end < end {
                    v.extend(&source[first_end..end])                    
                }
                Ok(())
            } else {
                Err(locerr!(scope.myself, "attempt to copy into a non-byte value"))
            }
        } else {
            let mut out = vec![0; dest_offset];
            out.extend_from_slice(source);
            scope.write_commands.push(Command::Set(self.myself, dest_attribute, Value::Bytes(out)));
            Ok(())
        }
    }
}

pub struct KeysIter {
    m: Memory,
    index: usize,
}

#[async_trait]
impl Stream<Attribute> for KeysIter {
    async fn next(&mut self) -> Result<Option<Attribute>, Error> {
        if self.index < self.m.values.len() {
            Ok(Some(self.m.attributes[self.index].clone()))
        } else  {
            Ok(None)
        }
    }
}


impl<'a> EntityHandler for Memory {
    fn keys(&self) -> DynStream<Attribute>  {
        Arc::new(KeysIter{index: 0, m:self.clone()})
    }

    fn get(&self, a: Attribute) -> Result<Option<Value>, Error> {
        if let Some(x) = self.values.get(&a) {
            Ok(Some(x.clone()))
        } else {
            Ok(None)
        }
    }

    fn commit(&self, _s: Vec<Command>) -> Result<(), Error> {
        Ok(())
    }

    // at some point we were _determined_ that all reads everywhere should self-allocate and return
    // a pointer. i guess we're still being constrained by read()
    fn copyout(&self,
               source_attribute:Attribute,
               source_offset:usize,
               dest:&[u8]) -> Result<(), Error> {
        if let Some(x) = self.get(source_attribute)? {
            if let Value::Bytes(v) = x {
                dest.copy_from_slice(v.as_slice()[source_offset..source_offset+dest.len()]);
                Ok(())
            } else {
                Err(locerr!(self.myself, "attempt to copy from a non-byte value"))
            }
        }
    }   
}
