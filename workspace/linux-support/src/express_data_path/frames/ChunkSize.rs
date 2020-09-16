// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Frame size in bytes.
///
/// Represents the size of elements in user memory used for storing (Ethernet) frames (packets).
///
/// It is the maximum size of `len(ethernet_packet) + frame_headroom`.
///
/// Called `chunk_size` in some Linux documentation and code.
///
/// * Must be a power of two.
/// * Must be greater than or equal to `XDP_UMEM_MIN_CHUNK_SIZE` (`2048`).
/// * Must be less than or equal to `PAGE_SIZE` (`4096` on most systems).
///
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum ChunkSize
{
	#[allow(missing_docs)]
	_2048 = 2048,
	
	/// This is the default used by `libbpf` as `XSK_UMEM__DEFAULT_FRAME_SIZE`.
	_4096 = 4096,
}

impl Default for ChunkSize
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::_4096
	}
}

impl Into<NonZeroU32> for ChunkSize
{
	#[inline(always)]
	fn into(self) -> NonZeroU32
	{
		unsafe { transmute(self )}
	}
}

impl ChunkSize
{
	#[inline(always)]
	pub(crate) const fn mask(self) -> u64
	{
		(self as u32 as u64) - 1
	}
}
