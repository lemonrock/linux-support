// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Copy, Clone)]
#[repr(C)]
pub(super) union io_uring_sqe_anonymous_2
{
	/// pointer to buffer or iovecs.
	pub(super) addr: u64,

	/// offset for splice in.
	pub(super) splice_off_in: u64,
}

impl Default for io_uring_sqe_anonymous_2
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe_zeroed()
	}
}

impl Debug for io_uring_sqe_anonymous_2
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "io_uring_sqe_anonymous_2({:?})", unsafe { self.addr })
	}
}

impl PartialEq for io_uring_sqe_anonymous_2
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.addr == other.addr }
	}
}

impl Eq for io_uring_sqe_anonymous_2
{
}

impl PartialOrd for io_uring_sqe_anonymous_2
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.addr.partial_cmp(&other.addr) }
	}
}

impl Ord for io_uring_sqe_anonymous_2
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.addr.cmp(&other.addr) }
	}
}

impl Hash for io_uring_sqe_anonymous_2
{
	#[inline(always)]
	fn hash<H>(&self, state: &mut H) where H: Hasher
	{
		unsafe { self.addr.hash(state) }
	}
}
