// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Copy, Clone)]
#[repr(C)]
pub(super) union io_uring_sqe_anonymous_4
{
	pub(super) anonymous_1: io_uring_sqe_anonymous_4_anonymous_1,
	__pad2: [u64; 3]
}

impl Default for io_uring_sqe_anonymous_4
{
	#[inline(always)]
	fn default() -> Self
	{
		Self { anonymous_1: io_uring_sqe_anonymous_4_anonymous_1::default() }
	}
}

impl Debug for io_uring_sqe_anonymous_4
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "io_uring_sqe_anonymous_4_anonymous_1({:?})", unsafe { self.anonymous_1 })
	}
}

impl PartialEq for io_uring_sqe_anonymous_4
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.anonymous_1 == other.anonymous_1 }
	}
}

impl Eq for io_uring_sqe_anonymous_4
{
}

impl PartialOrd for io_uring_sqe_anonymous_4
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.anonymous_1.partial_cmp(&other.anonymous_1) }
	}
}

impl Ord for io_uring_sqe_anonymous_4
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.anonymous_1.cmp(&other.anonymous_1) }
	}
}

impl Hash for io_uring_sqe_anonymous_4
{
	#[inline(always)]
	fn hash<H>(&self, state: &mut H) where H: Hasher
	{
		unsafe { self.anonymous_1.hash(state) }
	}
}
