use aarch64_cpu::{
    asm::wfi,
    registers::{DAIF, MPIDR_EL1, ReadWriteable, Readable},
};
use cpu_ops::{local_irq_restore, local_irq_save};
use exceptions::ExceptionState;
use memory::{
    PAGE_OFFSET,
    address_space::Arm64ProcessAddressSpace,
    mmu::{Arm64KernelAddressSpace, KERN_ADDR_SPC},
};

use crate::{
    arch::Arch,
    KernelError,
    CpuOps, VirtualMemory,
    arch::arm64::memory::pg_tables::{L0Table, PgTableArray},
    memory::address::{UA, VA},
//    linux::Task,
    SpinLock,
};

pub mod boot;
mod cpu_ops;
mod exceptions;
mod fdt;
mod memory;

pub struct Aarch64 {}
impl CpuOps for Aarch64 {
    fn id() -> usize {
        MPIDR_EL1.read(MPIDR_EL1::Aff0) as _
    }

    fn halt() -> ! {
        loop {
            wfi();
        }
    }

    fn disable_interrupts() -> usize {
        local_irq_save()
    }

    fn restore_interrupt_state(state: usize) {
        local_irq_restore(state);
    }

    fn enable_interrupts() {
        DAIF.modify(DAIF::I::Unmasked);
    }
}

impl VirtualMemory for Aarch64 {
    type PageTableRoot = PgTableArray<L0Table>;
    type ProcessAddressSpace = Arm64ProcessAddressSpace;
    type KernelAddressSpace = Arm64KernelAddressSpace;

    const PAGE_OFFSET: usize = PAGE_OFFSET;

    fn kern_address_space() -> &'static SpinLock<Self::KernelAddressSpace> {
        KERN_ADDR_SPC.get().unwrap()
    }
}

impl Arch for Aarch64 {
    type UserContext = ExceptionState;

    fn new_user_context(entry_point: VA, stack_top: VA) -> Self::UserContext {
        ExceptionState {
            x: [0; 31],
            elr_el1: entry_point.value() as _,
            spsr_el1: 0,
            sp_el0: stack_top.value() as _,
            tpid_el0: 0,
        }
    }

    fn name() -> &'static str {
        "aarch64"
    }

    unsafe fn copy_from_user(
        src: UA,
        dst: *mut (),
        len: usize,
    ) -> impl Future<Output = Result<(), KernelError>> {
        Arm64CopyFromUser::new(src, dst, len)
    }

    unsafe fn copy_to_user(
        src: *const (),
        dst: UA,
        len: usize,
    ) -> impl Future<Output = Result<(), KernelError>> {
        Arm64CopyToUser::new(src, dst, len)
    }

    unsafe fn copy_strn_from_user(
        src: UA,
        dst: *mut u8,
        len: usize,
    ) -> impl Future<Output = Result<usize, KernelError>> {
        Arm64CopyStrnFromUser::new(src, dst as *mut _, len)
    }
}
