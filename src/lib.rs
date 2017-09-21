#![no_std]
//! This crate provides an easy, fast, cross-platform way to retrieve the
//! memory page size of the current system. Modern hardware and software tend
//! to load data into RAM (and transfer data from RAM to disk) in discrete
//! chunk called pages. This crate provides a helper method to retrieve the size
//! in bytes of these pages. Since the page size *should not* change during
//! execution, this crate will cache the result after it has been called once.
//! To make this crate useful for writing memory allocators, it does not require
//! (but can use) the Rust standard library. 
//!
//! # Example
//!
#![cfg_attr(not(feature = "no-std"), doc = " ``` ")]
#![cfg_attr(feature = "no-std", doc = " ```no_run ")]
//! extern crate page_size;
//! println!("{}", page_size::get());
//! ```

// `const_fn` is needed for `spin::Once`.
#![cfg_attr(feature = "no-std", feature(const_fn))]

#[cfg(feature = "no-std")]
extern crate spin;
#[cfg(feature = "no-std")]
use spin::Once;

#[cfg(not(feature = "no-std"))]
extern crate std;
#[cfg(not(feature = "no-std"))]
use std::sync::{Once, ONCE_INIT};

/// This function retrieves the system's memory page size.
///
/// # Example
///
#[cfg_attr(not(feature = "no-std"), doc = " ``` ")]
#[cfg_attr(feature = "no-std", doc = " ```no_run ")]
/// extern crate page_size;
/// println!("{}", page_size::get());
/// ```
pub fn get() -> usize {
    get_helper()
}

// Unix Section

#[cfg(all(unix, feature = "no-std"))]
#[inline]
fn get_helper() -> usize {
    static INIT: Once<usize> = Once::new();
    
    *INIT.call_once(unix::get_unix)
}

#[cfg(all(unix, not(feature = "no-std")))]
#[inline]
fn get_helper() -> usize {
    static INIT: Once = ONCE_INIT;
    static mut PAGE_SIZE: usize = 0;

    unsafe {
        INIT.call_once(|| PAGE_SIZE = unix::get_unix());
        PAGE_SIZE
    }
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

#[cfg(all(windows, feature = "no-std"))]
#[inline]
fn get_helper() -> usize {
    static INIT: Once<usize> = Once::new();
    
    *INIT.call_once(windows::get_windows)
}

#[cfg(all(windows, not(feature = "no-std")))]
#[inline]
fn get_helper() -> usize {
    static INIT: Once = ONCE_INIT;
    static mut PAGE_SIZE: usize = 0;

    unsafe {
        INIT.call_once(|| PAGE_SIZE = windows::get_windows());
        PAGE_SIZE
    }
}

#[cfg(windows)]
mod windows {
    use super::*;

    #[inline]
    pub fn get_windows() -> usize {
        unsafe {
            let mut info: winapi::SYSTEM_INFO = mem::zeroed();
            winapi::kernel32::GetSystemInfo(&mut info);

            info.dwPageSize as usize
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get() {
        #[allow(unused_variables)]
        let page_size = get();
    }
}
