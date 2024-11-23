//! Virtual Memory Mapping Constants Module
//! These constants define virtual memory mapping flags and parameters
//! 
//! Primary Source References:
//! - Linux kernel v6.5: include/uapi/asm-generic/mman.h
//! - Linux kernel v6.5: include/uapi/asm-generic/mman-common.h
//! - POSIX.1-2017 (IEEE Std 1003.1-2017)

// ===== Memory Protection Flags =====
// Source: include/uapi/asm-generic/mman-common.h
pub const PROT_NONE: i32 = 0x0;    // Page cannot be accessed
pub const PROT_READ: i32 = 0x1;    // Page can be read
pub const PROT_WRITE: i32 = 0x2;   // Page can be written
pub const PROT_EXEC: i32 = 0x4;    // Page can be executed

// Mask for all protection bits
// Note: Some architectures may support additional bits
pub const PROT_MASK: u32 = 0x7;

// ===== Memory Mapping Flags =====
// Source: include/uapi/asm-generic/mman.h
pub const MAP_SHARED: u32 = 0x01;   // Share changes with other processes
pub const MAP_PRIVATE: u32 = 0x02;  // Changes are private to this process
pub const MAP_SHARING_MASK: u32 = 0x03;  // Mask to isolate sharing bits

pub const MAP_FIXED: u32 = 0x10;    // Interpret address exactly
pub const MAP_ANON: u32 = 0x20;     // Don't use a file descriptor
pub const MAP_ANONYMOUS: u32 = MAP_ANON;  // Linux alias for MAP_ANON

// ===== Page Size Constants =====
// Note: These values are architecture-dependent
// Current values are for x86_64 Linux
pub const PAGESHIFT: u32 = 12;           // 4KB pages (1 << 12 = 4096)
pub const PAGESIZE: u32 = 1 << PAGESHIFT;

// Lind-specific page size constants
pub const MAP_PAGESHIFT: u32 = 16;       // Custom value for Lind
pub const MAP_PAGESIZE: u32 = 1 << MAP_PAGESHIFT;

// ===== Memory Mapping Error Value =====
// Source: include/uapi/asm-generic/mman-common.h
pub const MAP_FAILED: *mut std::ffi::c_void = (-1isize) as *mut std::ffi::c_void;

// ===== Memory Remapping Flags =====
// Source: include/uapi/asm-generic/mman-common.h
pub const MREMAP_MAYMOVE: u32 = 0x01;  // Can relocate mapping
pub const MREMAP_FIXED: u32 = 0x02;    // New address is specified exactly

// ===== File Access Modes =====
// Source: include/uapi/asm-generic/fcntl.h
// NOTE: These should probably be moved to fs_constants.rs
pub const O_ACCMODE: i32 = 0o003;  // Mask for file access modes
pub const O_RDONLY: i32 = 0o0;     // Open read-only
pub const O_WRONLY: i32 = 0o1;     // Open write-only
pub const O_RDWR: i32 = 0o2;       // Open read-write
