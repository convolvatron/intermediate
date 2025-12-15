use crate::{
    console,
    current_task,
};

pub async fn handle_syscall() {
    let task = current_task();
    let ctx = task.ctx.lock_save_irq();    

    console!("syscall {}", ctx.user().x[8]);

    let ret_val = 0;

    task.ctx.lock_save_irq().user_mut().x[0] = 18;
}
