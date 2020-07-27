// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Inspired by `xsk_ring_prod` in `libbpf`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XskRingQueue
{
	ring_queue_depth: RingQueueDepth,
	producer_pointer: NonNull<u32>,
	consumer_pointer: NonNull<u32>,
	ring_pointer: *mut u8,
	flags_pointer: NonNull<u32>,
	use_ring_queue_depth_for_consumer: bool,
	memory: MappedMemory,
}

impl XskRingQueue
{
	/// `xsk_ring_cons`.
	#[inline(always)]
	fn from_receive_memory_map_offsets(xsk_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, memory_map_offsets: &xdp_mmap_offsets, receive_ring_queue_depth: RingQueueDepth, defaults: &DefaultPageSizeAndHugePageSizes) -> Self
	{
		Self::from_ring_queue_offsets::<xdp_desc>(xsk_socket_file_descriptor, memory_map_offsets.receive_ring_offsets(), receive_ring_queue_depth, defaults, XDP_PGOFF_RX_RING, false)
	}
	
	/// `xsk_ring_prod`.
	#[inline(always)]
	fn from_transmit_memory_map_offsets(xsk_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, memory_map_offsets: &xdp_mmap_offsets, transmit_ring_queue_depth: RingQueueDepth, defaults: &DefaultPageSizeAndHugePageSizes) -> Self
	{
		Self::from_ring_queue_offsets::<xdp_desc>(xsk_socket_file_descriptor, memory_map_offsets.transmit_ring_offsets(), transmit_ring_queue_depth, defaults, XDP_PGOFF_TX_RING, true)
	}
	
	/// `xsk_ring_prod`.
	#[inline(always)]
	fn from_fill_memory_map_offsets(user_memory_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, memory_map_offsets: &xdp_mmap_offsets, fill_ring_queue_depth: RingQueueDepth, defaults: &DefaultPageSizeAndHugePageSizes) -> Self
	{
		Self::from_ring_queue_offsets::<UmemDescriptor>(user_memory_socket_file_descriptor, memory_map_offsets.fill_ring_offsets(), fill_ring_queue_depth, defaults, XDP_UMEM_PGOFF_FILL_RING, true)
	}
	
	/// `xsk_ring_cons`.
	#[inline(always)]
	fn from_completion_memory_map_offsets(user_memory_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, memory_map_offsets: &xdp_mmap_offsets, completion_ring_queue_depth: RingQueueDepth, defaults: &DefaultPageSizeAndHugePageSizes) -> Self
	{
		Self::from_ring_queue_offsets::<UmemDescriptor>(user_memory_socket_file_descriptor, memory_map_offsets.completion_ring_offsets(), completion_ring_queue_depth, defaults, XDP_UMEM_PGOFF_COMPLETION_RING, false)
	}
	
	#[inline(always)]
	fn from_ring_queue_offsets<D: Descriptor>(socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, ring_queue_offsets: &xdp_ring_offset, ring_queue_depth: RingQueueDepth, defaults: &DefaultPageSizeAndHugePageSizes, offset: u64, use_ring_queue_depth_for_consumer: bool) -> Self
	{
		let length = ring_queue_offsets.length_of_memory_to_map::<D>(ring_queue_depth);
		let memory = MappedMemory::from_file(socket_file_descriptor, offset, length, AddressHint::any(), Protection::ReadWrite, Sharing::Shared, None, true, false, defaults).expect("Could not memory map XDP fill ring queue");
		Self
		{
			ring_queue_depth,
			producer_pointer: ring_queue_offsets.producer_pointer(&memory),
			consumer_pointer: ring_queue_offsets.consumer_pointer(&memory),
			ring_pointer: ring_queue_offsets.ring_pointer(&memory),
			flags_pointer: ring_queue_offsets.flags_pointer(&memory),
			use_ring_queue_depth_for_consumer,
			memory,
		}
	}
	
	#[inline(always)]
	fn size(&self) -> RingQueueDepth
	{
		self.ring_queue_depth
	}
	
	#[inline(always)]
	fn mask(&self) -> u32
	{
		self.ring_queue_depth.mask()
	}
	
	#[inline(always)]
	fn cached_producer(&self) -> u32
	{
		unsafe { *self.producer_pointer.as_ref() }
	}
	
	/// For fill rings, this is actually `self.consumer_pointer.add(self.ring_size as u32 as usize)`.
	/// For tx rings, this is actually `self.consumer_pointer.add(self.ring_size as u32 as usize)`.
	#[inline(always)]
	fn cached_consumer(&self) -> u32
	{
		if self.use_ring_queue_depth_for_consumer
		{
			unsafe { *self.consumer_pointer.as_ptr().add(self.ring_queue_depth as u32 as usize) }
		}
		else
		{
			unsafe { *self.consumer_pointer.as_ref() }
		}
	}
	
	#[inline(always)]
	fn ring(&self) -> u8
	{
		unsafe { *self.ring_pointer }
	}
	
	#[inline(always)]
	fn flags(&self) -> u32
	{
		unsafe { *self.flags_pointer.as_ref() }
	}
}
