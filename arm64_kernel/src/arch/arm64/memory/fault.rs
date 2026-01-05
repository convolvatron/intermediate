use protocol::{Error};
use crate::{
    UserAddressSpace,
    memory::{address::VA, proc_vm::vmarea::AccessKind, region::VirtMemoryRegion},    
    arch::arm64::{
        exceptions::{
            ExceptionState,
            esr::{AbortIss, Exception, IfscCategory},
        },
    },
    memory::fault::{handle_demand_fault, handle_protection_fault},
};

#[repr(C)]
struct FixupTable {
    start: VA,
    end: VA,
    fixup: VA,
}

unsafe extern "C" {
    static __UACCESS_FIXUP: FixupTable;
}

impl FixupTable {
    fn is_in_fixup(&self, addr: VA) -> bool {
        VirtMemoryRegion::from_start_end_address(self.start, self.end).contains_address(addr)
    }
}

fn run_mem_fault_handler(exception: Exception, info: AbortIss) -> Result<FaultResolution, Error> {
    let access_kind = determine_access_kind(exception, info);

    if let Some(far) = info.far {
        let fault_addr = VA::from_value(far as usize);

        let task = current_task();
        let mut vm = task.vm.lock_save_irq();

        match info.ifsc.category() {
            IfscCategory::TranslationFault => handle_demand_fault(&mut vm, fault_addr, access_kind),
            IfscCategory::PermissionFault => {
                let pg_info = vm
                    .mm_mut()
                    .address_space_mut()
                    .translate(fault_addr)
                    .expect("Could not find PTE in permission fault");

                handle_protection_fault(&mut vm, fault_addr, access_kind, pg_info)
            }
            _ => panic!("Unhandled memory fault"),
        }
    } else {
        panic!("Instruction/Data abort with no valid Fault Address Register",);
    }
}


pub fn handle_kernel_mem_fault(exception: Exception, info: AbortIss, state: &mut ExceptionState) {
/*    if unsafe { __UACCESS_FIXUP.is_in_fixup(VA::from_value(state.elr_el1 as usize)) } {
        handle_uacess_abort(exception, info, state);
        return;
    }
*/
    // If the source of the fault (ELR), wasn't in the uacess fixup section,
    // then any abort genereated by the kernel is a panic since we don't
    // demand-page any kernel memory.
    panic!("Kernel memory fault detected.  Context: {}", state);
}

pub fn handle_mem_fault(exception: Exception, info: AbortIss) {
    run_mem_fault_handler(exception, info);    
}

fn determine_access_kind(exception: Exception, info: AbortIss) -> AccessKind {
    if matches!(exception, Exception::InstrAbortLowerEL(_)) {
        AccessKind::Execute
    } else {
        // We know it must be a DataAbort, so we can use `info.write`.
        if info.write {
            AccessKind::Write
        } else {
            AccessKind::Read
        }
    }
}
