use libc::{c_int, pid_t};

extern "C" {
    pub fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
}

// std::Command can leave behind zombie processes that buid up over time, this small function uses
// unsafe parts of libc but it reliably gets rid of any zombies that are left over
#[inline]
pub fn reap() {
    unsafe {
        waitpid(-1, std::ptr::null_mut(), 0x00000001);
    }
}
