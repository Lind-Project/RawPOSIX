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
// mod lib_fs_utils;
pub mod safeposix;
pub mod tests;
pub mod fdtables;

// extern crate libc;
// pub mod librawposix {
//     use libc::{
//         c_char, c_void, fd_set, itimerval, off_t, rlimit, sockaddr, socklen_t, ssize_t, statfs,
//         timespec, timeval,
//     };

//     pub const LIND_SAFE_FS_ACCESS: i32 = 2;
//     pub const LIND_SAFE_FS_UNLINK: i32 = 4;
//     pub const LIND_SAFE_FS_LINK: i32 = 5;
//     pub const LIND_SAFE_FS_RENAME: i32 = 6;

//     pub const LIND_SAFE_FS_XSTAT: i32 = 9;
//     pub const LIND_SAFE_FS_OPEN: i32 = 10;
//     pub const LIND_SAFE_FS_CLOSE: i32 = 11;
//     pub const LIND_SAFE_FS_READ: i32 = 12;
//     pub const LIND_SAFE_FS_WRITE: i32 = 13;
//     pub const LIND_SAFE_FS_LSEEK: i32 = 14;
//     pub const LIND_SAFE_FS_IOCTL: i32 = 15;
//     pub const LIND_SAFE_FS_TRUNCATE: i32 = 16;
//     pub const LIND_SAFE_FS_FXSTAT: i32 = 17;
//     pub const LIND_SAFE_FS_FTRUNCATE: i32 = 18;
//     pub const LIND_SAFE_FS_FSTATFS: i32 = 19;
//     pub const LIND_SAFE_FS_MMAP: i32 = 21;
//     pub const LIND_SAFE_FS_MUNMAP: i32 = 22;
//     pub const LIND_SAFE_FS_GETDENTS: i32 = 23;
//     pub const LIND_SAFE_FS_DUP: i32 = 24;
//     pub const LIND_SAFE_FS_DUP2: i32 = 25;
//     pub const LIND_SAFE_FS_STATFS: i32 = 26;
//     pub const LIND_SAFE_FS_FCNTL: i32 = 28;

//     pub const LIND_SAFE_SYS_GETPPID: i32 = 29;
//     pub const LIND_SAFE_SYS_EXIT: i32 = 30;
//     pub const LIND_SAFE_SYS_GETPID: i32 = 31;

//     pub const LIND_SAFE_NET_BIND: i32 = 33;
//     pub const LIND_SAFE_NET_SEND: i32 = 34;
//     pub const LIND_SAFE_NET_SENDTO: i32 = 35;
//     pub const LIND_SAFE_NET_RECV: i32 = 36;
//     pub const LIND_SAFE_NET_RECVFROM: i32 = 37;
//     pub const LIND_SAFE_NET_CONNECT: i32 = 38;
//     pub const LIND_SAFE_NET_LISTEN: i32 = 39;
//     pub const LIND_SAFE_NET_ACCEPT: i32 = 40;

//     pub const LIND_SAFE_NET_GETSOCKOPT: i32 = 43;
//     pub const LIND_SAFE_NET_SETSOCKOPT: i32 = 44;
//     pub const LIND_SAFE_NET_SHUTDOWN: i32 = 45;
//     pub const LIND_SAFE_NET_SELECT: i32 = 46;
//     pub const LIND_SAFE_FS_GETCWD: i32 = 47;
//     pub const LIND_SAFE_NET_POLL: i32 = 48;
//     pub const LIND_SAFE_NET_SOCKETPAIR: i32 = 49;
//     pub const LIND_SAFE_SYS_GETUID: i32 = 50;
//     pub const LIND_SAFE_SYS_GETEUID: i32 = 51;
//     pub const LIND_SAFE_SYS_GETGID: i32 = 52;
//     pub const LIND_SAFE_SYS_GETEGID: i32 = 53;
//     pub const LIND_SAFE_FS_FLOCK: i32 = 54;

//     pub const LIND_SAFE_NET_EPOLL_CREATE: i32 = 56;
//     pub const LIND_SAFE_NET_EPOLL_CTL: i32 = 57;
//     pub const LIND_SAFE_NET_EPOLL_WAIT: i32 = 58;

//     pub const LIND_SAFE_FS_SHMGET: i32 = 62;
//     pub const LIND_SAFE_FS_SHMAT: i32 = 63;
//     pub const LIND_SAFE_FS_SHMDT: i32 = 64;
//     pub const LIND_SAFE_FS_SHMCTL: i32 = 65;

