// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This combines the parameters `umem_area` and `size` in `xsk_umem__create_v0_0_4()`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserMemoryArea(MappedMemory);

impl Deref for UserMemoryArea
{
	type Target = MappedMemory;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl UserMemoryArea
{
	/// New instance.
	#[inline(always)]
	pub fn new(number_of_frames: NonZeroU64, frame_size: FrameSize, huge_memory_page_size: Option<Option<HugePageSize>>, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, CreationError>
	{
		let length = unsafe { NonZeroU64::new_unchecked(number_of_frames.get() * (frame_size as u32 as u64)) };
		
		MappedMemory::anonymous(length, AddressHint::any(), Protection::ReadWrite, Sharing::Private, huge_memory_page_size, false, false, defaults).map(Self)
	}
	
	/// The network packet should start with an `ether_header` struct, eg see the function `swap_mac_addresses()` in Linux source `samples/bpf/xdpsock_user.c`.
	///
	/// ***WARNING***: The network packet is only valid until the underlying `descriptor` is released; the lifetime `'a` is overly long.
	pub fn network_packet<'a>(&self, descriptor: &'a xdp_desc) -> &'a mut [u8]
	{
		let pointer = self.network_packet_address(descriptor.user_memory_area_relative_address()).as_ptr();
		let length = descriptor.len as usize;
		unsafe { from_raw_parts_mut(pointer, length) }
	}
	
	/// Based on `xsk_umem__get_data()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// The `address` is the result of calling `xsk_umem__add_offset_to_addr()` on `xdp_desc.addr`.
	#[inline(always)]
	fn network_packet_address(&self, address: UserMemoryAreaRelativeAddress) -> NonNull<u8>
	{
		self.virtual_address().pointer_to(address as usize)
	}
}
