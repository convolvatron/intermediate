use alloc::string::String;

use crate::{
    err,
    Attribute,
    Buffer,
    Encodable,
    Entity,
    Error,
    Value,
};

#[repr(u8)]
pub enum Status {
    Success = 1,
    Error(String) = 2,
}


// ok. we had some intention to expose error processing to this minilanguage. i still
// think its important. however, the semantics for values which aren't intersections
// with the underlying data isn't at all clear. so, we're just going to do it out of
// band. the semantics being that once an error has occured,  the stream reports
// it as a special object, and the stream is closed.

// entity means entity or variable,
// create takes a unbound variable only,
// the last argument of copy is an unsigned or variable
// i know dependent types are supposed to be the answer, but i dont think
// this would require such a large hammer
#[derive(Debug)]
#[repr(u8)]
pub enum Command {
    Get(Entity, Attribute, Value) = 1,
    Set(Entity, Attribute, Value) = 2,
    Copy(Entity, Attribute, Value,
         Entity, Attribute, Value,
         Value) = 3,
    Create(Value) = 4,
}

// xxx - discriminant issues resulting in an unforuntate duplication that
// requires manual consistency for the codepoint
impl Encodable for Command {
    fn decode(source: &mut Buffer) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match source.read(1)?[0] {
            1 => Ok(Command::Get(
                Value::decode(source)?,
                Attribute::decode(source)?,
                Value::decode(source)?,
            )),
            2 => Ok(Command::Set(
                Value::decode(source)?,
                Attribute::decode(source)?,
                Value::decode(source)?,
            )),
            3 => Ok(Command::Copy(
                Value::decode(source)?,
                Attribute::decode(source)?,
                Value::decode(source)?,                
                Attribute::decode(source)?,
                Value::decode(source)?,
                Value::decode(source)?,                
                Value::Unsigned(u64::from_be_bytes(source.read(8)?.try_into().expect("word"))),
            )),
            4 => Ok(Command::Create(Value::decode(source)?)),
            x => Err(err!("invalid protocol command code {}", x as u8)),
        }
    }

    fn encode(&self, b: &mut Buffer) -> Result<(), Error> {
        match &self {
            Command::Get(e, a, v) => {
                b.write(&[1])?;
                e.encode(b)?;
                a.encode(b)?;
                v.encode(b)?;
            }
            Command::Set(e, a, v) => {
                b.write(&[2])?;
                e.encode(b)?;
                a.encode(b)?;
                v.encode(b)?;
            }
            Command::Copy(source_entity, source_attribute, source_offset,
                          dest_entity, dest_attribute, dest_offset,
                          length) => {
                b.write(&[3])?;
                source_entity.encode(b)?;
                source_attribute.encode(b)?;
                source_offset.encode(b)?;                
                dest_entity.encode(b)?;
                dest_attribute.encode(b)?;
                dest_offset.encode(b)?;                                
                length.encode(b)?;
            }
            Command::Create(dest) => {
                b.write(&[4])?;
                dest.encode(b)?;
            }
        }
        Ok(())
    }
}
