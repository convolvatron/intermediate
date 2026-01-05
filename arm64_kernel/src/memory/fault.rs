use alloc::boxed::Box;
use crate::{
    ProcVM,
    Error,
    PageInfo,
    UserAddressSpace,
    memory::{address::VA, permissions::PtePermissions, proc_vm::vmarea::AccessKind},
};

use super::{PAGE_ALLOC, page::ClaimedPage};


/// Handle a page fault when a PTE is not present.
pub fn handle_demand_fault(
    vm: &mut ProcVM,
    faulting_addr: VA,
    access_kind: AccessKind,
) -> Result<(), Error> {
    console("missing fault {}", faulting_addr);    
}

/// Handle a page fault when a page is present, but the access kind differ from
/// permissble accessees defined in the PTE, a 'protection' fault.
pub fn handle_protection_fault(
    vm: &mut ProcVM,
    faulting_addr: VA,
    access_kind: AccessKind,
    pg_info: PageInfo,
) -> Result<(), Error> {
    console("permissions fault {}", faulting_addr);
}
