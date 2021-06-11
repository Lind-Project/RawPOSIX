#![allow(dead_code)]
#![allow(unused_variables)]
// retreive cage table
const ACCESS_SYSCALL: i32 = 2;
const UNLINK_SYSCALL: i32 = 4;
const LINK_SYSCALL: i32 = 5;
const CHDIR_SYSCALL: i32 = 6;
const MKDIR_SYSCALL: i32 = 7;
const RMDIR_SYSCALL: i32 = 8;
const XSTAT_SYSCALL: i32 = 9;
const OPEN_SYSCALL: i32 = 10;
const CLOSE_SYSCALL: i32 = 11;
const READ_SYSCALL: i32 = 12;
const WRITE_SYSCALL: i32 = 13;
const LSEEK_SYSCALL: i32 = 14;
const IOCTL_SYSCALL: i32 = 15;
const FXSTAT_SYSCALL: i32 = 17;
const FSTATFS_SYSCALL: i32 = 19;
const MMAP_SYSCALL: i32 = 21;
const MUNMAP_SYSCALL: i32 = 22;
const GETDENTS_SYSCALL: i32 = 23;
const DUP_SYSCALL: i32 = 24;
const DUP2_SYSCALL: i32 = 25;
const STATFS_SYSCALL: i32 = 26;
const FCNTL_SYSCALL: i32 = 28;
const GETPPID_SYSCALL: i32 = 29;
const EXIT_SYSCALL: i32 = 30;
const GETPID_SYSCALL: i32 = 31;
const SOCKET_SYSCALL: i32 = 32;
const BIND_SYSCALL: i32 = 33;
const SEND_SYSCALL: i32 = 34;
const SENDTO_SYSCALL: i32 = 35;
const RECV_SYSCALL: i32 = 36;
const RECVFROM_SYSCALL: i32 = 37;
const CONNECT_SYSCALL: i32 = 38;
const LISTEN_SYSCALL: i32 = 39;
const ACCEPT_SYSCALL: i32 = 40;
const GETPEERNAME_SYSCALL: i32 = 41;
const GETSOCKNAME_SYSCALL: i32 = 42;
const GETSOCKOPT_SYSCALL: i32 = 43;
const SETSOCKOPT_SYSCALL: i32 = 44;
const SHUTDOWN_SYSCALL: i32 = 45;
const SELECT_SYSCALL: i32 = 46;
const GETIFADDRS_SYSCALL: i32 = 47;
const POLL_SYSCALL: i32 = 48;
const SOCKETPAIR_SYSCALL: i32 = 49;
const GETUID_SYSCALL: i32 = 50;
const GETEUID_SYSCALL: i32 = 51;
const GETGID_SYSCALL: i32 = 52;
const GETEGID_SYSCALL: i32 = 53;
const FLOCK_SYSCALL: i32 = 54;
const RENAME_SYSCALL: i32 = 55;
const EPOLL_CREATE_SYSCALL: i32 = 56;
const EPOLL_CTL_SYSCALL: i32 = 57;
const EPOLL_WAIT_SYSCALL: i32 = 58;

const PIPE_SYSCALL: i32 = 66;
const PIPE2_SYSCALL: i32 = 67;
const FORK_SYSCALL: i32 = 68;
const EXEC_SYSCALL: i32 = 69;

const GETHOSTNAME_SYSCALL: i32 = 125;
const PREAD_SYSCALL: i32 = 126;
const PWRITE_SYSCALL: i32 = 127;


use crate::interface;
use super::cage::{CAGE_TABLE, Cage};


#[repr(C)]
pub union Arg {
  int: i32,
  ulong: u64,
  cstr: *const u8,
  cstrarr: *const *const u8,
}

pub static THREAD2CAGEID: interface::RustLazyGlobal<interface::RustLock<interface::RustHashMap<interface::RustThreadId, u64>>> = interface::RustLazyGlobal::new(|| interface::RustLock::new(interface::new_hashmap()));

pub extern "C" fn dispatcher(callnum: i32, arg1: Arg, arg2: Arg, arg3: Arg, arg4: Arg, arg5: Arg, arg6: Arg) -> i32 {
    
    let threadnum = interface::rust_gettid().id();
    let cageid = { THREAD2CAGEID.read().unwrap().get(&threadnum).unwrap().clone() };

    // need to match based on if cage exists
    let cage = { CAGE_TABLE.read().unwrap().get(&cageid).unwrap().clone() };

    //implement syscall method calling using matching
    // match based on syscall, take cagetable lock as read for all except fork/exec, which need write

    match callnum {
      EXIT_SYSCALL => {
        0
      },
      EXEC_SYSCALL => {
        {
          let mut t2cid = THREAD2CAGEID.write().unwrap();
          t2cid.insert(threadnum, unsafe{arg1.ulong});
        }
        cage.exec_syscall(unsafe{arg1.ulong});
        0
      }
      FORK_SYSCALL => {
        //do something with threadid 2 cageid, but we don't know the threadid in advance?? 
        //dispatch to a register cageid kind of thing further on in fork?
        cage.fork_syscall(unsafe{arg1.ulong});
        0
      },
      _ => {//unknown syscall
        -1
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn cagetest() {
      let threadnum = interface::rust_gettid().id();
      {THREAD2CAGEID.write().unwrap().insert(threadnum, 1_u64);};

      {CAGE_TABLE.write().unwrap().insert(1, interface::RustRfc::new(Cage{cageid: 1, cwd:"/".to_string(), parent: 0, filedescriptortable: interface::RustLock::new(interface::RustHashMap::new())}));}
      {println!("{:?}", CAGE_TABLE.read().unwrap());};
      {println!("{}", interface::RustRfc::strong_count(CAGE_TABLE.read().unwrap().get(&1_u64).unwrap()));}
      dispatcher(FORK_SYSCALL, Arg {ulong: 2_u64}, Arg {int: 34132}, Arg {int: 109384}, Arg {int: -12341}, Arg {int: -12341}, Arg {int: 0});
      {println!("{:?}", CAGE_TABLE.read().unwrap());};
      dispatcher(EXEC_SYSCALL, Arg {ulong: 7_u64}, Arg {int: 34132}, Arg {int: 109384}, Arg {int: -12341}, Arg {int: -12341}, Arg {int: 0});
      {println!("{:?}", CAGE_TABLE.read().unwrap());};
      
    }
}