//     pub const LIND_SAFE_FS_PIPE: i32 = 66;
//     pub const LIND_SAFE_FS_PIPE2: i32 = 67;
//     pub const LIND_SAFE_FS_FORK: i32 = 68;
//     pub const LIND_SAFE_FS_EXEC: i32 = 69;

//     pub const LIND_SAFE_MUTEX_CREATE: i32 = 70;
//     pub const LIND_SAFE_MUTEX_DESTROY: i32 = 71;
//     pub const LIND_SAFE_MUTEX_LOCK: i32 = 72;
//     pub const LIND_SAFE_MUTEX_TRYLOCK: i32 = 73;
//     pub const LIND_SAFE_MUTEX_UNLOCK: i32 = 74;
//     pub const LIND_SAFE_COND_CREATE: i32 = 75;
//     pub const LIND_SAFE_COND_DESTROY: i32 = 76;
//     pub const LIND_SAFE_COND_WAIT: i32 = 77;
//     pub const LIND_SAFE_COND_BROADCAST: i32 = 78;
//     pub const LIND_SAFE_COND_SIGNAL: i32 = 79;
//     pub const LIND_SAFE_COND_TIMEDWAIT: i32 = 80;

//     pub const LIND_SAFE_SEM_INIT: i32 = 91;
//     pub const LIND_SAFE_SEM_WAIT: i32 = 92;
//     pub const LIND_SAFE_SEM_TRYWAIT: i32 = 93;
//     pub const LIND_SAFE_SEM_TIMEDWAIT: i32 = 94;
//     pub const LIND_SAFE_SEM_POST: i32 = 95;
//     pub const LIND_SAFE_SEM_DESTROY: i32 = 96;
//     pub const LIND_SAFE_SEM_GETVALUE: i32 = 97;

//     pub const LIND_SAFE_NET_GETHOSTNAME: i32 = 125;

//     pub const LIND_SAFE_FS_PREAD: i32 = 126;
//     pub const LIND_SAFE_FS_PWRITE: i32 = 127;
//     pub const LIND_SAFE_FS_CHDIR: i32 = 130;
//     pub const LIND_SAFE_FS_MKDIR: i32 = 131;
//     pub const LIND_SAFE_FS_RMDIR: i32 = 132;
//     pub const LIND_SAFE_FS_CHMOD: i32 = 133;
//     pub const LIND_SAFE_FS_FCHMOD: i32 = 134;

//     pub const LIND_SAFE_NET_SOCKET: i32 = 136;

//     pub const LIND_SAFE_NET_GETSOCKNAME: i32 = 144;
//     pub const LIND_SAFE_NET_GETPEERNAME: i32 = 145;
//     pub const LIND_SAFE_NET_GETIFADDRS: i32 = 146;
//     pub const LIND_SAFE_SYS_SIGACTION: i32 = 147;
//     pub const LIND_SAFE_SYS_KILL: i32 = 148;
//     pub const LIND_SAFE_SYS_SIGPROCMASK: i32 = 149;
//     pub const LIND_SAFE_SYS_LINDSETITIMER: i32 = 150;

//     pub const LIND_SAFE_FS_FCHDIR: i32 = 161;
//     pub const LIND_SAFE_FS_FSYNC: i32 = 162;
//     pub const LIND_SAFE_FS_FDATASYNC: i32 = 163;
//     pub const LIND_SAFE_FS_SYNC_FILE_RANGE: i32 = 164;

//     pub const LIND_SAFE_SYS_CLONE: i32 = 171;

