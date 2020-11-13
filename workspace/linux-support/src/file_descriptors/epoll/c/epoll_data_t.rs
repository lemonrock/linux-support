// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// epoll data associated with an event and registered with a file descriptor (FD).
#[repr(C)]
#[derive(Copy, Clone, Eq)]
pub(crate) union epoll_data_t
{
	/// Data as a pointer.
	pub(crate) ptr: *mut c_void,

	/// Data as a file descriptor.
	pub(crate) fd: RawFd,

	/// Data as an arbitrary 32-bit unsigned integer.
	pub(crate) u32: u32,

	/// Data as an arbitrary 64-bit unsigned integer.
	pub(crate) u64: u64,
}

impl Default for epoll_data_t
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe_zeroed()
	}
}

impl Debug for epoll_data_t
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", unsafe { self.u64 })
	}
}

impl PartialEq for epoll_data_t
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.u64 == other.u64 }
	}
}

impl Hash for epoll_data_t
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		unsafe { self.u64.hash(hasher) }
	}
}

impl PartialOrd for epoll_data_t
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.u64.partial_cmp(&other.u64) }
	}
}

impl Ord for epoll_data_t
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.u64.cmp(&other.u64) }
	}
}
