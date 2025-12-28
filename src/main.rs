#![feature(btree_cursors)]
use std::alloc::{Layout, alloc};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::slice;
use xhypervisor::*;

const PAGESIZE: usize = 65536;

fn pad(x: usize, by: usize) -> usize {
    (((x - 1) / by) + 1) * by
}

fn aligned_mem(size: usize) -> Result<&'static mut [u8], std::io::Error> {
    let rlen = pad(size, PAGESIZE);
    let ptr = unsafe {
        let ptr =
            alloc(Layout::from_size_align(rlen, PAGESIZE).map_err(|e| std::io::Error::other(e))?);
        if ptr.is_null() {
            return Err(std::io::Error::other("allocate"));
        }
        slice::from_raw_parts_mut(ptr, rlen)
    };
    Ok(ptr)
}

fn load_kernel_aligned(pathname: String) -> Result<&'static mut [u8], std::io::Error> {
    let metadata = std::fs::metadata(&pathname)?;
    let ptr = aligned_mem(metadata.len() as usize)?;
    let mut f = File::open(&pathname)?;
    f.read(ptr)?;
    let file =
        elf::ElfBytes::<elf::endian::LittleEndian>::minimal_parse(ptr).expect("elf parse kernel");
    /*
        for phdr in file.segments().expect("segments") {
    //        let ptr = aligned_mem(phdr.p_memsz as usize)?;
            let start:usize = phdr.p_offset as usize;
            let end = pad(start + (phdr.p_filesz as usize), PAGESIZE);
            println!("seg {:x} {:x} {:x} {:x} {:x}", start, end, metadata.len(), phdr.p_vaddr, end-start);
    //        map_mem(&ptr[start..end], phdr.p_vaddr, MemPerm::ExecWrite).unwrap();
    //        break;
            // zero p_memsz - p_filesz
    }
        */
    Ok(ptr)
}

struct VM {
    regions: BTreeMap<u64, (u64, usize)>,
}

impl VM {
    fn new() -> VM {
        create_vm().unwrap();
        VM {
            regions: BTreeMap::new(),
        }
    }

    fn map(
        &mut self,
        source: *const u8,
        target: u64,
        length: usize,
        perm: xhypervisor::MemPerm,
    ) -> Result<(), std::io::Error> {
        self.regions.insert(target, (source as u64, length));
        unsafe {
            let s = slice::from_raw_parts(source, pad(length, PAGESIZE));
            // error?
            map_mem(s, target, perm).unwrap();
            Ok(())
        }
    }

    fn guest_to_host(&self, source: u64) -> Result<u64, std::io::Error> {
        match self
            .regions
            .lower_bound(std::ops::Bound::Included(&source))
            .peek_prev()
        {
            Some(x) => Ok(x.1.0 + (source - x.0)),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "missing region",
            )),
        }
    }
}

fn vm_create() {
    let kernel = load_kernel_aligned(std::env::args().nth(1).expect("rag")).expect("kernel");
    // from elf
    const EL1_USER_PAYLOAD_ADDRESS: u64 = 0x10000000;
    let mut vm = VM::new();
    let vcpu = VirtualCpu::new(0).unwrap();

    // should use start address and .. you know
    vm.map(
        kernel[0x10000..].as_ptr() as *const u8,
        EL1_USER_PAYLOAD_ADDRESS,
        (kernel.len() - 0x10000) as usize,
        MemPerm::ExecReadWrite,
    )
    .unwrap();

    vcpu.write_register(Register::CPSR, 0x3c4).unwrap();
    vcpu.write_register(Register::PC, EL1_USER_PAYLOAD_ADDRESS)
        .unwrap();

    loop {
        vcpu.run().unwrap();
        let reason = vcpu.exit_reason();

        match reason {
            VirtualCpuExitReason::Exception { exception } => {
                let ec = (exception.syndrome >> 26) & 0x3f;

                if ec == 0x16 {
                    unsafe {
                        let n = vm
                            .guest_to_host(vcpu.read_register(Register::X0).unwrap())
                            .expect("translate");
                        let s = slice::from_raw_parts(
                            n as *const u8,
                            vcpu.read_register(Register::X1).unwrap() as usize,
                        );
                        println!("{}", str::from_utf8(s).expect("Invalid UTF-8"));
                    }
                    continue;
                //		    break;
                } else {
                    println!(
                        "address: {:x} {:x}",
                        exception.virtual_address, exception.physical_address
                    );
                    println!("Unknown exception class 0x{:x}", ec);
                    break;
                }
            }
            reason => {
                println!("Unexpected exit! Reason: {:?}", reason);
                break;
            }
        }
    }
    //		dealloc(mem_raw, layout);
}

fn main() {
    vm_create();
}