//     #[repr(C)]
//     pub union RustArg {
//         pub dispatch_int: i32,
//         pub dispatch_uint: u32,
//         pub dispatch_intptr: *mut i32,
//         pub dispatch_ulong: u64,
//         pub dispatch_ulong_long: u64,
//         pub dispatch_long: i64,
//         pub dispatch_size_t: usize,
//         pub dispatch_ssize_t: ssize_t,
//         pub dispatch_off_t: off_t,
//         pub dispatch_socklen_t: socklen_t,
//         pub dispatch_socklen_t_ptr: *mut socklen_t,
//         pub dispatch_cbuf: *const c_void,
//         pub dispatch_mutcbuf: *mut c_void,
//         pub dispatch_cstr: *const c_char,
//         pub dispatch_cstrarr: *const *const c_char,
//         pub strdispatch_rlimitstruct: *mut rlimit, // Standard
//         // pub dispatch_statstruct: *mut lind_stat,   // Needs manual definition
//         pub dispatch_statfsstruct: *mut statfs,     // Standard
//         pub dispatch_timevalstruct: *mut timeval,   // Standard
//         pub dispatch_timespecstruct: *mut timespec, // Standard
//         pub dispatch_sockaddrstruct: *mut sockaddr, // Standard
//         // pub dispatch_epolleventstruct: *mut epoll_event, // Standard
//         pub dispatch_constsockaddrstruct: *mut sockaddr, // Standard
//         // pub dispatch_shmidstruct: *mut lind_shmid_ds, // Needs manual definition
//         pub dispatch_pipearray: *mut i32,
//         // pub dispatch_naclabisigactionstruct: *mut nacl_abi_sigaction, // Needs manual definition
//         // pub dispatch_constnaclabisigactionstruct: *const nacl_abi_sigaction, // Needs manual definition
//         // pub dispatch_naclsigset: *mut u64, // Use u64 in Rust
//         // pub dispatch_constnaclsigset: *const u64, // Use u64 in Rust
//         pub dispatch_structitimerval: *mut itimerval, // Standard
//         pub dispatch_conststructitimerval: *const itimerval, // Standard
//         pub fdset: *mut fd_set,
//     }

//     pub const BLANKARG: RustArg = RustArg { dispatch_ulong: 0 };

//     #[link(name = "rawposix")]
//     extern "C" {
//         pub(crate) fn dispatcher(
//             cageid: u64,
//             callnum: i32,
//             arg1: RustArg,
//             arg2: RustArg,
//             arg3: RustArg,
//             arg4: RustArg,
//             arg5: RustArg,
//             arg6: RustArg,
//         );

//         pub(crate) fn lindrustinit(verbosity: i32);

//         pub(crate) fn lindrustfinalize();

//         pub(crate) fn quick_write(fd: i32, buf: *const u8, count: usize, cageid: u64);

//         pub(crate) fn rustposix_thread_init(cageid: u64, signalflag: u64);

//         pub(crate) fn lind_syscall_api(
//             cageid: u64,
//             call_number: u32,
//             call_name: u64,
//             start_address: u64,
//             arg1: u64,
//             arg2: u64,
//             arg3: u64,
//             arg4: u64,
//             arg5: u64,
//             arg6: u64,
//         ) -> u32;
//     }
// }

use std::sync::{Condvar, Mutex};

// use crate::librawposix::*;
use crate::safeposix::dispatcher::*;

#[macro_export]
macro_rules! dispatch {
    ($cageid:expr, $callnum:expr) => {
        dispatcher(
            $cageid, $callnum, BLANKARG, BLANKARG, BLANKARG, BLANKARG, BLANKARG, BLANKARG,
        )
    };
    ($cageid:expr, $callnum:expr, $arg1:expr) => {
        dispatcher(
            $cageid, $callnum, $arg1, BLANKARG, BLANKARG, BLANKARG, BLANKARG, BLANKARG,
        )
    };
    ($cageid:expr, $callnum:expr, $arg1:expr, $arg2:expr) => {
        dispatcher(
            $cageid, $callnum, $arg1, $arg2, BLANKARG, BLANKARG, BLANKARG, BLANKARG,
        )
    };
    ($cageid:expr, $callnum:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {
        dispatcher(
            $cageid, $callnum, $arg1, $arg2, $arg3, BLANKARG, BLANKARG, BLANKARG,
        )
    };
    ($cageid:expr, $callnum:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        dispatcher(
            $cageid, $callnum, $arg1, $arg2, $arg3, $arg4, BLANKARG, BLANKARG,
        )
    };
    ($cageid:expr, $callnum:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        dispatcher(
            $cageid, $callnum, $arg1, $arg2, $arg3, $arg4, $arg5, BLANKARG,
        )
    };
    ($cageid:expr, $callnum:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {
        dispatcher($cageid, $callnum, $arg1, $arg2, $arg3, $arg4, $arg5, $arg6)
    };
}

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
        // dispatch!(
        //     cageid,
        //     crate::librawposix::LIND_SAFE_FS_WRITE,
        //     RustArg { dispatch_int: fd },
        //     RustArg { dispatch_cbuf: buf },
        //     RustArg {
        //         dispatch_size_t: count
        //     }
        // )
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
