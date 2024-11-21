use libc::{MAP_ANONYMOUS, MAP_FIXED, MAP_PRIVATE, PROT_EXEC};

use crate::safeposix::cage::{MemoryBackingType, Vmmap, VmmapOps, MAP_SHARED, PAGESHIFT, PAGESIZE, PROT_NONE, PROT_READ, PROT_WRITE};

use crate::interface::{cagetable_getref, syscall_error, Errno};

pub fn round_up_page(length: u64) -> u64 {
    if length % PAGESIZE as u64 == 0 {
        length
    } else {
        ((length / PAGESIZE as u64) + 1) * PAGESIZE as u64
    }
}

pub fn fork_vmmap(parent_vmmap: &Vmmap, child_vmmap: &Vmmap) {
    let parent_base = parent_vmmap.base_address.unwrap();
    let child_base = child_vmmap.base_address.unwrap();

    // iterate through each vmmap entry
    for (_interval, entry) in parent_vmmap.entries.iter() {
        // if the entry has PROT_NONE, that means the entry is currently not used
        if entry.prot == PROT_NONE { continue; }
        // translate page number to user address
        let addr_st = (entry.page_num << PAGESHIFT) as i32;
        let addr_len = (entry.npages << PAGESHIFT) as usize;

        // translate user address to system address
        let parent_st = parent_vmmap.user_to_sys(addr_st);
        let child_st = child_vmmap.user_to_sys(addr_st);
        if entry.flags & (MAP_SHARED as i32) > 0 {
            // for shared memory, we are using mremap to fork shared memory
            let result = unsafe { libc::mremap(parent_st as *mut libc::c_void, 0, addr_len, libc::MREMAP_MAYMOVE | libc::MREMAP_FIXED, child_st as *mut libc::c_void) };
        } else {
            // temporarily enable write on child's memory region to write parent data
            unsafe { libc::mprotect(child_st as *mut libc::c_void, addr_len, PROT_READ | PROT_WRITE) };

            // write parent data
            // TODO: replace copy_nonoverlapping with writev for potential performance boost
            unsafe { std::ptr::copy_nonoverlapping(parent_st as *const u8, child_st as *mut u8, addr_len) };

            // revert child's memory region prot
            unsafe { libc::mprotect(child_st as *mut libc::c_void, addr_len, entry.prot) };
        }
    }
}

pub fn munmap(cageid: u64, addr: *mut u8, len: usize) -> i32 {
    let cage = cagetable_getref(cageid);

    // check if the provided address is multiple of pages
    let rounded_addr = round_up_page(addr as u64) as usize;
    if rounded_addr != addr as usize {
        return syscall_error(Errno::EINVAL, "mmap", "address it not aligned");
    }

    let vmmap = cage.vmmap.read();
    let sysaddr = vmmap.user_to_sys(rounded_addr as i32);
    drop(vmmap);

    let rounded_length = round_up_page(len as u64) as usize;

    let result = cage.mmap_syscall(sysaddr as *mut u8, rounded_length, PROT_NONE, (MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED) as i32, -1, 0);
    if result as usize != rounded_addr {
        panic!("MAP_FIXED not fixed");
    }

    let mut vmmap = cage.vmmap.write();

    vmmap.remove_entry(rounded_addr as u32 >> PAGESHIFT, len as u32 >> PAGESHIFT);

    0
}

pub fn mmap(cageid: u64, addr: *mut u8, len: usize, mut prot: i32, mut flags: i32, mut fildes: i32, off: i64) -> i32 {
    let cage = cagetable_getref(cageid);

    // only these four flags are allowed
    let allowed_flags = (MAP_FIXED as i32 | MAP_SHARED as i32 | MAP_PRIVATE as i32 | MAP_ANONYMOUS as i32);
    if flags & !allowed_flags > 0 {
        // truncate flag to remove flags that are not allowed
        flags &= allowed_flags;
    }

    if prot & PROT_EXEC > 0 {
        return syscall_error(Errno::EINVAL, "mmap", "PROT_EXEC is not allowed");
    }

    // check if the provided address is multiple of pages
    let rounded_addr = round_up_page(addr as u64);
    if rounded_addr != addr as u64 {
        return syscall_error(Errno::EINVAL, "mmap", "address it not aligned");
    }

    // offset should be non-negative and multiple of pages
    if off < 0 {
        return syscall_error(Errno::EINVAL, "mmap", "offset cannot be negative");
    }
    let rounded_off = round_up_page(off as u64);
    if rounded_off != off as u64 {
        return syscall_error(Errno::EINVAL, "mmap", "offset it not aligned");
    }

    // round up length to be multiple of pages
    let rounded_length = round_up_page(len as u64);

    let mut useraddr = addr as i32;
    // if MAP_FIXED is not set, then we need to find an address for the user
    if flags & MAP_FIXED as i32 == 0 {
        let mut vmmap = cage.vmmap.write();
        let result;
        
        // pick an address of appropriate size, anywhere
        if addr as usize == 0 {
            result = vmmap.find_map_space(rounded_length as u32 >> PAGESHIFT, 1);
        } else {
            // use address user provided as hint to find address
            result = vmmap.find_map_space_with_hint(rounded_length as u32 >> PAGESHIFT, 1, addr as u32);
        }

        // did not find desired memory region
        if result.is_none() {
            return syscall_error(Errno::ENOMEM, "mmap", "no memory");
        }

        let space = result.unwrap();
        useraddr = (space.start() << PAGESHIFT) as i32;
    }

    // TODO: validate useraddr (like checking whether within the program break)

    flags |= MAP_FIXED as i32;

    // either MAP_PRIVATE or MAP_SHARED should be set, but not both
    if (flags & MAP_PRIVATE as i32 == 0) == (flags & MAP_SHARED as i32 == 0) {
        return syscall_error(Errno::EINVAL, "mmap", "invalid flags");
    }

    let vmmap = cage.vmmap.read();

    let sysaddr = vmmap.user_to_sys(useraddr);

    drop(vmmap);

    if rounded_length > 0 {
        if flags & MAP_ANONYMOUS as i32 > 0 {
            fildes = -1;
        }

        let result = cage.mmap_syscall(sysaddr as *mut u8, rounded_length as usize, prot, flags, fildes, off);
        if result >= 0 {
            if result != useraddr {
                panic!("MAP_FIXED not fixed");
            }

            let mut vmmap = cage.vmmap.write();
            let backing = {
                MemoryBackingType::Anonymous
                // TODO: should return backing type accordingly
                // if flags & MAP_ANONYMOUS > 0 {
                //     MemoryBackingType::Anonymous
                // } else if flags & MAP_SHARED > 0 {

                // }
            };
            vmmap.add_entry_with_overwrite((useraddr >> PAGESHIFT) as u32, (rounded_length >> PAGESHIFT) as u32, prot, 0, flags, backing, off, 0, cageid);
        }
    }

    useraddr
}