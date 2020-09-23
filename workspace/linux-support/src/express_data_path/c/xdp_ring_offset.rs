// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct xdp_ring_offset
{
	producer: UserMemoryAreaRelativeAddress,
	
	consumer: UserMemoryAreaRelativeAddress,
	
	desc: UserMemoryAreaRelativeAddress,
	
	flags: UserMemoryAreaRelativeAddress,
}

impl xdp_ring_offset
{
	#[inline(always)]
	pub(crate) fn length_of_memory_to_map<D: Descriptor>(&self, ring_queue_depth: RingQueueDepth) -> NonZeroU64
	{
		let end = self.desc + ring_queue_depth.memory_length::<D>();
		end.try_into().unwrap()
	}
	
	#[inline(always)]
	pub(crate) fn producer_pointer(&self, ring_mapped_memory: &MappedMemory) -> NonNull<u32>
	{
		self.pointer_non_null_u32(ring_mapped_memory, self.producer)
	}
	
	#[inline(always)]
	pub(crate) fn consumer_pointer(&self, ring_mapped_memory: &MappedMemory) -> NonNull<u32>
	{
		self.pointer_non_null_u32(ring_mapped_memory, self.consumer)
	}
	
	#[inline(always)]
	pub(crate) fn ring_pointer<D: Descriptor>(&self, ring_mapped_memory: &MappedMemory) -> *mut D
	{
		self.pointer(ring_mapped_memory, self.desc).into()
	}
	
	#[inline(always)]
	pub(crate) fn flags_pointer(&self, ring_mapped_memory: &MappedMemory) -> NonNull<u32>
	{
		self.pointer_non_null_u32(ring_mapped_memory, self.flags)
	}
	
	#[inline(always)]
	fn pointer_non_null_u32(&self, ring_mapped_memory: &MappedMemory, field: UserMemoryAreaRelativeAddress) -> NonNull<u32>
	{
		self.pointer(ring_mapped_memory, field).into()
	}
	
	#[inline(always)]
	fn pointer(&self, ring_mapped_memory: &MappedMemory, field: UserMemoryAreaRelativeAddress) -> VirtualAddress
	{
		(ring_mapped_memory.virtual_address() + field)
	}
}
