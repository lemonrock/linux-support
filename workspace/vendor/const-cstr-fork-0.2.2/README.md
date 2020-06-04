# `const-cstr-fork` [![](https://img.shields.io/crates/v/const-cstr-fork.svg)](https://crates.io/crates/const-cstr-fork)
___This crate has been forked from [const-str](https://github.com/abonander/const-cstr) to try to optimise performance in some scenarios. It may disappear or be merged back into const-cstr. Many thanks are due to constr-str's author for the original design.___

Create static C-compatible strings from Rust string literals.

Usage
------
Cargo.toml:
```toml
[dependencies]
const-cstr-fork = "0.1"
```

Crate root:
```rust
#[macro_use] extern crate const_cstr_fork;
```

Example
-------
```rust
 #[macro_use] extern crate const_cstr_fork;
 // Just for the `libc::c_char` type alias.
 extern crate libc;
     
 use std::ffi::CStr;

 const_cstr! {
     HELLO_CSTR = "Hello, world!";

     // Multiple declarations can be made with one invocation.
     // GOODNIGHT_CSTR = "Goodnight, sun!";

     // But only with the same visibility:
     // pub GOODNIGHT_CSTR = "Goodnight, sun!";
     // ^~~ Error: expected identifier, found `pub` 
 }

 // Imagine this is an `extern "C"` function linked from some other lib.
 unsafe fn print_c_string(cstr: *const libc::c_char) {
     println!("{}", CStr::from_ptr(cstr).to_str().unwrap());
 }

 fn main() {
     // When just passed a literal, returns an rvalue instead.
     let goodnight_cstr = const_cstr!("Goodnight, sun!");

     unsafe {
         print_c_string(HELLO_CSTR.as_ptr());
         print_c_string(goodnight_cstr.as_ptr());
     }
 }
 ```

 Prints:

 ```notest
 Hello, world!
 Goodnight, sun!
 ```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
