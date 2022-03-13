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
//! Since Windows addresses sometimes have to correspond with an allocation
//! granularity that does not always match the size of the page, I have included
//! a method to retrieve that as well.
//!
//! # Example
//!
//! ```rust
//! extern crate page_size;
//! println!("{}", page_size::get());
//! ```

// wasm test runtime needs std
#[cfg(all(test, not(target_os = "emscripten"), target_family = "wasm"))]
extern crate std;

use once_cell::race::OnceNonZeroUsize;

/// This function retrieves the system's memory page size.
///
/// # Example
///
/// ```rust
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
/// ```rust
/// extern crate page_size;
/// println!("{}", page_size::get_granularity());
/// ```
pub fn get_granularity() -> usize {
    get_granularity_helper()
}

// Unix Section

#[cfg(unix)]
#[inline]
fn get_helper() -> usize {
    static PAGE_SIZE: OnceNonZeroUsize = OnceNonZeroUsize::new();
    PAGE_SIZE.get_or_init(unix::get).get()
}

// Unix does not have a specific allocation granularity.
// The page size works well.
#[cfg(unix)]
#[inline]
fn get_granularity_helper() -> usize {
    get_helper()
}

#[cfg(unix)]
mod unix {
    use core::num::NonZeroUsize;

    use libc::{sysconf, _SC_PAGESIZE};

    #[inline]
    pub fn get() -> NonZeroUsize {
        unsafe { NonZeroUsize::new(sysconf(_SC_PAGESIZE) as usize).unwrap() }
    }
}

// WebAssembly section

#[cfg(all(not(target_os = "emscripten"), target_family = "wasm"))]
#[inline]
fn get_helper() -> usize {
    // <https://webassembly.github.io/spec/core/exec/runtime.html#page-size>
    65536
}

// WebAssembly does not have a specific allocation granularity.
// The page size works well.
#[cfg(all(not(target_os = "emscripten"), target_family = "wasm"))]
#[inline]
fn get_granularity_helper() -> usize {
    get_helper()
}

// Windows Section

#[cfg(windows)]
#[inline]
fn get_helper() -> usize {
    static PAGE_SIZE: OnceNonZeroUsize = OnceNonZeroUsize::new();
    PAGE_SIZE.get_or_init(windows::get).get()
}

#[cfg(windows)]
#[inline]
fn get_granularity_helper() -> usize {
    static GRANULARITY: OnceNonZeroUsize = OnceNonZeroUsize::new();
    GRANULARITY.get_or_init(windows::get_granularity).get()
}

#[cfg(windows)]
mod windows {
    use core::mem;
    use core::num::NonZeroUsize;

    use winapi::um::sysinfoapi::GetSystemInfo;
    use winapi::um::sysinfoapi::{LPSYSTEM_INFO, SYSTEM_INFO};

    #[inline]
    pub fn get() -> NonZeroUsize {
        unsafe {
            let mut info: SYSTEM_INFO = mem::zeroed();
            GetSystemInfo(&mut info as LPSYSTEM_INFO);

            NonZeroUsize::new(info.dwPageSize as usize).unwarp()
        }
    }

    #[inline]
    pub fn get_granularity() -> NonZeroUsize {
        unsafe {
            let mut info: SYSTEM_INFO = mem::zeroed();
            GetSystemInfo(&mut info as LPSYSTEM_INFO);

            NonZeroUsize::new(info.dwAllocationGranularity as usize).unwarp()
        }
    }
}

// Stub Section

#[cfg(not(any(unix, windows, target_family = "wasm")))]
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
    }
}

/// Normal tests will do nothing inside WASM
#[cfg(all(test, not(target_os = "emscripten"), target_family = "wasm"))]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_get() {
        #[allow(unused_variables)]
        let page_size = get();
    }

    #[wasm_bindgen_test]
    fn test_get_granularity() {
        #[allow(unused_variables)]
        let granularity = get_granularity();
    }
}
