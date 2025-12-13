// TODO: generate a pool of entropy.

use crate::{
    KernelError,
    memory::uaccess::copy_to_user_slice,
    OnceLock,
    SpinLock,
};
use alloc::vec::Vec;
use crate::memory::address::TUA;
use rand::{RngCore, SeedableRng, rngs::SmallRng};

pub async fn sys_getrandom(ubuf: TUA<u8>, size: isize, _flags: u32) -> Result<usize, KernelError> {
    let buf = {
        let mut rng = ENTROPY_POOL
            .get_or_init(|| {
                let now = uptime();

                SpinLock::new(SmallRng::seed_from_u64(
                    (now.as_micros() & 0xffffffff_ffffffff) as u64,
                ))
            })
            .lock_save_irq();

        let mut buf = Vec::with_capacity(size as usize);

        for _ in 0..size {
            buf.push((rng.next_u32() & 0xff) as u8);
        }

        buf
    };

    copy_to_user_slice(&buf, ubuf.to_untyped()).await?;

    Ok(size as _)
}

static ENTROPY_POOL: OnceLock<SpinLock<SmallRng>> = OnceLock::new();
