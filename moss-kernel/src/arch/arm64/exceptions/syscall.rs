use crate::{
    current_task,
};

pub async fn handle_syscall() {
    let task = current_task();

    let (nr, arg1, arg2, arg3, arg4, arg5, arg6) = {
        let ctx = task.ctx.lock_save_irq();
        let state = ctx.user();

        (
            state.x[8] as u32,
            state.x[0],
            state.x[1],
            state.x[2],
            state.x[3],
            state.x[4],
            state.x[5],
        )
    };

    console!("syscall {}", nr);

    let ret_val = 0;

    task.ctx.lock_save_irq().user_mut().x[0] = ret_val.cast_unsigned() as u64;
}
