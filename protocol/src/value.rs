use crate::{Buffer, Error, err};
use alloc::string::String;
use alloc::vec::Vec;

#[derive(PartialEq, Eq)]
pub struct Oid(pub u128);

pub type Variable = u32;

#[derive(PartialEq, Eq)]
pub struct Attribute(pub String);

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum Value {
    Oid(Oid) = 1,
    Utf8String(String)= 2,
    Bytes(Vec<u8>) = 3,    
    Unsigned(u64) = 4,
    Signed(i64) = 5,
    Variable(Variable) = 6, // the zero variable is the always matches never non-emtpy '_' of this world
    Empty() = 7, // used for deletia        
}

pub trait Encodable {
    fn encode(&self, dest:&mut Buffer);
    fn decode(dest:&mut Buffer) -> Result<Self, Error> where Self:Sized;
}

impl Encodable for Oid {
    fn encode(&self, dest:&mut Buffer) {
        dest.write(&self.0.to_be_bytes())
    }
    fn decode(source:&mut Buffer) -> Result<Self, Error> {
        // there is a compiler constant
        Ok(Oid(u128::from_be_bytes(source.read(16)?.try_into().expect("cast"))))
    }
}

impl Encodable for Variable {
    fn encode(&self, dest:&mut Buffer) {
        dest.write(&self.to_be_bytes())
    }
    fn decode(source:&mut Buffer) -> Result<Self, Error> {
        // there is a compiler constant
        Ok(u32::from_be_bytes(source.read(4)?.try_into().expect("cast")))
    }
}

impl Encodable for Attribute {
    fn encode(&self, dest:&mut Buffer) {
        dest.write_varint(self.0.len() as u64);
        dest.write(self.0.as_bytes());
    }
    fn decode(source:&mut Buffer) -> Result<Self, Error> {
        let len = source.read_varint()?;
        Ok(Attribute(String::from_utf8(source.read(len as usize)?.to_vec()).expect("cast")))
    }
}


impl Value {
    fn encode(&self, dest:Buffer) {
    }
    
    fn decode(source:Buffer) -> Result<Value, Error> {
        match source.read(1)?[0] {
            x if x == Value::Oid as u8 => {
                Ok(Value::Oid(Oid(1)))
            }
            
            x if x == Value::Utf8String as u8 => {
                Ok(Value::Oid(Oid(1)))                
            }
            x if x == Value::Bytes as u8 => {
                Ok(Value::Oid(Oid(1)))                                
            }
            x if x == Value::Unsigned as u8 => {
                Ok(Value::Oid(Oid(1)))                                
            }
            x if x == Value::Signed as u8 => {
                Ok(Value::Oid(Oid(1)))                                
            }
            x if x == Value::Variable as u8 => {
                Ok(Value::Oid(Oid(1)))                                                
            }
            x if x == Value::Empty as u8 => {
                Ok(Value::Oid(Oid(1)))                                                
            }
            x => Err(err!("invalid Value codepoint {}", x))
        }
    }
    
}
