use protocol::Error;

use core::{
    ops::{Deref, DerefMut},
};
use crate::{
    ksym_pa, 
    arch::arm64::{fdt::MAX_FDT_SZ, SpinLock},    
    arch::arm64::memory::{
        FIXMAP_BASE,         
        pg_descriptors::{
            L0Descriptor, L1Descriptor, L2Descriptor, L3Descriptor, MemoryType, PaMapper,
            PageTableEntry, TableMapper,
        },
        pg_tables::{L0Table, L1Table, L2Table, L3Table, PgTable, PgTableArray},
    },
    memory::{
        PAGE_SIZE,
        address::{IdentityTranslator, TPA, TVA, VA},
        permissions::PtePermissions,
    },
};

pub struct TempFixmapGuard<T> {
    fixmap: *mut Fixmap,
    va: TVA<T>,
}

impl<T> TempFixmapGuard<T> {
    /// Get the VA associated with this temp fixmap.
    ///
    /// SAFETY: The returned VA is not tied back to the lifetime of the guard.
    /// Thefeore, care *must* be taken that it is not used after the guard has
    /// gone out of scope.
    pub unsafe fn get_va(&self) -> TVA<T> {
        self.va
    }
}

impl<T> Deref for TempFixmapGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.va.as_ptr().cast::<T>().as_ref().unwrap() }
    }
}

impl<T> DerefMut for TempFixmapGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.va.as_ptr_mut().cast::<T>().as_mut().unwrap() }
    }
}

impl<T> Drop for TempFixmapGuard<T> {
    fn drop(&mut self) {
        unsafe {
            let fixmap = &mut *self.fixmap;
            fixmap.unmap_temp_page();
        }
    }
}

#[derive(Clone, Copy)]
#[repr(usize)]
enum FixmapSlot {
    DtbStart = 0,
    _DtbEnd = MAX_FDT_SZ / PAGE_SIZE, // 2MiB max DTB size,
    PgTableTmp,
}

pub struct Fixmap {
    l1: PgTableArray<L1Table>,
    l2: PgTableArray<L2Table>,
    l3: [PgTableArray<L3Table>; 2],
}

unsafe impl Send for Fixmap {}
unsafe impl Sync for Fixmap {}

pub static FIXMAPS: SpinLock<Fixmap> = SpinLock::new(Fixmap::new());

impl Fixmap {
    pub const fn new() -> Self {
        Self {
            l1: PgTableArray::new(),
            l2: PgTableArray::new(),
            l3: [const { PgTableArray::new() }; 2],
        }
    }

    pub fn setup_fixmaps(&mut self, l0_base: TPA<PgTableArray<L0Table>>) {
        let l0_table = L0Table::from_ptr(l0_base.to_va::<IdentityTranslator>());
//        let invalidator = AllEl1TlbInvalidator::new();

        L1Table::from_ptr(TVA::from_ptr(&mut self.l1 as *mut _)).set_desc(
            FIXMAP_BASE,
            L1Descriptor::new_next_table(ksym_pa!(self.l2)),
//            &invalidator,
        );

        L2Table::from_ptr(TVA::from_ptr(&mut self.l2 as *mut _)).set_desc(
            FIXMAP_BASE,
            L2Descriptor::new_next_table(ksym_pa!(self.l3[0])),
//            &invalidator,
        );

        L2Table::from_ptr(TVA::from_ptr(&mut self.l2 as *mut _)).set_desc(
            VA::from_value(FIXMAP_BASE.value() + (1 << L2Table::SHIFT)),
            L2Descriptor::new_next_table(ksym_pa!(self.l3[1])),
//            &invalidator,
        );

        l0_table.set_desc(
            FIXMAP_BASE,
            L0Descriptor::new_next_table(ksym_pa!(self.l1)),
//            &invalidator,
        );
    }

    pub fn temp_remap_page_table<T: PgTable>(
        &mut self,
        pa: TPA<PgTableArray<T>>,
    ) -> Result<TempFixmapGuard<PgTableArray<T>>, Error> {
        let va = Self::va_for_slot(FixmapSlot::PgTableTmp);
//        let invalidator = AllEl1TlbInvalidator::new();

        L3Table::from_ptr(TVA::from_ptr_mut(&mut self.l3[1] as *mut _)).set_desc(
            va,
            L3Descriptor::new_map_pa(
                pa.to_untyped(),
                MemoryType::Normal,
                PtePermissions::rw(false),
            ),
//            &invalidator,
        );

        Ok(TempFixmapGuard {
            fixmap: self as *mut _,
            va: va.cast(),
        })
    }

    fn unmap_temp_page(&mut self) {
        let va = Self::va_for_slot(FixmapSlot::PgTableTmp);
//        let invalidator = AllEl1TlbInvalidator::new();

        L3Table::from_ptr(TVA::from_ptr_mut(&mut self.l3[1] as *mut _)).set_desc(
            va,
            L3Descriptor::invalid(),
//            &invalidator,
        );
    }

    fn va_for_slot(slot: FixmapSlot) -> VA {
        TVA::from_value(FIXMAP_BASE.value() + (slot as usize * PAGE_SIZE))
    }
}
