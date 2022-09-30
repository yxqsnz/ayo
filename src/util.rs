use libc::PRIO_PROCESS;

pub(crate) fn set_nice(pid: u32, nice: i64) -> i32 {
    unsafe { libc::setpriority(PRIO_PROCESS, pid, nice as _) }
}
