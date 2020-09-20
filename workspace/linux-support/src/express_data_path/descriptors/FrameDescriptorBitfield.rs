// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub(crate) struct FrameDescriptorBitfield(u64);

impl Into<u64> for FrameDescriptorBitfield
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl FrameDescriptorBitfield
{
	#[inline(always)]
	pub(crate) fn for_aligned(start_of_packet: u64) -> Self
	{
		Self(start_of_packet)
	}
	
	#[inline(always)]
	pub(crate) fn for_unaligned(orig_addr: u64, offset: u64) -> Self
	{
		debug_assert_eq!(orig_addr, orig_addr & XSK_UNALIGNED_BUF_ADDR_MASK);
		
		Self(orig_addr | (offset << XSK_UNALIGNED_BUF_OFFSET_SHIFT))
	}
	
	#[inline(always)]
	pub(crate) const fn start_of_packet_if_aligned(self) -> u64
	{
		self.0
	}
	
	#[inline(always)]
	pub(crate) const fn start_of_packet_if_unaligned(self) -> u64
	{
		self.orig_addr_if_unaligned() + self.offset_if_unaligned()
	}
	
	#[inline(always)]
	pub(crate) const fn orig_addr_if_unaligned(self) -> u64
	{
		self.0 & XSK_UNALIGNED_BUF_ADDR_MASK
	}
	
	#[inline(always)]
	pub(crate) const fn offset_if_unaligned(self) -> u64
	{
		self.0 >> XSK_UNALIGNED_BUF_OFFSET_SHIFT
	}
}
