// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Rx/Tx descriptor.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct xdp_desc
{
	/// This is the `addr` in the functions `xsk_umem__extract_addr()` and `xsk_umem__add_offset_to_addr()` in the Linux source `tools/lib/bpf/xsk.h`.
	///
	/// It is not actually an address, but a bit field:-
	///
	/// * The top 16 bits contain an offset.
	/// * The bottom 48 bits contain an address.
	addr: u64,
	
	pub(super) len: u32,
	
	options: u32,
}

impl Descriptor for xdp_desc
{
}

impl xdp_desc
{
	/// Based on `xsk_umem__add_offset_to_addr()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// The return value of this function is the `addr` to `xsk_umem__get_data()` in Linux source `tools/lib/bpf/xsk.h`.
	#[inline(always)]
	pub(super) fn user_memory_area_relative_address(&self) -> UserMemoryAreaRelativeAddress
	{
		self.extract_address() + self.extract_offset()
	}
	
	/// Based on `xsk_umem__extract_addr()` in Linux source `tools/lib/bpf/xsk.h`.
	#[inline(always)]
	fn extract_address(&self) -> u64
	{
		self.addr & XSK_UNALIGNED_BUF_ADDR_MASK
	}
	
	/// Based on `xsk_umem__extract_offset()` in Linux source `tools/lib/bpf/xsk.h`.
	#[inline(always)]
	fn extract_offset(&self) -> u64
	{
		self.addr >> XSK_UNALIGNED_BUF_OFFSET_SHIFT
	}
}
