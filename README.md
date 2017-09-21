`page_size_rs` is a Rust crate that provides an easy, fast, cross-platform way to retrieve the memory page size of the current system. It supports any POSIX-compliant system and Windows.

Modern hardware and software tend to load data into RAM (and transfer data from RAM to disk) in discrete chunk called pages. This crate provides a helper method to retrieve the size in bytes of these pages. Since the page size *should not* change during execution, `page_size_rs` will cache the result after it has been called once. 

To make this crate useful for writing memory allocators, it does not require (but can use) the Rust standard library.

# Example

```
extern crate page_size;
println!("{}", page_size::get());
```
