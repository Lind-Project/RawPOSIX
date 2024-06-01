//  DashMap<u64,[Option<FDTableEntry>;FD_PER_PROCESSS_MAX]>  Space is ~24KB 
//  per cage w/ 1024 fds?!?
//      Static DashMap.  Let's see if having the FDTableEntries be a static
//      array is any faster...

use crate::safeposix::cage;
use crate::safeposix::syscalls::fs_calls::*;

use super::threei;

use dashmap::DashMap;

use lazy_static::lazy_static;

use std::collections::HashMap;

use std::sync::Mutex;

use std::io::Write;
use std::io;

// This uses a Dashmap (for cages) with an array of FDTableEntry items.

// Get constants about the fd table sizes, etc.
pub use super::commonconstants::*;

// algorithm name.  Need not be listed.  Used in benchmarking output
// #[doc(hidden)]
pub const ALGONAME: &str = "DashMapArrayGlobal";

// These are the values we look up with at the end...
// #[doc = include_str!("../docs/fdtableentry.md")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FDTableEntry {
    pub realfd: i32, // underlying fd (may be a virtual fd below us or
    // a kernel fd)
    pub should_cloexec: bool, // should I close this when exec is called?
    pub optionalinfo: i32,    // user specified / controlled data
}

// It's fairly easy to check the fd count on a per-process basis (I just check
// when I would add a new fd).
//
// BUG: I will ignore the total limit for now.  I would ideally do this on
// every creation, close, fork, etc. but it's a PITA to track this.

// We will raise a panic anywhere we receive an unknown cageid.  This frankly
// should not be possible and indicates some sort of internal error in our
// code.  However, it is expected we could receive an invalid file descriptor
// when a cage makes a call.

// In order to store this information, I'm going to use a DashMap which
// has keys of (cageid:u64) and values that are an array of FD_PER_PROCESS_MAX
// Option<FDTableEntry> items. 
//
//

// This lets me initialize the code as a global.
lazy_static! {

  #[derive(Debug)]
  pub static ref FDTABLE: DashMap<u64, [Option<FDTableEntry>;FD_PER_PROCESS_MAX as usize]> = {
    let m = DashMap::new();
    // Insert a cage so that I have something to fork / test later, if need
    // be. Otherwise, I'm not sure how I get this started. I think this
    // should be invalid from a 3i standpoint, etc. Could this mask an
    // error in the future?
    // m.insert(threei::TESTING_CAGEID,[Option::None;FD_PER_PROCESS_MAX as usize]);
    m
  };
}

lazy_static! {
    // This is needed for close and similar functionality.  I need track the
    // number of times a realfd is open
    #[derive(Debug)]
    // REALFDCOUNT = <realfd, count>
    pub static ref REALFDCOUNT: DashMap<i32, u64> = {
        DashMap::new()
    };

}

// Internal helper to hold the close handlers...
pub struct CloseHandlers {
    pub intermediate_handler: fn(i32),
    pub final_handler: fn(i32),
    pub unreal_handler: fn(i32),
}

// Seems sort of like a constant...  I'm not sure if this is bad form or not...
#[allow(non_snake_case)]
pub fn NULL_FUNC(_:i32) { }

lazy_static! {
    // This holds the user registered handlers they want to have called when
    // a close occurs.  I did this rather than return messy data structures
    // from the close, exec, and exit handlers because it seemed cleaner...
    #[derive(Debug)]
    pub static ref CLOSEHANDLERTABLE: Mutex<CloseHandlers> = {
        let c = CloseHandlers {
            intermediate_handler:NULL_FUNC, 
            // final_handler:NULL_FUNC,
            final_handler:kernel_close,
            unreal_handler:NULL_FUNC,
        };
        Mutex::new(c)
    };
}


// #[doc = include_str!("../docs/translate_virtual_fd.md")]
pub fn translate_virtual_fd(cageid: u64, virtualfd: i32) -> Result<i32, threei::RetVal> {

    // They should not be able to pass a new cage I don't know.  I should
    // always have a table for each cage because each new cage is added at fork
    // time
    if !FDTABLE.contains_key(&cageid) {
        panic!("Unknown cageid in fdtable access");
    }

    return match FDTABLE.get(&cageid).unwrap()[virtualfd as usize] {
        Some(tableentry) => Ok(tableentry.realfd),
        None => Err(threei::Errno::EBADFD as u64),
    };
}


