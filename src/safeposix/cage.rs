#![allow(dead_code)]
use crate::interface;
//going to get the datatypes and errnos from the cage file from now on
pub use crate::interface::errnos::{syscall_error, Errno};

pub use crate::interface::types::{
    Arg, EpollEvent, IoctlPtrUnion, PipeArray, PollStruct,
};

use super::filesystem::normpath;
// use super::net::SocketHandle;
pub use super::syscalls::fs_constants::*;
pub use super::syscalls::net_constants::*;
pub use super::syscalls::sys_constants::*;
pub use super::syscalls::vmmap::*;

pub use crate::interface::CAGE_TABLE;

// pub type FdTable = Vec<interface::RustRfc<interface::RustLock<Option<FileDescriptor>>>>;

#[derive(Debug)]
pub struct Cage {
    pub cageid: u64,
    pub cwd: interface::RustLock<interface::RustRfc<interface::RustPathBuf>>,
    pub parent: u64,
    pub cancelstatus: interface::RustAtomicBool,
    pub getgid: interface::RustAtomicI32,
    pub getuid: interface::RustAtomicI32,
    pub getegid: interface::RustAtomicI32,
    pub geteuid: interface::RustAtomicI32,
    pub rev_shm: interface::Mutex<Vec<(u32, i32)>>, //maps addr within cage to shmid
    pub mutex_table: interface::RustLock<Vec<Option<interface::RustRfc<interface::RawMutex>>>>,
    pub cv_table: interface::RustLock<Vec<Option<interface::RustRfc<interface::RawCondvar>>>>,
    pub sem_table: interface::RustHashMap<u32, interface::RustRfc<interface::RustSemaphore>>,
    pub thread_table: interface::RustHashMap<u64, bool>,
    pub signalhandler: interface::RustHashMap<i32, interface::SigactionStruct>,
    pub sigset: interface::RustHashMap<u64, interface::RustAtomicU64>,
    pub pendingsigset: interface::RustHashMap<u64, interface::RustAtomicU64>,
    pub main_threadid: interface::RustAtomicU64,
    pub interval_timer: interface::IntervalTimer,
    pub vmmap:  Vmmap,
}

impl Cage {
    pub fn changedir(&self, newdir: interface::RustPathBuf) {
        let newwd = interface::RustRfc::new(normpath(newdir, self));
        let mut cwdbox = self.cwd.write();
        *cwdbox = newwd;
    }

    // function to signal all cvs in a cage when forcing exit
    pub fn signalcvs(&self) {
        let cvtable = self.cv_table.read();

        for cv_handle in 0..cvtable.len() {
            if cvtable[cv_handle as usize].is_some() {
                let clonedcv = cvtable[cv_handle as usize].as_ref().unwrap().clone();
                clonedcv.broadcast();
            }
        }
    }

    pub fn send_pending_signals(&self, sigset: interface::SigsetType, pthreadid: u64) {
        for signo in 1..SIGNAL_MAX {
            if interface::lind_sigismember(sigset, signo) {
                interface::lind_threadkill(pthreadid, signo);
            }
        }
    }

}

// pub fn init_fdtable() -> FdTable {
//     let mut fdtable = Vec::new();
//     // load lower handle stubs
//     let stdin = interface::RustRfc::new(interface::RustLock::new(Some(FileDescriptor::Stream(
//         StreamDesc {
//             position: 0,
//             stream: 0,
//             flags: O_RDONLY,
//             advlock: interface::RustRfc::new(interface::AdvisoryLock::new()),
//         },
//     ))));
//     let stdout = interface::RustRfc::new(interface::RustLock::new(Some(FileDescriptor::Stream(
//         StreamDesc {
//             position: 0,
//             stream: 1,
//             flags: O_WRONLY,
//             advlock: interface::RustRfc::new(interface::AdvisoryLock::new()),
//         },
//     ))));
//     let stderr = interface::RustRfc::new(interface::RustLock::new(Some(FileDescriptor::Stream(
//         StreamDesc {
//             position: 0,
//             stream: 2,
//             flags: O_WRONLY,
//             advlock: interface::RustRfc::new(interface::AdvisoryLock::new()),
//         },
//     ))));
//     fdtable.push(stdin);
//     fdtable.push(stdout);
//     fdtable.push(stderr);

//     for _fd in 3..MAXFD as usize {
//         fdtable.push(interface::RustRfc::new(interface::RustLock::new(None)));
//     }
//     fdtable
// }

// pub fn create_unix_sockpipes() -> (
//     interface::RustRfc<interface::EmulatedPipe>,
//     interface::RustRfc<interface::EmulatedPipe>,
// ) {
//     let pipe1 = interface::RustRfc::new(interface::new_pipe(UDSOCK_CAPACITY));
//     let pipe2 = interface::RustRfc::new(interface::new_pipe(UDSOCK_CAPACITY));

//     (pipe1, pipe2)
// }
