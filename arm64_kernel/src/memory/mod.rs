pub mod address;
//pub mod kbuf;
pub mod page;
pub mod page_alloc;
pub mod permissions;
pub mod pg_offset;
pub mod proc_vm;
pub mod region;
pub mod smalloc;

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SHIFT: usize = PAGE_SIZE.trailing_zeros() as usize;
pub const PAGE_MASK: usize = PAGE_SIZE - 1;

use crate::{
    arch::ArchImpl,
    OnceLock,
    SpinLock,
};
use crate::memory::{
    page_alloc::FrameAllocator,
    region::PhysMemoryRegion,
    smalloc::{RegionList, Smalloc},
};

pub mod fault;
//pub mod uaccess;

pub type PageOffsetTranslator = crate::memory::pg_offset::PageOffsetTranslator<ArchImpl>;

// Initial memory allocator. Used for initial memory setup.
const STATIC_REGION_COUNT: usize = 128;

static INIT_MEM_REGIONS: [PhysMemoryRegion; STATIC_REGION_COUNT] =
    [PhysMemoryRegion::empty(); STATIC_REGION_COUNT];
static INIT_RES_REGIONS: [PhysMemoryRegion; STATIC_REGION_COUNT] =
    [PhysMemoryRegion::empty(); STATIC_REGION_COUNT];

pub static INITIAL_ALLOCATOR: SpinLock<Option<Smalloc<PageOffsetTranslator>>> =
    SpinLock::new(Some(Smalloc::new(
        RegionList::new(STATIC_REGION_COUNT, INIT_MEM_REGIONS.as_ptr().cast_mut()),
        RegionList::new(STATIC_REGION_COUNT, INIT_RES_REGIONS.as_ptr().cast_mut()),
    )));

// Main page allocator, setup by consuming smalloc.
pub static PAGE_ALLOC: OnceLock<FrameAllocator<ArchImpl>> = OnceLock::new();
