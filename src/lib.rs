#![no_std]
//! This crate provides an easy, fast, cross-platform way to retrieve the
//! memory page size of the current system.
//!
//! Modern hardware and software tend to load data into RAM (and transfer data
//! from RAM to disk) in discrete chunk called pages. This crate provides a
//! helper method to retrieve the size in bytes of these pages. Since the page
//! size *should not* change during execution, this crate will cache the result
//! after it has been called once.
//!
//! To make this crate useful for writing memory allocators, it does not require
//! (but can use) the Rust standard library. 
//!
//! # Example
//!
//! ```
//! extern crate page_size;
//! println!("{}", page_size::get());
//! ```

// `const_fn` is needed for `spin::Once`.
#![cfg_attr(feature = "no_std", feature(const_fn))]

#[cfg(feature = "no_std")]
extern crate spin;
#[cfg(feature = "no_std")]
use spin::Once;

#[cfg(not(feature = "no_std"))]
extern crate std;
#[cfg(not(feature = "no_std"))]
use std::sync::{Once, ONCE_INIT};

/// This function retrieves the system's memory page size.
///
/// # Example
///
/// ```
/// extern crate page_size;
/// println!("{}", page_size::get());
/// ```
pub fn get() -> usize {
    get_helper()
}

/// This function retrieves the system's memory allocation granularity.
///
/// # Example
///
/// ```
/// extern crate page_size;
/// println!("{}", page_size::get_granularity());
/// ```
pub fn get_granularity() -> usize {
    get_granularity_helper()
}

// Unix Section

#[cfg(all(unix, feature = "no_std"))]
#[inline]
fn get_helper() -> usize {
    static INIT: Once<usize> = Once::new();
    
    *INIT.call_once(unix::get_unix)
}

#[cfg(all(unix, not(feature = "no_std")))]
#[inline]
fn get_helper() -> usize {
    static INIT: Once = ONCE_INIT;
    static mut PAGE_SIZE: usize = 0;

    unsafe {
        INIT.call_once(|| PAGE_SIZE = unix::get_unix());
        PAGE_SIZE
    }
}

// Unix does not have a specific allocation granularity.
// The page size works well.
#[cfg(unix)]
#[inline]
fn get_granularity_helper() -> usize {
    get_helper()
}

#[cfg(unix)]
extern crate libc;

#[cfg(unix)]
mod unix {
    use super::*;

    #[inline]
    pub fn get_unix() -> usize {
        unsafe {
            libc::sysconf(libc::_SC_PAGESIZE) as usize
        }
    }
}

// Windows Section

#[cfg(windows)]
extern crate winapi;

#[cfg(all(windows, feature = "no_std"))]
#[inline]
fn get_helper() -> usize {
    static INIT: Once<usize> = Once::new();
    
    *INIT.call_once(windows::get_windows)
}

#[cfg(all(windows, not(feature = "no_std")))]
#[inline]
fn get_helper() -> usize {
    static INIT: Once = ONCE_INIT;
    static mut PAGE_SIZE: usize = 0;

    unsafe {
        INIT.call_once(|| PAGE_SIZE = windows::get_windows());
        PAGE_SIZE
    }
}

#[cfg(all(windows, feature = "no_std"))]
#[inline]
fn get_granularity_helper() -> usize {
    static GRINIT: Once<usize> = Once::new();
    
    *GRINIT.call_once(windows::get_granularity_windows)
}

#[cfg(all(windows, not(feature = "no_std")))]
#[inline]
fn get_granularity_helper() -> usize {
    static GRINIT: Once = ONCE_INIT;
    static mut GRANULARITY: usize = 0;

    unsafe {
        GRINIT.call_once(|| GRANULARITY = windows::get_granulariy_windows());
        GRANULARITY
    }
}

#[cfg(windows)]
mod windows {
    use super::*;

    use winapi::sysinfoapi::{SYSTEM_INFO, LPSYSTEM_INFO};
    use winapi::kernel32::GetSystemInfo;

    #[inline]
    pub fn get_windows() -> usize {
        unsafe {
            let mut info: SYSTEM_INFO = mem::zeroed();
            GetSystemInfo(&mut info as LPSYSTEM_INFO);

            info.dwPageSize as usize
        }
    }

    #[inline]
    pub fn get_granularity_windows() -> usize {
        unsafe {
            let mut info: SYSTEM_INFO = mem::zeroed();
            GetSystemInfo(&mut info as LPSYSTEM_INFO);

            info.dwAllocationGranularity as usize
        }
    }
}

// Stub Section

#[cfg(not(any(unix, windows)))]
#[inline]
fn get_helper() -> usize {
    4096 // 4k is the default on many systems
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get() {
        #[allow(unused_variables)]
        let page_size = get();
    }    

    #[test]
    fn test_get_granularity() {
        #[allow(unused_variables)]
        let granularity = get_granularity();
        assert_eq!(granularity, 4096);
    }
}
