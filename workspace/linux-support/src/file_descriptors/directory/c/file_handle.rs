// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
pub(super) struct file_handle
{
	/// Set by caller to indicate maximum size of `f_handle`.
	/// Set by callee to indicate actual size of `f_handle`.
	/// Can not exceed `MAX_HANDLE_SZ`.
	pub(super) handle_bytes: c_uint,

	/// Set by callee.
	pub(super) handle_type: c_int,

	/// File identifier.
	///
	/// Set by callee.
	///
	/// Technically a variable sized array.
	f_handle: [c_char; MAX_HANDLE_SZ],
}

impl file_handle
{
	#[allow(deprecated)]
	#[inline(always)]
	pub(super) const fn new() -> Self
	{
		Self
		{
			handle_bytes: MAX_HANDLE_SZ as u32,
			handle_type: unsafe { uninitialized() },
			f_handle: unsafe { uninitialized() },
		}
	}
}
