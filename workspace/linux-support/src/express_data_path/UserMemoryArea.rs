// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This combines the parameters `umem_area` and `size` in `xsk_umem__create_v0_0_4()`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct UserMemoryArea(MappedMemory);

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
	#[inline(always)]
	fn new(number_of_frames: NonZeroU32, chunk_size: ChunkSize, huge_memory_page_size: Option<Option<HugePageSize>>, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, ExpressDataPathSocketCreationError>
	{
		let length = unsafe { NonZeroU64::new_unchecked((number_of_frames.get() as u64) * (chunk_size as u32 as u64)) };
		
		let mapped_memory = MappedMemory::anonymous(length, AddressHint::any(), Protection::ReadWrite, Sharing::Private, huge_memory_page_size, false, false, defaults).map_err(ExpressDataPathSocketCreationError::CouldNotCreateUserMemory)?;
		mapped_memory.zero();
		Ok(Self(mapped_memory))
	}
	
	#[inline(always)]
	fn address_and_length(&self) -> (NonZeroU64, NonZeroU64)
	{
		(
			memory.virtual_address().into(),
			NonZeroU64::new(memory.mapped_size_in_bytes() as u64).expect("Memory can not be zero length (empty)")
		)
	}
	
	#[inline(always)]
	fn frame<'a>(&'a self, frame_headroom: FrameHeadroom, user_memory_area_relative_address: UserMemoryAreaRelativeAddress, length: usize) -> (&'a mut [u8], &'a mut [u8])
	{
		let pointer = self.frame_address(user_memory_area_relative_address).as_ptr();
		let length = descriptor.len as usize;
		
		let frame_headroom = unsafe { XXXXX };
		let frame = unsafe { from_raw_parts_mut(pointer, length) };
		(frame_headroom, frame)
	}
	
	/// Based on `xsk_umem__get_data()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// The `address` is the result of calling `xsk_umem__add_offset_to_addr()` on `xdp_desc.addr`.
	#[inline(always)]
	fn frame_address(&self, address: UserMemoryAreaRelativeAddress) -> NonNull<u8>
	{
		self.virtual_address().pointer_to(address as usize)
	}
}
