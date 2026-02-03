use crate::{Buffer, DynEntityHandler, DynStream, Error, err, Encodable};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug)]
pub struct Oid(pub u128);

pub type Variable = u32;
pub type Attribute = Value; 
pub type Entity = Value; // really {oid, variable}


#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
#[repr(u8)]
pub enum Value {
    Oid(Oid) = 1,
    Utf8String(String) = 2,
    Bytes(Vec<u8>) = 3,
    Unsigned(u64) = 4,
    Signed(i64) = 5,
    Variable(Variable) = 6, 
    Empty() = 7,            // used for deletia
    Union(Variable) = 8, 
}

impl Encodable for Oid {
    fn encode(&self, dest: &mut Buffer)-> Result<(), Error> {
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
    fn encode(&self, dest: &mut Buffer) -> Result<(), Error> {
        dest.write(&self.to_be_bytes())
    }
    fn decode(source: &mut Buffer) -> Result<Self, Error> {
        // there is a compiler constant
        Ok(u32::from_be_bytes(
            source.read(4)?.try_into().expect("cast"),
        ))
    }
}

// discrimimant issues
// since the discriminant is so small we could use the rest of those bits for something
impl Encodable for Value {
    fn encode(&self, dest: &mut Buffer) -> Result<(), Error> {
        match self {
            Value::Oid(oid) => {
                dest.write(&[1])?;
            }
            Value::Utf8String(string) => {
                dest.write(&[2])?;
            }
            Value::Bytes(v) => {
                dest.write(&[3])?;
                dest.write_varint(v.len() as u64)?;
                dest.write(v)?;                
            }
           Value::Unsigned(u) => {
                dest.write(&[4])?;
                dest.write_varint(*u)?;                
            }
            Value::Signed(i) => {
                panic!("no signed");
            }
            Value::Variable(v) => {
                dest.write(&[6])?;
                dest.write_varint(*v as u64)?;
            }
            Value::Empty() => {
                dest.write(&[7])?;
            }
            Value::Union(v) => {
                dest.write(&[8])?;
                dest.write_varint(*v as u64)?;                
            }
        }
        Ok(())
    }

    fn decode(source: &mut Buffer) -> Result<Value, Error> {
        match source.read(1)?[0] {
            x if x == 1 => Ok(Value::Oid(Oid::decode(source)?)),
            x if x == 2 => {
                let length = source.read_varint()?;
                let body = source.read(length as usize)?;
                let string = String::from_utf8(body.to_vec()).map_err(|_|err!("invalid utf8 contents"))?;
                Ok(Value::Utf8String(string))
            }
            x if x == 3 => {
                let length = source.read_varint()?;
                Ok(Value::Bytes(source.read(length as usize)?.to_vec()))
            }
            x if x == 4 => Ok(Value::Unsigned(source.read_varint()?)),
            x if x == 5 => panic!("no signed values"),
            x if x == 6 => Ok(Value::Variable(source.read_varint()? as Variable)),
            x if x == 7 => Ok(Value::Empty()),
            x if x == 8 => Ok(Value::Union(source.read_varint()? as Variable)),            
            x => Err(err!("invalid Value codepoint {}", x)),
        }
    }
}

// oid or dyn?
pub fn get_u64(_e: DynEntityHandler, _a: Attribute) -> Result<u64, Error> {
    Ok(1)
}

pub fn get_string(_e: DynEntityHandler, _a: Attribute) -> Result<String, Error> {
    Ok("just kidding".to_string())
}

pub fn get_attributes(_e: DynEntityHandler) -> Result<DynStream<Attribute>, Error> {
    Err(err!("foo"))
}
