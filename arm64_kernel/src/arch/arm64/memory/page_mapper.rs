use protocol::Error;

use crate::{
    memory::PageOffsetTranslator,
    arch::arm64::memory::pg_tables::{PageTableMapper, PgTable, PgTableArray},
    memory::address::{TPA, TVA},
};

pub struct PageOffsetPgTableMapper {}

impl PageTableMapper for PageOffsetPgTableMapper {
    unsafe fn with_page_table<T: PgTable, R>(
        &mut self,
        pa: TPA<PgTableArray<T>>,
        f: impl FnOnce(TVA<PgTableArray<T>>) -> R,
    ) -> Result<R, Error> {
        Ok(f(pa.to_va::<PageOffsetTranslator>()))
    }
}
