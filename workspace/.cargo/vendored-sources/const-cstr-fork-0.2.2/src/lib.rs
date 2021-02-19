// Copyright (c) 2015 const-cstr developers
// Copyright (c) 2017 const-cstr-fork developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.
//! Create static C-compatible strings from Rust string literals.
//! 
//! Example
//! -------
//!
//! ```rust
//! #[macro_use] extern crate const_cstr_fork;
//! // Just for the `libc::c_char` type alias.
//! extern crate libc;
//!     
//! use std::ffi::CStr;
//!
//! const_cstr! {
//!     HELLO_CSTR = "Hello, world!";
//!
//!     // Multiple declarations can be made with one invocation.
//!     // GOODNIGHT_CSTR = "Goodnight, sun!";
//!
//!     // But only with the same visibility:
//!     // pub GOODNIGHT_CSTR = "Goodnight, sun!";
//!     // ^~~ Error: expected identifier, found `pub` 
//! }
//!
//! // Imagine this is an `extern "C"` function linked from some other lib.
//! unsafe fn print_c_string(cstr: *const libc::c_char) {
//!     println!("{}", CStr::from_ptr(cstr).to_str().unwrap());
//! }
//!
//! fn main() {
//!     // When just passed a literal, returns an rvalue instead.
//!     let goodnight_cstr = const_cstr!("Goodnight, sun!");
//!
//!     unsafe {
//!         print_c_string(HELLO_CSTR.as_ptr());
//!         print_c_string(goodnight_cstr.as_ptr());
//!     }
//! }
//! ```
//!
//! Prints:
//!
//! ```notest
//! Hello, world!
//! Goodnight, sun!
//! ```

#![allow(non_snake_case)]

extern crate libc;

use std::ffi::CStr;

pub trait ToPointerCString
{
	#[inline(always)]
	fn to_ptr(&self) -> *const ::libc::c_char;
}

/// A type representing a static C-compatible string.
///
/// Note
/// ----
/// Use ONLY the `const_cstr!` macro to create an instance of this struct.
/// The macro will include the NUL byte for you.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCStr {
    /// The wrapped string value. Not intended to be used for manual initialization.
    /// Public only to allow initialization by the `const_cstr!` macro.
    ///
    /// Includes the NUL terminating byte. Use `to_str()` to get an `&'static str`
    /// without the NUL terminating byte.
    pub cValue: &'static str,
	
	/// Excludes the terminating byte
	pub rustValue: &'static str,
}

impl ToPointerCString for ConstCStr
{
	#[inline(always)]
	fn to_ptr(&self) -> *const ::libc::c_char
	{
		self.as_ptr()
	}
}

impl ToPointerCString for CStr
{	
	#[inline(always)]
	fn to_ptr(&self) -> *const ::libc::c_char
	{
		self.as_ptr() as *const ::libc::c_char
	}
}

impl ConstCStr {
    /// Returns the wrapped string, without the NUL terminating byte.
    ///
    /// Compare to `CStr::to_str()` which checks that the string is valid UTF-8 first,
    /// since it starts from an arbitrary pointer instead of a Rust string slice.
	#[inline(always)]
    pub fn to_str(&self) -> &'static str {
        self.rustValue
    }

    /// Returns the wrapped string as a byte slice, **without** the NUL terminating byte.
	#[inline(always)]
    pub fn to_bytes(&self) -> &'static [u8] {
        self.rustValue.as_bytes()
    }

    /// Returns the wrapped string as a byte slice, *with** the NUL terminating byte.
	#[inline(always)]
    pub fn to_bytes_with_nul(&self) -> &'static [u8] {
        self.cValue.as_bytes()
    }

    /// Returns a pointer to the beginning of the wrapped string.
    ///
    /// Suitable for passing to any function that expects a C-compatible string. 
    /// Since the underlying string is guaranteed to be `'static`, 
    /// the pointer should always be valid.
	#[inline(always)]
    pub fn as_ptr(&self) -> *const libc::c_char {
		self.cValue.as_ptr() as *const libc::c_char
    }

    /// Returns the wrapped string as an `&'static CStr`, skipping the length check that
    /// `CStr::from_ptr()` performs (since we know the length already).
	#[inline(always)]
    pub fn as_cstr(&self) -> &'static CStr {
        unsafe {
            CStr::from_ptr(self.cValue.as_ptr() as *const i8)
        }
    }
}

/// Create a C-compatible string as an rvalue or a `const` binding.
/// Appends a NUL byte to the passed string.
///
/// Multiple `const` declarations can be created with one invocation, but only with the same
/// visibility (`pub` or not).
///
/// See crate root documentation for example usage.
///
/// Note
/// ----
/// For logical consistency, the passed string(s) should not contain any NUL bytes.
/// Remember that functions consuming a C-string will only see up to the first NUL byte.
#[macro_export]
macro_rules! const_cstr {
    ($strval:expr) => (
        $crate::ConstCStr
		{
			rustValue: $strval,
			cValue: concat!($strval, "\0")
		}
    );
    ($($strname:ident = $strval:expr);+;) => (
        $(
            const $strname: $crate::ConstCStr = const_cstr!($strval);
        )+
    );
    ($(pub $strname:ident = $strval:expr);+;) => (
        $(
            pub const $strname: $crate::ConstCStr = const_cstr!($strval);
        )+
    );
}

#[test]
fn test_creates_valid_str() {
    const_cstr! {
        HELLO_CSTR = "Hello, world!";
    }

    let cstr = unsafe { CStr::from_ptr(HELLO_CSTR.as_ptr()) };

    assert_eq!(HELLO_CSTR.to_str(), cstr.to_str().unwrap());
}

