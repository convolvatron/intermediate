use crate::process::Task;
use alloc::sync::Arc;
use crate::UserAddressSpace;

pub mod idle;
pub mod signal;

pub fn context_switch(new: Arc<Task>) {
    new.vm
        .lock_save_irq()
        .mm_mut()
        .address_space_mut()
        .activate();
}
