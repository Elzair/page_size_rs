#![no_std]

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

pub fn get_size() -> usize {
    get_size_helper()
}

// Unix Section

#[cfg(all(unix, feature = "no-std"))]
#[inline]
fn get_size_helper() -> usize {
    static INIT: Once<usize> = Once::new();
    
    *INIT.call_once(unix::get_size_unix)
}

#[cfg(all(unix, not(feature = "no-std")))]
#[inline]
fn get_size_helper() -> usize {
    static INIT: Once = ONCE_INIT;
    static mut PAGE_SIZE: usize = 0;

    unsafe {
        INIT.call_once(|| PAGE_SIZE = unix::get_size_unix());
        PAGE_SIZE
    }
}

#[cfg(unix)]
extern crate libc;

#[cfg(unix)]
mod unix {
    use super::*;

    #[inline]
    pub fn get_size_unix() -> usize {
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
fn get_size_helper() -> usize {
    static INIT: Once<usize> = Once::new();
    
    *INIT.call_once(windows::get_size_windows)
}

#[cfg(all(windows, not(feature = "no-std")))]
#[inline]
fn get_size_helper() -> usize {
    static INIT: Once = ONCE_INIT;
    static mut PAGE_SIZE: usize = 0;

    unsafe {
        INIT.call_once(|| PAGE_SIZE = windows::get_size_windows());
        PAGE_SIZE
    }
}

#[cfg(windows)]
mod windows {
    use super::*;

    #[inline]
    pub fn get_size_windows() -> usize {
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
    fn test_get_size() {
        #[allow(unused_variables)]
        let page_size = get_size();
    }
}
