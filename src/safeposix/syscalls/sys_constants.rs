// System related constants
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::interface;

// Define constants using static or const
// Imported into fs_calls file

//GID AND UID DEFAULT VALUES

pub const DEFAULT_UID: u32 = 1000;
pub const DEFAULT_GID: u32 = 1000;

// RESOURCE LIMITS

pub const SIGNAL_MAX: i32 = 64;

pub const NOFILE_CUR: u64 = 1024;
pub const NOFILE_MAX: u64 = 4 * 1024;

pub const STACK_CUR: u64 = 8192 * 1024;
pub const STACK_MAX: u64 = 1 << 32;

pub const RLIMIT_STACK: u64 = 0;
pub const RLIMIT_NOFILE: u64 = 1;

// Constants for exit_syscall status

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;

// Signal Table (x86/ARM)
// Based on https://man7.org/linux/man-pages/man7/signal.7.html
pub const SIGHUP: i32 = 1;
pub const SIGINT: i32 = 2;
pub const SIGQUIT: i32 = 3;
pub const SIGILL: i32 = 4;
pub const SIGTRAP: i32 = 5;
pub const SIGABRT: i32 = 6;
pub const SIGIOT: i32 = 6;
pub const SIGBUS: i32 = 7;
// pub const SIGEMT: i32
pub const SIGFPE: i32 = 8;
pub const SIGKILL: i32 = 9;
pub const SIGUSR1: i32 = 10;
pub const SIGSEGV: i32 = 11;
pub const SIGUSR2: i32 = 12;
pub const SIGPIPE: i32 = 13;
pub const SIGALRM: i32 = 14;
pub const SIGTERM: i32 = 15;
pub const SIGSTKFLT: i32 = 16;
pub const SIGCHLD: i32 = 17;
// pub const SIGCLD: i32
pub const SIGCONT: i32 = 18;
pub const SIGSTOP: i32 = 19;
pub const SIGTSTP: i32 = 20;
pub const SIGTTIN: i32 = 21;
pub const SIGTTOU: i32 = 22;
pub const SIGURG: i32 = 23;
pub const SIGXCPU: i32 = 24;
pub const SIGXFSZ: i32 = 25;
pub const SIGVTALRM: i32 = 26;
pub const SIGPROF: i32 = 27;
pub const SIGWINCH: i32 = 28;
pub const SIGIO: i32 = 29;
pub const SIGPOLL: i32 = 29;
pub const SIGPWR: i32 = 30;
// pub const SIGINFO: i32
// pub const SIGLOST: i32
pub const SIGSYS: i32 = 31;
pub const SIGUNUSED: i32 = 31;

pub const SIG_BLOCK: i32 = 0;
pub const SIG_UNBLOCK: i32 = 1;
pub const SIG_SETMASK: i32 = 2;
pub const ITIMER_REAL: i32 = 0;

// /* Cloning flags.  */
// pub const CSIGNAL: u64 =       0x000000ff; /* Signal mask to be sent at exit.  */
// pub const CLONE_VM: u64 =      0x00000100; /* Set if VM shared between processes.  */
// pub const CLONE_FS: u64 =      0x00000200; /* Set if fs info shared between processes.  */
// pub const CLONE_FILES: u64 =   0x00000400; /* Set if open files shared between processes.  */
// pub const CLONE_SIGHAND: u64 = 0x00000800; /* Set if signal handlers shared.  */
// pub const CLONE_PIDFD: u64 =   0x00001000; /* Set if a pidfd should be placed in parent.  */
// pub const CLONE_PTRACE: u64 =  0x00002000; /* Set if tracing continues on the child.  */
// pub const CLONE_VFORK: u64 =   0x00004000; /* Set if the parent wants the child to wake it up on mm_release.  */
// pub const CLONE_PARENT: u64 =  0x00008000; /* Set if we want to have the same parent as the cloner.  */
// pub const CLONE_THREAD: u64 =  0x00010000; /* Set to add to same thread group.  */
// pub const CLONE_NEWNS: u64 =   0x00020000; /* Set to create new namespace.  */
// pub const CLONE_SYSVSEM: u64 = 0x00040000; /* Set to shared SVID SEM_UNDO semantics.  */
// pub const CLONE_SETTLS: u64 =  0x00080000; /* Set TLS info.  */
// pub const CLONE_PARENT_SETTID: u64 = 0x00100000; /* Store TID in userlevel buffer before MM copy.  */
// pub const CLONE_CHILD_CLEARTID: u64 = 0x00200000; /* Register exit futex and memory location to clear.  */
// pub const CLONE_DETACHED: u64 = 0x00400000; /* Create clone detached.  */
// pub const CLONE_UNTRACED: u64 = 0x00800000; /* Set if the tracing process can't force CLONE_PTRACE on this clone.  */
// pub const CLONE_CHILD_SETTID: u64 = 0x01000000; /* Store TID in userlevel buffer in the child.  */
// pub const CLONE_NEWCGROUP: u64 =    0x02000000;	/* New cgroup namespace.  */
// pub const CLONE_NEWUTS: u64 =	0x04000000;	/* New utsname group.  */
// pub const CLONE_NEWIPC: u64 =	0x08000000;	/* New ipcs.  */
// pub const CLONE_NEWUSER: u64 =	0x10000000;	/* New user namespace.  */
// pub const CLONE_NEWPID: u64 =	0x20000000;	/* New pid namespace.  */
// pub const CLONE_NEWNET: u64 =	0x40000000;	/* New network namespace.  */
// pub const CLONE_IO: u64 =	0x80000000;	/* Clone I/O context.  */
// /* cloning flags intersect with CSIGNAL so can be used only with unshare and
//    clone3 syscalls.  */
// pub const CLONE_NEWTIME: u64 =	0x00000080;      /* New time namespace */