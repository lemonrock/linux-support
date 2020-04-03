// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[cfg(target_pointer_width = "32")]
#[repr(C)]
pub(crate) struct cmsghdr
{
	pub(crate) cmsg_len: socklen_t,
	pub(crate) cmsg_level: c_int,
	pub(crate) cmsg_type: c_int,
	data: PhantomData<u8>,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub(crate) struct cmsghdr
{
	#[cfg(target_endian = "little")] pub(crate) cmsg_len: socklen_t,
	#[cfg(target_endian = "little")] __pad1: u32,
	#[cfg(target_endian = "big")] __pad1: u32,
	#[cfg(target_endian = "big")] pub(crate) cmsg_len: socklen_t,
	pub(crate) cmsg_level: c_int,
	pub(crate) cmsg_type: c_int,
	data: PhantomData<u8>,
}

impl cmsghdr
{
	#[inline(always)]
	pub(crate) fn initialize_known_fields(&mut self, cmsg_level: c_int, cmsg_type: c_int, data_size: usize)
	{
		let cmsg_len = Self::CMSG_LEN(data_size);

		unsafe
		{
			std::ptr::write(&mut self.cmsg_level, cmsg_level);
			std::ptr::write(&mut self.cmsg_type, cmsg_type);
			std::ptr::write(&mut self.cmsg_len, cmsg_len as u32);
		}
	}

	#[inline(always)]
	pub(crate) fn initialize<T: Sized>(&mut self, cmsg_level: c_int, cmsg_type: c_int, array: &[T])
	{
		self.initialize_known_fields(cmsg_level, cmsg_type, size_of::<T>() * array.len());

		self.initialize_payload(array);
	}

	#[inline(always)]
	fn initialize_payload<T: Sized>(&mut self, array: &[T])
	{
		let destination = self.CMSG_DATA() as *mut T;
		unsafe { array.as_ptr().copy_to_nonoverlapping(destination, array.len()) }
	}

	/// Equivalent to the lib c macro `CMSG_DATA()`.
	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) fn data(&self) -> &[u8]
	{
		unsafe { from_raw_parts(&self.data as *const PhantomData<u8> as *const u8, self.length()) }
	}

	/// Equivalent to the lib c macro `CMSG_DATA()`.
	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) fn data_mut(&mut self) -> &mut [u8]
	{
		unsafe { from_raw_parts_mut(&mut self.data as *mut PhantomData<u8> as *mut u8, self.length()) }
	}

	const Size: c_uint = size_of::<Self>() as c_uint;

	#[inline(always)]
	fn is_last(&self, parent: &msghdr) -> bool
	{
		self.cmsg_len < Self::Size || self.__CMSG_LEN() + size_of::<Self>() >= parent.end() - (self as *const Self as usize)
	}

	/// Equivalent to the lib c macro `CMSG_NXTHDR()`.
	#[inline(always)]
	pub(crate) fn next(&self, parent: &msghdr) -> Option<&Self>
	{
		if likely!(self.is_last(parent))
		{
			None
		}
		else
		{
			Some(unsafe { & * (self.__CMSG_NEXT()) })
		}
	}

	/// Equivalent to the lib c macro `CMSG_NXTHDR()`.
	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) fn next_mut(&mut self, parent: &msghdr) -> Option<&mut Self>
	{
		if likely!(self.is_last(parent))
		{
			None
		}
		else
		{
			Some(unsafe { & mut * (self.__CMSG_NEXT_mut()) })
		}
	}

	#[inline(always)]
	fn length(&self) -> usize
	{
		self.cmsg_len as usize
	}

	#[inline(always)]
	fn __CMSG_LEN(&self) -> usize
	{
		Self::CMSG_ALIGN(self.length())
	}

	#[inline(always)]
	fn __CMSG_NEXT(&self) -> *const Self
	{
		(self as *const Self as usize + self.__CMSG_LEN()) as *const Self
	}

	#[inline(always)]
	fn __CMSG_NEXT_mut(&mut self) -> *mut Self
	{
		(self as *mut Self as usize + self.__CMSG_LEN()) as *mut Self
	}

	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) fn CMSG_NXTHDR(&mut self, mhdr: &msghdr) -> *mut Self
	{
		if self.is_last(mhdr)
		{
			null_mut()
		}
		else
		{
			self.__CMSG_NEXT() as *mut _
		}
	}

	#[inline(always)]
	pub(crate) fn CMSG_DATA_mut(&mut self) -> *mut c_uchar
	{
		(unsafe { (self as *mut Self).add(1) }) as *mut c_uchar
	}

	#[inline(always)]
	pub(crate) fn CMSG_DATA(&self) -> *const c_uchar
	{
		(unsafe { (self as *const Self).add(1) }) as *const c_uchar
	}

	#[inline(always)]
	pub(crate) const fn CMSG_ALIGN(length: usize) -> usize
	{
		// This rounds up `length` to the nearest size of `usize`.
		(length + size_of::<usize>() - 1) & !(size_of::<usize>() - 1)
	}

	#[inline(always)]
	pub(crate) const fn CMSG_SPACE(length: usize) -> usize
	{
		Self::CMSG_ALIGN(length) + Self::CMSG_ALIGN(size_of::<Self>())
	}

	#[inline(always)]
	pub(crate) const fn CMSG_LEN(length: usize) -> usize
	{
		length + Self::CMSG_ALIGN(size_of::<Self>())
	}
}
