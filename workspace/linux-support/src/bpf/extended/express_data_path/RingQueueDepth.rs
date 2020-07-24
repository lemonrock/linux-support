// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Ring queue depth.
///
/// Must be a power of two.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum RingQueueDepth
{
	/// This is the same as `libbpf`'s `XSK_RING_PROD__DEFAULT_NUM_DESCS` and `XSK_RING_CONS__DEFAULT_NUM_DESCS`.
	_2048,
}

impl Default for RingQueueDepth
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::_2048
	}
}

impl Into<NonZeroU32> for RingQueueDepth
{
	#[inline(always)]
	fn into(self) -> NonZeroU32
	{
		unsafe { transmute(self )}
	}
}

impl RingQueueDepth
{
	#[inline(always)]
	const fn memory_length<D: Descriptor>(self) -> u64
	{
		(self as u32 as u64) * (size_of::<D>() as u64)
	}
	
	#[inline(always)]
	fn mask(&self) -> u32
	{
		(self as u32) - 1
	}
}
