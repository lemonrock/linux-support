// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used to register user space memory (`UMEM`).
///
/// Constructed in `libbpf` from a `struct xsk_umem_config`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub(super) struct xdp_umem_reg
{
	/// Start of packet data area.
	addr: NonZeroU64,
	
	/// Length of packet data area.
	len: NonZeroU64,
	
	/// Must be greater than or equal to `XDP_UMEM_MIN_CHUNK_SIZE` (`2048`).
	/// Must be less than or equal to `PAGE_SIZE` (`4096` on most systems).
	chunk_size: NonZeroU32,
	
	headroom: u32,
	
	flags: XdpUmemRegFlags,
}

impl xdp_umem_reg
{
	/// `memory` must be page-aligned (technically, it does not need to mmap'd but in practice it is easier to ensure it is page-aligned; it is also likely to be a large allocation for which `malloc()` is not appropriate).
	/// `frame_headroom` is usually `0`.
	#[inline(always)]
	pub(super) const fn new(memory: &MappedMemory, frame_size: FrameSize, frame_headroom: u32, flags: XdpUmemRegFlags) -> Self
	{
		Self
		{
			addr: memory.virtual_address().into(),
			len: NonZeroU64::new(memory.mapped_size_in_bytes() as u64).expect("Memory can not be zero length (empty)"),
			chunk_size: frame_size.into(),
			headroom: frame_headroom,
			flags,
		}
	}
	
}
