
pub struct Error {
}

pub enum BaseValue {
    Oid = 1,
    Utf8String= 2,
    Bytes = 3,    
    Unsigned = 4,
    Signed = 5,
    Variable = 6, // the zero variable is the always matches never non-emtpy '_' of this world
    Empty = 7, // used for deletia        
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


impl Value {
    fn encode(&self, dest:Buffer) {
    }
    
    fn decode(dest:Buffer) -> Result<Value, Error> {
    }
    
}
