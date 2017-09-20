#![no_std]

#[cfg(feature = "use-std")]
extern crate std;

#[cfg(feature = "use-std")]
use std::mem;
#[cfg(not(feature = "use-std"))]
use core::mem;

pub fn get_size() -> usize {
    get_size_helper()
}

// Unix Section

#[cfg(unix)]
extern crate libc;

#[cfg(unix)]
fn get_size_helper() -> usize {
    unsafe {
        libc::sysconf(libc::_SC_PAGESIZE) as usize
    }
}

// Windows Section

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
fn get_size_helper() -> usize {
    unsafe {
        let mut info: winapi::SYSTEM_INFO = mem::zeroed();
        winapi::kernel32::GetSystemInfo(&mut info);

        info.dwPageSize as usize
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
