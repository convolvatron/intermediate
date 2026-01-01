use crate::{
    console, 
    arch::arm64::memory::{mmu::setup_kern_addr_space, 
                          fixmap::FIXMAPS},
    arch::arm64::ExceptionState,
    arch::{ArchImpl, arm64::exceptions::exceptions_init},    
    arch::arm64::memory::pg_tables::{L0Table, PgTableArray},    
    KernelError,
    interrupts::{
        // cpu_messenger::{Message, cpu_messenger_init, message_cpu},
        get_interrupt_root,
    },
    CpuOps,
    memory::{
        INITIAL_ALLOCATOR,
        PAGE_ALLOC,
        address::{PA, TPA, VA},
        page_alloc::FrameAllocator,
    },
    sync::per_cpu::setup_percpu,    
    kmain,
};
use aarch64_cpu::{
    asm::{self, barrier},
    registers::{ReadWriteable, SCTLR_EL1, TCR_EL1},
};
use core::arch::global_asm;
use logical_map::setup_logical_map;
use memory::{setup_allocator, setup_stack_and_heap};

mod exception_level;
mod logical_map;
mod memory;
mod paging_bootstrap;

global_asm!(include_str!("start.s"));

unsafe extern "C" {fn print(s:u64, len:u64);}


/// Stage 1 Initialize of the system architechture.
///
/// This function is called by the main primary CPU with the other CPUs parked.
/// All interrupts should be disabled, the ID map setup in TTBR0 and the highmem
/// map setup in TTBR1.
///
/// The memory map is setup as follows:
///
/// 0xffff_0000_0000_0000 - 0xffff_8000_0000_0000 | Logical Memory Map
/// 0xffff_8000_0000_0000 - 0xffff_8000_1fff_ffff | Kernel image
/// 0xffff_9000_0000_0000 - 0xffff_9000_0020_1fff | Fixed mappings
/// 0xffff_b000_0000_0000 - 0xffff_b000_0400_0000 | Kernel Heap
/// 0xffff_b800_0000_0000 - 0xffff_b800_0000_8000 | Kernel Stack (per CPU)
/// 0xffff_d000_0000_0000 - 0xffff_d000_ffff_ffff | MMIO remap
/// 0xffff_e000_0000_0000 - 0xffff_e000_0000_0800 | Exception Vector Table
///
/// Returns the stack pointer in X0, which should be hen set by the boot asm.

pub fn console_output(s:&str) {
    unsafe {
        print(s.as_ptr() as u64, s.len() as u64);
    };    
}
    
#[unsafe(no_mangle)]
fn arch_init_stage1(
    dtb_ptr: TPA<u8>,
    image_start: PA,
    image_end: PA,
    highmem_pgtable_base: TPA<PgTableArray<L0Table>>,
) -> VA {

    (|| -> Result<VA, KernelError> {
        console!("secondardy\n");
        setup_allocator(dtb_ptr, image_start, image_end)?;
        console!("C\n");        
        let dtb_addr = {
            let mut fixmaps = FIXMAPS.lock_save_irq();
            fixmaps.setup_fixmaps(highmem_pgtable_base);

            unsafe { fixmaps.remap_fdt(dtb_ptr) }.unwrap()
        };
        console!("C\n");

        // set_fdt_va(dtb_addr.cast());
        setup_logical_map(highmem_pgtable_base)?;
        let stack_addr = setup_stack_and_heap(highmem_pgtable_base)?;
        setup_kern_addr_space(highmem_pgtable_base)?;

        Ok(stack_addr)
    })()
    .unwrap_or_else(|_| park_cpu())
}

#[unsafe(no_mangle)]
fn arch_init_stage2(frame: *mut ExceptionState) -> *mut ExceptionState {
    
    // Save the ID map addr for booting secondaries.
    // save_idmap(PA::from_value(TTBR0_EL1.get_baddr() as _));

    // Disable the ID map.
    TCR_EL1.modify(TCR_EL1::EPD0::DisableTTBR0Walks);
    barrier::isb(barrier::SY);

    // We now have enough memory setup to switch to the real page allocator.
    let smalloc = INITIAL_ALLOCATOR
        .lock_save_irq()
        .take()
        .expect("Smalloc should not have been taken yet");

    let page_alloc = unsafe { FrameAllocator::init(smalloc) };

    if PAGE_ALLOC.set(page_alloc).is_err() {
        panic!("Cannot setup physical memory allocator");
    }

    // Don't trap wfi/wfe in el0.
    SCTLR_EL1.modify(SCTLR_EL1::NTWE::DontTrap + SCTLR_EL1::NTWI::DontTrap);

    exceptions_init().expect("Failed to initialize exceptions");
    ArchImpl::enable_interrupts();

    //    unsafe { run_initcalls() };
    //    probe_for_fdt_devices();
    //    cpu_count())
    unsafe { setup_percpu(1) };

    cpu_messenger_init(1);

    kmain(
        frame,
    );

    // boot_secondaries();

    // Prove that we can send IPIs through the messenger.
    let _ = message_cpu(1, Message::Ping(ArchImpl::id() as _));

    frame
}

fn arch_init_secondary(ctx_frame: *mut ExceptionState) -> *mut ExceptionState {
    // Disable the ID map.
    TCR_EL1.modify(TCR_EL1::EPD0::DisableTTBR0Walks);
    barrier::isb(barrier::SY);

    // Enable interrupts and exceptions.
    // secondary_exceptions_init();

    if let Some(ic) = get_interrupt_root() {
        ic.enable_core(ArchImpl::id());
    }

    ArchImpl::enable_interrupts();

//    secondary_booted();

//    sched_init_secondary();

    // dispatch_userspace_task(ctx_frame);

    ctx_frame
}

#[unsafe(no_mangle)]
pub extern "C" fn park_cpu() -> ! {
    loop {
        asm::wfe();
    }
}
