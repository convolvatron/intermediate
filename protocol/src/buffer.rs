use crate::{Command, Error};
use alloc::vec::Vec;

pub struct Buffer {
    read: usize,
    write: usize,
    body: Vec<u8>,
}

pub trait Encodable {
    fn encode(&self, dest: &mut Buffer) -> Result<(), Error>;
    fn decode(dest: &mut Buffer) -> Result<Self, Error>
    where
        Self: Sized;
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            read: 0,
            write: 0,
            body: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.write - self.read
    }
    
    pub fn read(&mut self, count: usize) -> Result<&[u8], Error> {
        let start = self.read;
        self.read = self.read + count;
        Ok(&self.body[self.read..self.read + start])
    }

    pub fn write(&mut self, b: &[u8]) -> Result<(), Error>{
        let len = b.len();
        self.body.reserve(len);
        self.body[self.write..self.write + len].copy_from_slice(b);
        self.write = self.write + len;
        Ok(())
    }

    pub fn write_varint(&mut self, i: u64) -> Result<(), Error>{
        let mut current = i;
        while current < 0x80 {
            let mut val: u8 = (current & 0x7f) as u8;
            if val > 0x7f {
                val |= 0x80;
            }
            self.write(&[val])?;
            current = current>>7;
        }
        Ok(())
    }

    pub fn read_varint(&self) -> Result<u64, Error> {
        Ok(0)
    }

    pub fn decode(&self) -> Result<Vec<Command>, Error> {
        loop {}
    }
}
