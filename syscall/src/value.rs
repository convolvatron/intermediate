
pub struct Error {
}

pub enum BaseValue {
    Oid,
    Address,
    Utf8String,
    Bytes,    
    Unsigned,
    Signed,
}

trait Encodable {
    fn encode(&self, dest:Buffer);
    fn decode(dest:Buffer) -> Result<Self, Error>;
}

type Value: BaseValue + Encodable;

impl Encodable for Oid {
    fn encode(&self, dest:Buffer);
    fn decode(dest:Buffer) -> Result<Self, Error>;
}

