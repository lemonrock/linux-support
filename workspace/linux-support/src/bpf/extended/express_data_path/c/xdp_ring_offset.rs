// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub(super) struct xdp_ring_offset
{
	producer: u64,
	
	consumer: u64,
	
	desc: u64,
	
	flags: u64,
}

impl xdp_ring_offset
{
	#[inline(always)]
	pub(crate) fn length_of_memory_to_map<D: Descriptor>(&self, ring_queue_depth: RingQueueDepth) -> NonZeroU64
	{
		unsafe { NonZeroU64::new_unchecked(self.desc + fill_ring_queue_depth.memory_length::<D>()) }
	}
	
	#[inline(always)]
	pub(crate) fn producer_pointer(&self, ring_mapped_memory: &MappedMemory) -> NonNull<u32>
	{
		ring_mapped_memory.virtual_address().add(self.producer).into()
	}
	
	#[inline(always)]
	pub(crate) fn consumer_pointer(&self, ring_mapped_memory: &MappedMemory) -> NonNull<u32>
	{
		ring_mapped_memory.virtual_address().add(self.consumer).into()
	}
	
	#[inline(always)]
	pub(crate) fn ring_pointer(&self, ring_mapped_memory: &MappedMemory) -> *mut c_void
	{
		ring_mapped_memory.virtual_address().add(self.desc).into()
	}
	
	#[inline(always)]
	pub(crate) fn flags_pointer(&self, ring_mapped_memory: &MappedMemory) -> NonNull<u32>
	{
		ring_mapped_memory.virtual_address().add(self.flags).into()
	}
}
