// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[cfg(target_pointer_width = "32")]
#[derive(Default, Debug, Clone)]
#[repr(C)]
pub(crate) struct msghdr
{
	pub(crate) msg_name: *mut c_void,
	pub(crate) msg_namelen: socklen_t,
	pub(crate) msg_iov: *mut iovec,
	pub(crate) msg_iovlen: i32,
	pub(crate) msg_control: *mut c_void,
	pub(crate) msg_controllen: socklen_t,
	pub(crate) msg_flags: c_int,
}

#[cfg(target_pointer_width = "64")]
#[derive(Debug, Clone)]
#[repr(C)]
pub(crate) struct msghdr
{
	pub(crate) msg_name: *mut c_void,
	pub(crate) msg_namelen: socklen_t,
	pub(crate) msg_iov: *mut iovec,
	#[cfg(target_endian = "little")] pub(crate) msg_iovlen: i32,
	#[cfg(target_endian = "little")] __pad1: u32,
	#[cfg(target_endian = "big")] __pad1: u32,
	#[cfg(target_endian = "big")] pub(crate) msg_iovlen: socklen_t,
	pub(crate) msg_control: *mut c_void,
	#[cfg(target_endian = "little")] pub(crate) msg_controllen: socklen_t,
	#[cfg(target_endian = "little")] __pad2: u32,
	#[cfg(target_endian = "big")] __pad2: u32,
	#[cfg(target_endian = "big")] pub(crate) msg_controllen: socklen_t,
	pub(crate) msg_flags: c_int,
}

impl Default for msghdr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl msghdr
{
	#[cfg(target_pointer_width = "32")]
	pub(crate) fn new(msg_name: *mut c_void, msg_namelen: socklen_t, msg_iov: *mut iovec, msg_iovlen: i32, msg_control: *mut c_void, msg_controllen: socklen_t, msg_flags: c_int) -> Self
	{
		Self
		{
			msg_name,
			msg_namelen,
			msg_iov,
			msg_iovlen,
			msg_control,
			msg_controllen,
			msg_flags,
		}
	}

	#[cfg(target_pointer_width = "64")]
	pub(crate) fn new(msg_name: *mut c_void, msg_namelen: socklen_t, msg_iov: *mut iovec, msg_iovlen: i32, msg_control: *mut c_void, msg_controllen: socklen_t, msg_flags: c_int) -> Self
	{
		#[allow(deprecated)]
		Self
		{
			msg_name,
			msg_namelen,
			msg_iov,
			msg_iovlen,
			__pad1: unsafe { uninitialized() },
			msg_control,
			msg_controllen,
			__pad2: unsafe { uninitialized() },
			msg_flags,
		}
	}

	#[inline(always)]
	pub(crate) fn initialize_sole_header<T: Sized>(&mut self, cmsg_level: c_int, cmsg_type: c_int, array: &[T])
	{
		let control_length =
		{
			let first_header_mut = self.first_header_mut();
			let first_header = first_header_mut.unwrap();
			first_header.initialize(cmsg_level, cmsg_type, array);

			first_header.cmsg_len
		};

		// Sum of the length of all control messages in the buffer.
		self.msg_controllen = control_length;
	}

	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) fn message_headers_iterator<'a>(&'a self) -> MessageHeadersIterator<'a>
	{
		MessageHeadersIterator
		{
			parent: self,
			next: self.first_header(),
		}
	}

	/// Equivalent to the lib c macro `CMSG_FIRSTHDR()`.
	#[inline(always)]
	pub(crate) fn first_header(&self) -> Option<&cmsghdr>
	{
		if likely!(self.msg_controllen >= cmsghdr::Size)
		{
			debug_assert!(!self.msg_control.is_null(), "msg_control is null but msg_controllen is positive");

			Some(unsafe { & * (self.msg_control as *const cmsghdr) })
		}
		else
		{
			None
		}
	}

	/// Equivalent to the lib c macro `CMSG_FIRSTHDR()`.
	#[inline(always)]
	pub(crate) fn first_header_mut(&mut self) -> Option<&mut cmsghdr>
	{
		let there_is_one_or_more_headers = self.msg_controllen >= cmsghdr::Size;

		if likely!(there_is_one_or_more_headers)
		{
			debug_assert!(!self.msg_control.is_null(), "msg_control is null but msg_controllen is positive");

			Some(unsafe { &mut * (self.msg_control as *mut cmsghdr) })
		}
		else
		{
			None
		}
	}

	#[inline(always)]
	pub(crate) fn end(&self) -> usize
	{
		(self.msg_control as usize) + (self.msg_controllen as usize)
	}

//	#[inline(always)]
//	fn __MHDR_END(&mut self) -> *mut c_uchar
//	{
//		((self.msg_control as usize) + (self.msg_controllen as usize)) as *mut c_char
//	}
//
//	#[inline(always)]
//	fn CMSG_FIRSTHDR(&mut self) -> *mut cmsghdr
//	{
//		if (self.msg_controllen as usize) >= size_of::<cmsghdr>()
//		{
//			self.msg_control as *mut cmsghdr
//		}
//		else
//		{
//			null_mut()
//		}
//	}
}
