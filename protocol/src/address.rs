use crate::{Attribute, Encodable, Buffer, Error, Oid, err};
use alloc::boxed::Box;

#[repr(u8)]
pub enum Address {
    Entity(Oid, Attribute) = 1,    
    Offset(Box<Address>, u64) = 2,
}

impl Encodable for Address {
    fn encode(&self, _dest:&mut Buffer) {
    }
    
    fn decode(source: &mut Buffer) -> Result<Self, Error> {
        match source.read(1)?[0] {
            x if x == Address::Entity as u8 => {
                Ok(Address::Entity(Encodable::decode(source)?, Encodable::decode(source)?))
            }
            x if x == Address::Offset as u8 => {
                Ok(Address::Offset(Box::new(Encodable::decode(source)?),
                                   u64::from_be_bytes(source.read(8)?.try_into().expect("cant"))))
            }
            x => Err(err!("invalid address {}", x))
        }
    }
}
    
