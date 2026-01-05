// this is a logical kernel instance, really just a place to stash all the
// globals from moss-kernel in someplace that isn't so global

pub struct Kernel {
    pid_count : alloc::sync::Atomic::u64,
}

