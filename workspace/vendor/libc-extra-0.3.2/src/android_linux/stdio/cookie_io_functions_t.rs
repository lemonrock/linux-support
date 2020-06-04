// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


//noinspection SpellCheckingInspection
/// Used by `fopencookie`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct cookie_io_functions_t
{
	/// Read function pointer.
	pub read: cookie_read_function_t,
	
	/// Write function pointer.
	pub write: cookie_write_function_t,
	
	/// Seek function pointer.
	pub seek: cookie_seek_function_t,
	
	/// Close function pointer.
	pub close: cookie_close_function_t,
}

impl Default for cookie_io_functions_t
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			read: Self::default_read,
			write: Self::default_write,
			seek: Self::default_seek,
			close: Self::default_close,
		}
	}
}

impl cookie_io_functions_t
{
	/// No-op cookie read function.
	pub unsafe extern "C" fn default_read(_cookier: *mut c_void, _buf: *mut c_char, _size: size_t) -> ssize_t
	{
		0
	}
	
	/// No-op cookie write function.
	pub unsafe extern "C" fn default_write(_cookier: *mut c_void, _buf: *const c_char, _size: size_t) -> ssize_t
	{
		0
	}
	
	/// No-op cookie seek function.
	pub unsafe extern "C" fn default_seek(_cookier: *mut c_void, _offset: *mut off_t, _whence: c_int) -> c_int
	{
		0
	}
	
	/// No-op cookie close function.
	pub unsafe extern "C" fn default_close(_cookier: *mut c_void) -> c_int
	{
		0
	}
}