// This is fairly slow if I just iterate sequentially through numbers.
// However there are not that many to choose from.  I could pop from a list
// or a set as well...  Likely the best solution is to keep a count of the
// largest fd handed out and to just use this until you wrap.  This will be
// super fast for a normal cage and will be correct in the weird case.
// Right now, I'll just implement the slow path and will speed this up
// later, if needed.
// #[doc = include_str!("../docs/get_unused_virtual_fd.md")]
pub fn get_unused_virtual_fd(
    cageid: u64,
    realfd: i32,
    should_cloexec: bool,
    optionalinfo: i32,
) -> Result<i32, threei::RetVal> {

    if !FDTABLE.contains_key(&cageid) {
        panic!("Unknown cageid in fdtable access");
    }
    // Set up the entry so it has the right info...
    // Note, a HashMap stores its data on the heap!  No need to box it...
    // https://doc.rust-lang.org/book/ch08-03-hash-maps.html#creating-a-new-hash-map
    let myentry = FDTableEntry {
        realfd,
        should_cloexec,
        optionalinfo,
    };

    let mut myfdarray = FDTABLE.get_mut(&cageid).unwrap();

    // Check the fds in order.
    for fdcandidate in 0..FD_PER_PROCESS_MAX {
        // FIXME: This is likely very slow.  Should do something smarter...
        if myfdarray[fdcandidate as usize].is_none() {
            // I just checked.  Should not be there...
            myfdarray[fdcandidate as usize] = Some(myentry);
            _increment_realfd(realfd);
            return Ok(fdcandidate);
        }
    }

    // I must have checked all fds and failed to find one open.  Fail!
    Err(threei::Errno::EMFILE as u64)
}

// This is used for things like dup2, which need a specific fd...
// NOTE: I will assume that the requested_virtualfd isn't used.  If it is, I
// will return ELIND
// #[doc = include_str!("../docs/get_specific_virtual_fd.md")]
pub fn get_specific_virtual_fd(
    cageid: u64,
    requested_virtualfd: i32,
    realfd: i32,
    should_cloexec: bool,
    optionalinfo: i32,
) -> Result<(), threei::RetVal> {

    if !FDTABLE.contains_key(&cageid) {
        panic!("Unknown cageid in fdtable access");
    }

    // If you ask for a FD number that is too large, I'm going to reject it.
    // Note that, I need to use the FD_PER_PROCESS_MAX setting because this
    // is also how I'm tracking how many values you have open.  If this
    // changed, then these constants could be decoupled...
    if requested_virtualfd > FD_PER_PROCESS_MAX {
        return Err(threei::Errno::EBADF as u64);
    }

    // Set up the entry so it has the right info...
    // Note, a HashMap stores its data on the heap!  No need to box it...
    // https://doc.rust-lang.org/book/ch08-03-hash-maps.html#creating-a-new-hash-map
    let myentry = FDTableEntry {
        realfd,
        should_cloexec,
        optionalinfo,
    };

    if FDTABLE
        .get(&cageid)
        .unwrap()[requested_virtualfd as usize].is_some()
    {
        Err(threei::Errno::ELIND as u64)
    } else {
        FDTABLE.get_mut(&cageid).unwrap()[requested_virtualfd as usize] = Some(myentry);
        _increment_realfd(realfd);
        Ok(())
    }
}

// We're just setting a flag here, so this should be pretty straightforward.
// #[doc = include_str!("../docs/set_cloexec.md")]
pub fn set_cloexec(cageid: u64, virtualfd: i32, is_cloexec: bool) -> Result<(), threei::RetVal> {

    if !FDTABLE.contains_key(&cageid) {
        panic!("Unknown cageid in fdtable access");
    }

    // return EBADFD, if the fd is missing...
    if FDTABLE.get(&cageid).unwrap()[virtualfd as usize].is_none() {
        return Err(threei::Errno::EBADFD as u64);
    }
    // Set the is_cloexec flag
    FDTABLE.get_mut(&cageid).unwrap()[virtualfd as usize].as_mut().unwrap().should_cloexec = is_cloexec;
    Ok(())
}

// Super easy, just return the optionalinfo field...
// #[doc = include_str!("../docs/get_optionalinfo.md")]
pub fn get_optionalinfo(cageid: u64, virtualfd: i32) -> Result<i32, threei::RetVal> {
    if !FDTABLE.contains_key(&cageid) {
        panic!("Unknown cageid in fdtable access");
    }

    return match FDTABLE.get(&cageid).unwrap()[virtualfd as usize] {
        Some(tableentry) => Ok(tableentry.optionalinfo),
        None => Err(threei::Errno::EBADFD as u64),
    };
}

