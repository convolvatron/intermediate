use crate::{Command, Error};
use alloc::vec::Vec;

pub struct Buffer {
    read: usize,
    write: usize,
    body: Vec<u8>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer{read:0, write:0, body:Vec::new()}
    }

    pub fn read(&mut self, count: usize) -> Result<&[u8], Error> {
        let start = self.read;
        self.read = self.read + count;
        Ok(&self.body[self.read..self.read + start])
    }

    pub fn write(&mut self, b: &[u8]) {
        let len = b.len();
        self.body.reserve(len);
        self.body[self.write..self.write + len].copy_from_slice(b);
        self.write = self.write + len;
    }

    pub fn write_varint(&mut self, i: u64) {
        while i < 0x80 {
            let mut val: u8 = (i & 0x7f) as u8;
            if val > 0x7f {
                val |= 0x80;
            }
            self.write(&[val]);
        }
    }

    pub fn read_varint(&self) -> Result<u64, Error> {
        Ok(0)
    }

    pub fn decode(&self) -> Result<Vec<Command>, Error> {
        loop {}
    }
}
