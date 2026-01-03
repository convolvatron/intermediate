use crate::{memory::address::TUA,
            KernelError, 
            timespec::TimeSpec};

pub async fn sys_nanosleep(_rqtp: TUA<TimeSpec>, _rmtp: TUA<TimeSpec>) -> Result<usize, KernelError> {
//    let timespec = TimeSpec::copy_from_user(rqtp).await?;
//
//    sleep(timespec.into()).await;

    Ok(0)
}