// We're setting an opaque value here. This should be pretty straightforward.
// #[doc = include_str!("../docs/set_optionalinfo.md")]
pub fn set_optionalinfo(
    cageid: u64,
    virtualfd: i32,
    optionalinfo: i32,
) -> Result<(), threei::RetVal> {

    if !FDTABLE.contains_key(&cageid) {
        panic!("Unknown cageid in fdtable access");
    }

    // return EBADFD, if the fd is missing...
    if FDTABLE.get(&cageid).unwrap()[virtualfd as usize].is_none() {
        return Err(threei::Errno::EBADFD as u64);
    }

    // Set optionalinfo or return EBADFD, if that's missing...
    FDTABLE.get_mut(&cageid).unwrap()[virtualfd as usize].as_mut().unwrap().optionalinfo = optionalinfo;
    Ok(())
}

// Helper function used for fork...  Copies an fdtable for another process
// #[doc = include_str!("../docs/copy_fdtable_for_cage.md")]
pub fn copy_fdtable_for_cage(srccageid: u64, newcageid: u64) -> Result<(), threei::Errno> {

    if !FDTABLE.contains_key(&srccageid) {
        panic!("Unknown srccageid in fdtable access");
    }
    if FDTABLE.contains_key(&newcageid) {
        panic!("Known newcageid in fdtable access");
    }

    // Insert a copy and ensure it didn't exist...
    // BUG: Is this a copy!?!  Am I passing a ref to the same thing!?!?!?
    let hmcopy = *FDTABLE.get(&srccageid).unwrap();

    // Increment copied items
    for entry in FDTABLE.get(&srccageid).unwrap().iter() {
        if entry.is_some() {
            let thisrealfd = entry.unwrap().realfd;
            if thisrealfd as u64 != NO_REAL_FD {
                _increment_realfd(thisrealfd);
            }
        }
    }

    assert!(FDTABLE.insert(newcageid, hmcopy).is_none());
    Ok(())
    // I'm not going to bother to check the number of fds used overall yet...
    //    Err(threei::Errno::EMFILE as u64),
}

// This is mostly used in handling exit, etc.  Returns the HashMap
// for the cage.
// #[doc = include_str!("../docs/remove_cage_from_fdtable.md")]
pub fn remove_cage_from_fdtable(cageid: u64) {

    if !FDTABLE.contains_key(&cageid) {
        panic!("Unknown cageid in fdtable access");
    }


    let myfdarray = FDTABLE.get(&cageid).unwrap();
    for item in 0..FD_PER_PROCESS_MAX as usize {
        if myfdarray[item].is_some() {
            let therealfd = myfdarray[item].unwrap().realfd;
            if therealfd as u64 != NO_REAL_FD {
                if therealfd == -1 {
                    println!("When realfd == -1, virtualfd: {:?}", item);
                    io::stdout().flush().unwrap();
                }
                _decrement_realfd(therealfd);
            }
            else{
                // Let their code know this has been closed...
                let closehandlers = CLOSEHANDLERTABLE.lock().unwrap();
                (closehandlers.unreal_handler)(myfdarray[item].unwrap().optionalinfo);
            }
        }
    }
    // I need to do this or else I'll try to double claim the lock and
    // deadlock...
    drop(myfdarray);

    FDTABLE.remove(&cageid);

}

// This removes all fds with the should_cloexec flag set.  They are returned
// in a new hashmap...
// #[doc = include_str!("../docs/empty_fds_for_exec.md")]
pub fn empty_fds_for_exec(cageid: u64) {

    if !FDTABLE.contains_key(&cageid) {
        panic!("Unknown cageid in fdtable access");
    }

    let mut myfdarray = FDTABLE.get_mut(&cageid).unwrap();
    for item in 0..FD_PER_PROCESS_MAX as usize {
        if myfdarray[item].is_some() && myfdarray[item].unwrap().should_cloexec {
            let therealfd = myfdarray[item].unwrap().realfd;
            if therealfd as u64 != NO_REAL_FD {
                _decrement_realfd(therealfd);
            }
            else{
                // Let their code know this has been closed...
                let closehandlers = CLOSEHANDLERTABLE.lock().unwrap();
                (closehandlers.unreal_handler)(myfdarray[item].unwrap().optionalinfo);
            }
            myfdarray[item] = None;
        }
    }

}


