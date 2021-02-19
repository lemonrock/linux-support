// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#[cfg(not(any(target_os = "ios", target_os = "macos")))] use ::libc::c_char;
use ::libc::FILE;
#[cfg(not(any(target_os = "ios", target_os = "macos")))] use ::libc::size_t;


#[link(name = "c")]
extern "C"
{
	pub static mut stdin: *const FILE;

	pub static mut stdout: *const FILE;

	pub static mut stderr: *const FILE;

	#[cfg(not(any(target_os = "ios", target_os = "macos")))]
	pub fn open_memstream(buffer: *mut *mut c_char, size: *mut size_t) -> *mut FILE;
}
