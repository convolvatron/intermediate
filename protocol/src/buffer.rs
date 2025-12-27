use crate::{Attribute, Value, Command, Error, Address, Oid, Variable, Encodable};
use alloc::{vec::Vec};

pub struct Buffer {
    read: usize,
    write: usize,    
    body: Vec<u8>, 
}

impl Buffer {
    pub fn read(&mut self, count:usize) -> Result<&[u8], Error> {
        let start = self.read;
        self.read = self.read + count;
        Ok(&self.body[self.read..self.read+start])
    }
    
    pub fn write(&self, b:&[u8]) {
        let len = b.len();
        self.body.reserve(len);
        self.body[self.write..self.write+len].copy_from_slice(b);
        self.write = self.write + len;
    }

    pub fn write_varint(&self, i:u64) {
        while i < 0x80 {
            let val:u8 = (i&0x7f) as u8;
            if val> 0x7f {
                val |= 0x80;
            }
            self.write(&[val]);
        }
    }

    pub fn read_varint(&self) -> Result<u64, Error> {
        Ok(0)
    }
    
    // command append
    pub fn get(&mut self, e:Oid, a:Attribute, v:Value, r:Variable) {
        self.write(&[Command::Get as u8])
    }
    
    pub fn set(&mut self, e:Oid, a:Attribute, v:Value, r:Variable) {
        self.write(&[Command::Set as u8])
    }
    
    pub fn copy(&mut self, source:Address, dest:Address, length:u64, r:Variable) {
        self.write(&[Command::Copy as u8]);
        source.encode(self);
        dest.encode(self);
        self.write(&length.to_be_bytes());
        r.encode(self);        
    }
    
    pub fn create(&mut self, r:Value) {
        self.write(&[Command::Create as u8])
    }
    
    pub fn decode(&self) -> Result<Vec<Command>, Error>{
        loop {
        }
        
    }
}