// Helper for close.  Returns a tuple of realfd, number of references
// remaining.
// #[doc = include_str!("../docs/close_virtualfd.md")]
pub fn close_virtualfd(cageid:u64, virtfd:i32) -> Result<(),threei::RetVal> {
    if !FDTABLE.contains_key(&cageid) {
        panic!("Unknown cageid in fdtable access");
    }

    let mut myfdarray = FDTABLE.get_mut(&cageid).unwrap();


    if myfdarray[virtfd as usize].is_some() {
        let therealfd = myfdarray[virtfd as usize].unwrap().realfd;

        if therealfd as u64 == NO_REAL_FD {
            // Let their code know this has been closed...
            let closehandlers = CLOSEHANDLERTABLE.lock().unwrap();
            (closehandlers.unreal_handler)(myfdarray[virtfd as usize].unwrap().optionalinfo);
            // Zero out this entry...

            myfdarray[virtfd as usize] = None;
            return Ok(());
        }
        _decrement_realfd(therealfd);
        // Zero out this entry...
        myfdarray[virtfd as usize] = None;
        return Ok(());
    }
    Err(threei::Errno::EBADFD as u64)
}

fn printfd() {
    FDTABLE.iter().for_each(|entry| {
        let (cageid, fd_array) = entry.pair();
        println!("CageID: {}", cageid);
        for (index, fd_entry) in fd_array.iter().enumerate() {
            match fd_entry {
                Some(fd_entry) => {
                    println!("  FD {}: {:?}", index, fd_entry);
                    io::stdout().flush().unwrap();
                }
                None => {
                    println!("  FD {}: None", index);
                    io::stdout().flush().unwrap();
                }
            }
        }
    });
}

// Returns the HashMap returns a copy of the fdtable for a cage.  Useful 
// helper function for a caller that needs to examine the table.  Likely could
// be more efficient by letting the caller borrow this...
// #[doc = include_str!("../docs/return_fdtable_copy.md")]
pub fn return_fdtable_copy(cageid: u64) -> HashMap<i32, FDTableEntry> {

    if !FDTABLE.contains_key(&cageid) {
        panic!("Unknown cageid in fdtable access");
    }

    let mut myhashmap = HashMap::new();

    let myfdarray = FDTABLE.get(&cageid).unwrap();
    for item in 0..FD_PER_PROCESS_MAX as usize {
        if myfdarray[item].is_some() {
            myhashmap.insert(item as i32,myfdarray[item].unwrap());
        }
    }
    myhashmap
}

// Register a series of helpers to be called for close.  Can be called
// multiple times to override the older helpers.
// #[doc = include_str!("../docs/register_close_handlers.md")]
pub fn register_close_handlers(intermediate_handler: fn(i32), final_handler: fn(i32), unreal_handler: fn(i32)) {
    // Unlock the table and set the handlers...
    let mut closehandlers = CLOSEHANDLERTABLE.lock().unwrap();
    closehandlers.intermediate_handler = intermediate_handler;
    closehandlers.final_handler = final_handler;
    closehandlers.unreal_handler = unreal_handler;
}



// #[doc(hidden)]
// Helper to initialize / empty out state so we can test with a clean system...
// This is only used in tests, thus is hidden...
// pub fn refresh() {
//     FDTABLE.clear();
//     FDTABLE.insert(threei::TESTING_CAGEID,[Option::None;FD_PER_PROCESS_MAX as usize]);
//     let mut closehandlers = CLOSEHANDLERTABLE.lock().unwrap_or_else(|e| {
//         CLOSEHANDLERTABLE.clear_poison();
//         e.into_inner()
//     });
//     closehandlers.intermediate_handler = NULL_FUNC;
//     closehandlers.final_handler = NULL_FUNC;
//     closehandlers.unreal_handler = NULL_FUNC;
//     // Note, it doesn't seem that Dashmaps can be poisoned...
// }

// Helpers to track the count of times each realfd is used
// #[doc(hidden)]
fn _decrement_realfd(realfd: i32) -> u64 {
    if realfd as u64 == NO_REAL_FD {
        panic!("Called _decrement_realfd with NO_REAL_FD");
    }

    let newcount:u64 = REALFDCOUNT.get(&realfd).unwrap().value() - 1;

    let closehandlers = CLOSEHANDLERTABLE.lock().unwrap();

    if newcount > 0 {
        (closehandlers.intermediate_handler)(realfd);
        REALFDCOUNT.insert(realfd,newcount);
    }
    else{
        println!("[fd-final close]: realfd - {:?}", realfd);
        io::stdout().flush().unwrap();
        (closehandlers.final_handler)(realfd);
    }
    newcount
}

// Helpers to track the count of times each realfd is used
// #[doc(hidden)]
fn _increment_realfd(realfd: i32) -> u64 {
    if realfd as u64 == NO_REAL_FD {
        return 0
    }

    // Get a mutable reference to the entry so we can update it.
    return match REALFDCOUNT.get_mut(&realfd) {
        Some(mut count) => {
            *count += 1;
            *count
        }
        None => {
            REALFDCOUNT.insert(realfd, 1);
            1
        }
    }
}