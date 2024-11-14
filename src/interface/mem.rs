use crate::safeposix::cage::{Vmmap, MAP_SHARED, PAGESHIFT, PAGESIZE, PROT_NONE, PROT_READ, PROT_WRITE};

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

    for (_interval, entry) in parent_vmmap.entries.iter() {
        if entry.prot == PROT_NONE { continue; }
        let addr_st = (entry.page_num << PAGESHIFT) as i32;
        let addr_len = (entry.npages << PAGESHIFT) as usize;
        println!("vmmap fork: copy {}-{}\n", addr_st, addr_st + addr_len as i32);

        let parent_st = parent_vmmap.user_to_sys(addr_st);
        let child_st = child_vmmap.user_to_sys(addr_st);
        if entry.flags & (MAP_SHARED as i32) > 0 {
            let result = unsafe { libc::mremap(parent_st as *mut libc::c_void, 0, addr_len, libc::MREMAP_MAYMOVE | libc::MREMAP_FIXED, child_st as *mut libc::c_void) };
        } else {
            // temporarily enable write on child's memory region to write parent data
            unsafe { libc::mprotect(child_st as *mut libc::c_void, addr_len, PROT_READ | PROT_WRITE) };

            // write parent data
            unsafe { std::ptr::copy_nonoverlapping(parent_st as *const u8, child_st as *mut u8, addr_len) };

            // revert child's memory region prot
            unsafe { libc::mprotect(child_st as *mut libc::c_void, addr_len, entry.prot) };
        }
        // println!("done fork entry");
    }
}
