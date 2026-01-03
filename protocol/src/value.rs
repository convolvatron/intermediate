use crate::{Buffer, Error, err, DynEntity};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug)]
pub struct Oid(pub u128);

pub type Variable = u32;

#[derive(PartialEq, Eq, Clone)]
pub struct Attribute(pub String);

#[derive(PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum Value {
    Oid(Oid) = 1,
    Utf8String(String) = 2,
    Bytes(Vec<u8>) = 3,
    Unsigned(u64) = 4,
    Signed(i64) = 5,
    Variable(Variable) = 6, // the zero variable is the always matches never non-emtpy '_' of this world
    Empty() = 7,            // used for deletia
}

pub trait Encodable {
    fn encode(&self, dest: &mut Buffer);
    fn decode(dest: &mut Buffer) -> Result<Self, Error>
    where
        Self: Sized;
}

impl Encodable for Oid {
    fn encode(&self, dest: &mut Buffer) {
        dest.write(&self.0.to_be_bytes())
    }
    fn decode(source: &mut Buffer) -> Result<Self, Error> {
        // there is a compiler constant
        Ok(Oid(u128::from_be_bytes(
            source.read(16)?.try_into().expect("cast"),
        )))
    }
}

impl Encodable for Variable {
    fn encode(&self, dest: &mut Buffer) {
        dest.write(&self.to_be_bytes())
    }
    fn decode(source: &mut Buffer) -> Result<Self, Error> {
        // there is a compiler constant
        Ok(u32::from_be_bytes(
            source.read(4)?.try_into().expect("cast"),
        ))
    }
}

impl Encodable for Attribute {
    fn encode(&self, dest: &mut Buffer) {
        dest.write_varint(self.0.len() as u64);
        dest.write(self.0.as_bytes());
    }
    fn decode(source: &mut Buffer) -> Result<Self, Error> {
        let len = source.read_varint()?;
        Ok(Attribute(
            String::from_utf8(source.read(len as usize)?.to_vec()).expect("cast"),
        ))
    }
}

// discrimimant issues
impl Encodable for Value {
    fn encode(&self, _dest: &mut Buffer) {}

    fn decode(source: &mut Buffer) -> Result<Value, Error> {
        match source.read(1)?[0] {
            x if x == 1 => Ok(Value::Oid(Oid(1))),
            x if x == 2 => Ok(Value::Oid(Oid(1))),
            x if x == 3 => Ok(Value::Oid(Oid(1))),
            x if x == 4 => Ok(Value::Oid(Oid(1))),
            x if x == 5 => Ok(Value::Oid(Oid(1))),
            x if x == 6 => Ok(Value::Oid(Oid(1))),
            x if x == 7 => Ok(Value::Oid(Oid(1))),
            x => Err(err!(Oid(1), "invalid Value codepoint {}", x)),
        }
    }
}

// oid or dyn?
pub fn get_u64(_e:DynEntity, _a:Attribute) -> Result<u64, Error>{
    Ok(1)
}

pub fn get_string(_e:DynEntity, _a:Attribute) -> Result<String, Error>{
    Ok("just kidding".to_string())
}
