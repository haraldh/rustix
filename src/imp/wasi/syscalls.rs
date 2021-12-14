// There are a lot of filesystem and network system calls, so they're split
// out into separate files.
pub(crate) use super::fs::syscalls::*;
pub(crate) use super::io::syscalls::*;
pub(crate) use super::thread::syscalls::*;
pub(crate) use super::time::syscalls::*;

#[cfg(windows)]
use super::fd::{BorrowedFd, LibcFd, RawFd};
#[cfg(any(windows, target_os = "linux"))]
use crate::io;

#[inline]
pub(crate) fn exit_group(code: i32) -> ! {
    todo!("exit_group")
}

#[inline]
pub(crate) fn sched_yield() {
    todo!("sched_yield")
}
