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
	fn new(number_of_frames: NonZeroU32, chunk_size: AlignedChunkSize, huge_memory_page_size: Option<Option<HugePageSize>>, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, ExpressDataPathSocketCreationError>
	{
		let length = unsafe { NonZeroU64::new_unchecked((number_of_frames.get() as u64) * (chunk_size as u32 as u64)) };
		
		let mapped_memory = MappedMemory::anonymous(length, AddressHint::any(), Protection::ReadWrite, Sharing::Private, huge_memory_page_size, false, false, defaults).map_err(ExpressDataPathSocketCreationError::CouldNotCreateUserMemory)?;
		mapped_memory.zero();
		Ok(Self(mapped_memory))
	}
	
	#[inline(always)]
	fn slice(&self, start_relative_address: u64, length: usize) -> &[u8]
	{
		unsafe { from_raw_parts(self.slice_starts_from(start_relative_address, length).into(), length) }
	}
	
	#[inline(always)]
	fn slice_mut(&self, start_relative_address: u64, length: usize) -> &mut [u8]
	{
		unsafe { from_raw_parts_mut(self.slice_starts_from(start_relative_address, length).into(), length) }
	}
	
	#[inline(always)]
	fn slice_starts_from(&self, start_relative_address: u64, length: usize) -> VirtualAddress
	{
		let starts_from = self.virtual_address().add(start_relative_address);
		if cfg!(debug_assertions)
		{
			let end_address = self.virtual_address().add(self.mapped_size_in_bytes());
			debug_assert!(starts_from <= end_address);
			
			debug_assert!(starts_from.add(length) <= end_address);
		}
		starts_from
	}
	
	#[inline(always)]
	fn start_address_and_length(&self) -> (NonZeroU64, NonZeroU64)
	{
		(
			self.virtual_address().into(),
			NonZeroU64::new(memory.mapped_size_in_bytes() as u64).expect("Memory can not be zero length (empty)")
		)
	}
}
