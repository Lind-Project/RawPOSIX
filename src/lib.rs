#![feature(lazy_cell)]
#![feature(rustc_private)] //for private crate imports for tests
#![feature(vec_into_raw_parts)]
#![feature(thread_local)]
#![allow(unused_imports)]
#![feature(hash_extract_if)]
#![allow(dead_code)]

// interface and safeposix are public because otherwise there isn't a great
// way to 'use' them for benchmarking.
pub mod interface;
pub mod safeposix;
pub mod tests;
pub mod fdtables;

use crate::safeposix::dispatcher::*;

pub fn lind_lindrustinit(verbosity: i32) {
    unsafe {
        lindrustinit(verbosity as isize);
    }
}

pub fn lind_lindrustfinalize() {
    unsafe {
        lindrustfinalize();
    }
}

pub fn lind_rustposix_thread_init(cageid: u64, signalflag: u64) {
    unsafe {
        rustposix_thread_init(cageid, signalflag);
    }
}

pub fn lind_write_inner(fd: i32, buf: *const u8, count: usize, cageid: u64) {
    unsafe {
        quick_write(fd, buf, count, cageid);
    }
}

pub fn lind_fork(parent_cageid: u64, child_cageid: u64) -> i32 {
    unsafe {
        lind_syscall_api(
            parent_cageid,
            68 as u32,
            0,
            0,
            child_cageid,
            0,
            0,
            0,
            0,
            0,
        )
    }
}

pub fn lind_exit(cageid: u64, status: i32) -> i32 {
    unsafe {
        lind_syscall_api(
            cageid,
            30 as u32,
            0,
            0,
            status as u64,
            0,
            0,
            0,
            0,
            0,
        )
    }
}

pub fn lind_exec(parent_cageid: u64, child_cageid: u64) -> i32 {
    unsafe {
        lind_syscall_api(
            parent_cageid,
            69 as u32,
            0,
            0,
            child_cageid,
            0,
            0,
            0,
            0,
            0,
        )
    }
}

pub fn lind_syscall_inner(
    cageid: u64,
    call_number: u32,
    call_name: u64,
    start_address: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
    arg6: u64,
) -> i32 {
    unsafe {
        lind_syscall_api(
            cageid,
            call_number,
            call_name,
            start_address,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
        )
    }
}
