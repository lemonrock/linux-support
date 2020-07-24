// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub(crate) struct xdp_mmap_offsets
{
	/// Receive ring offsets.
	rx: xdp_ring_offset,
	
	/// Transmit ring offsets.
	tx: xdp_ring_offset,
	
	/// Fill ring offsets.
	fr: xdp_ring_offset,
	
	/// Completion ring offsets.
	cr: xdp_ring_offset,
}

impl xdp_mmap_offsets
{
	#[inline(always)]
	pub(crate) fn receive_ring_offsets(&self) -> &xdp_ring_offset
	{
		&self.rx
	}
	
	#[inline(always)]
	pub(crate) fn transmit_ring_offsets(&self) -> &xdp_ring_offset
	{
		&self.tx
	}
	
	#[inline(always)]
	pub(crate) fn fill_ring_offsets(&self) -> &xdp_ring_offset
	{
		&self.fr
	}
	
	#[inline(always)]
	pub(crate) fn completion_ring_offsets(&self) -> &xdp_ring_offset
	{
		&self.cr
	}
}
