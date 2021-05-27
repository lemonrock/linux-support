// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(super) union io_uring_sqe_anonymous_4_anonymous_1_anonymous_1
{
	/// Index into an array of fixed buffers.
	///
	/// Only valid if fixed buffers were registered.
	pub(super) buf_index: RegisteredBufferIndex,

	/// for grouped buffer selection.
	pub(super) buf_group: BufferGroup,
}

impl Default for io_uring_sqe_anonymous_4_anonymous_1_anonymous_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe_zeroed()
	}
}

impl Debug for io_uring_sqe_anonymous_4_anonymous_1_anonymous_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "io_uring_sqe_anonymous_4_anonymous_1_anonymous_1({:?})", self.buf_index())
	}
}

impl PartialEq for io_uring_sqe_anonymous_4_anonymous_1_anonymous_1
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.buf_index() == other.buf_index()
	}
}

impl Eq for io_uring_sqe_anonymous_4_anonymous_1_anonymous_1
{
}

impl PartialOrd for io_uring_sqe_anonymous_4_anonymous_1_anonymous_1
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		self.buf_index().partial_cmp(&other.buf_index())
	}
}

impl Ord for io_uring_sqe_anonymous_4_anonymous_1_anonymous_1
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.buf_index().cmp(&other.buf_index())
	}
}

impl Hash for io_uring_sqe_anonymous_4_anonymous_1_anonymous_1
{
	#[inline(always)]
	fn hash<H>(&self, state: &mut H) where H: Hasher
	{
		self.buf_index().hash(state)
	}
}

impl io_uring_sqe_anonymous_4_anonymous_1_anonymous_1
{
	#[inline(always)]
	fn buf_index(&self) -> RegisteredBufferIndex
	{
		let unaligned_pointer = unsafe { addr_of!(self.buf_index) };
		unsafe { unaligned_pointer.read_unaligned() }
	}
}
