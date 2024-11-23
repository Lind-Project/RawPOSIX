//! RawPOSIX Constants Module
//! 
//! Primary Source References:
//! - Linux kernel v6.5: include/uapi/asm-generic/mman-common.h
//! - Linux kernel v6.5: include/uapi/asm-generic/fcntl.h
//! - Linux kernel v6.5: include/uapi/linux/stat.h
//! - Linux kernel v6.5: include/uapi/asm-generic/errno-base.h
//! - POSIX.1-2017 (IEEE Std 1003.1-2017)

// Change these from private to public modules
pub mod fs_constants;
pub mod net_constants;
pub mod sys_constants;
pub mod vmmap_constants;

// Re-export all constants from their respective modules
pub use fs_constants::*;
pub use net_constants::*;
pub use sys_constants::*;
pub use vmmap_constants::*;

// ===== Memory Protection Flags =====
// Source: include/uapi/asm-generic/mman-common.h
// These flags control memory segment permissions for mmap and mprotect
pub const PROT_NONE: i32 = 0x0;    // No permissions - pages may not be accessed
pub const PROT_READ: i32 = 0x1;    // Pages may be read
pub const PROT_WRITE: i32 = 0x2;   // Pages may be written
pub const PROT_EXEC: i32 = 0x4;    // Pages may be executed
pub const PROT_MASK: u32 = 0x7;    // Mask for all permission bits
                                   // Note: Some platforms may support additional bits

// ===== Memory Mapping Flags =====
// Source: include/uapi/asm-generic/mman.h, include/uapi/asm-generic/mman-common.h
// These flags control the behavior of mmap operations
pub const MAP_SHARED: i32 = 0x01;         // Share changes with other processes
pub const MAP_PRIVATE: i32 = 0x02;        // Changes are private to this process
pub const MAP_FIXED: i32 = 0x10;          // Interpret address exactly
pub const MAP_ANON: i32 = 0x20;           // Don't use a file descriptor
pub const MAP_ANONYMOUS: i32 = MAP_ANON;   // Synonym for MAP_ANON
pub const MAP_SHARING_MASK: i32 = 0x03;    // Mask to isolate sharing bits
                                          // Note: Values may differ on BSD systems

// ===== Memory Mapping Error Values =====
// Source: include/uapi/asm-generic/mman-common.h
// Standard error return value for mmap
pub const MAP_FAILED: *mut std::ffi::c_void = (-1_isize as *mut std::ffi::c_void);

// ===== Memory Page Size Constants =====
// Note: These values are architecture-dependent
// Current values are for x86_64 Linux
pub const PAGESHIFT: u32 = 12;           // 4KB pages (1 << 12 = 4096)
pub const PAGESIZE: u32 = 1 << PAGESHIFT;
pub const MAP_PAGESHIFT: u32 = 16;       // Custom value for Lind
pub const MAP_PAGESIZE: u32 = 1 << MAP_PAGESHIFT;

// ===== Memory Remapping Flags =====
// Source: include/uapi/asm-generic/mman-common.h
pub const MREMAP_MAYMOVE: u32 = 0x01;    // Can relocate mapping
pub const MREMAP_FIXED: u32 = 0x02;      // New address is specified exactly

// ===== File Access Modes =====
// Source: include/uapi/asm-generic/fcntl.h
// These flags control how files are opened and accessed
pub const O_ACCMODE: i32 = 0o003;    // Mask for file access modes
pub const O_RDONLY: i32 = 0o0;       // Open read-only
pub const O_WRONLY: i32 = 0o1;       // Open write-only
pub const O_RDWR: i32 = 0o2;         // Open read-write
pub const O_CREAT: i32 = 0o100;      // Create file if it doesn't exist
pub const O_EXCL: i32 = 0o200;       // Error if O_CREAT and file exists
pub const O_TRUNC: i32 = 0o1000;     // Truncate file to zero length
pub const O_APPEND: i32 = 0o2000;    // Append mode - writes always at end

// ===== File Types =====
// Source: include/uapi/linux/stat.h
// These constants define the different types of files in the filesystem
pub const S_IFMT: u32 = 0o170000;    // Bit mask for file type
pub const S_IFREG: u32 = 0o100000;   // Regular file
pub const S_IFDIR: u32 = 0o040000;   // Directory
pub const S_IFCHR: u32 = 0o020000;   // Character device
pub const S_IFBLK: u32 = 0o060000;   // Block device
pub const S_IFIFO: u32 = 0o010000;   // FIFO (named pipe)
pub const S_IFLNK: u32 = 0o120000;   // Symbolic link
pub const S_IFSOCK: u32 = 0o140000;  // Socket

// ===== File Permissions =====
// Source: include/uapi/linux/stat.h
// Standard UNIX permission bits
pub const S_IRWXU: u32 = 0o700;  // User read, write, execute
pub const S_IRUSR: u32 = 0o400;  // User read
pub const S_IWUSR: u32 = 0o200;  // User write
pub const S_IXUSR: u32 = 0o100;  // User execute

pub const S_IRWXG: u32 = 0o070;  // Group read, write, execute
pub const S_IRGRP: u32 = 0o040;  // Group read
pub const S_IWGRP: u32 = 0o020;  // Group write
pub const S_IXGRP: u32 = 0o010;  // Group execute

pub const S_IRWXO: u32 = 0o007;  // Others read, write, execute
pub const S_IROTH: u32 = 0o004;  // Others read
pub const S_IWOTH: u32 = 0o002;  // Others write
pub const S_IXOTH: u32 = 0o001;  // Others execute

// ===== Error Numbers =====
// Source: include/uapi/asm-generic/errno-base.h
// Standard POSIX error codes
pub const EPERM: i32 = 1;        // Operation not permitted
pub const ENOENT: i32 = 2;       // No such file or directory
pub const EINTR: i32 = 4;        // Interrupted system call
pub const EIO: i32 = 5;          // I/O error
pub const EBADF: i32 = 9;        // Bad file descriptor
pub const EAGAIN: i32 = 11;      // Resource temporarily unavailable
pub const ENOMEM: i32 = 12;      // Cannot allocate memory
pub const EACCES: i32 = 13;      // Permission denied
pub const EFAULT: i32 = 14;      // Bad address
pub const EEXIST: i32 = 17;      // File exists
pub const ENOTDIR: i32 = 20;     // Not a directory
pub const EINVAL: i32 = 22;      // Invalid argument
