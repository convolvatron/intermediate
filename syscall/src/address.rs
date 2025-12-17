// memory region...
enum Address {
    Offset(u64),
    IPv4(u32),
    Instance(u32),
    ByteRange((u64, u64)),    
}
