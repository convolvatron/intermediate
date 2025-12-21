struct Buffer {
    read: u64,
    write: u64,    
    body: Vec<u8>, 
};

impl Buffer {
    fn read(&self, count:usize) {
    }
    fn write(&self, count:usize) {
    }    
}

