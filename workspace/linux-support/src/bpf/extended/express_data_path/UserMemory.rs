// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// User memory ring queues.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserMemory
{
	/// `xsk_ring_prod`.
	fill: XskRingQueue,
	
	/// `xsk_ring_cons`.
	completion: XskRingQueue,
	
	user_memory: MappedMemory,
	
	user_memory_socket_file_descriptor: ExpressDataPathSocketFileDescriptor,
}

impl UserMemory
{
	/// Based on `libbpf`'s `xsk_umem__create_v0_0_4()`.
	pub fn new(user_memory: MappedMemory, frame_size: FrameSize, frame_headroom: u32, flags: XdpUmemRegFlags, fill_ring_queue_depth: RingQueueDepth, completion_ring_queue_depth: RingQueueDepth, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, CreationError>
	{
		let user_memory_socket_file_descriptor = ExpressDataPathSocketFileDescriptor::new()?;
		let configuration = xdp_umem_reg::new(&user_memory, frame_size, frame_headroom, flags);
		user_memory_socket_file_descriptor.register_user_space_memory(&configuration, fill_ring_queue_depth, completion_ring_queue_depth);
		let memory_map_offsets = user_memory_socket_file_descriptor.get_memory_map_offsets();
		
		Ok
		(
			Self
			{
				fill: XskRingQueue::from_fill_memory_map_offsets(&user_memory_socket_file_descriptor, &memory_map_offsets, fill_ring_queue_depth, defaults),
				completion: XskRingQueue::from_completion_memory_map_offsets(&user_memory_socket_file_descriptor, &memory_map_offsets, completion_ring_queue_depth, defaults),
				user_memory,
				user_memory_socket_file_descriptor,
			}
		)
	}
	
	/// Makes the necessary calls to create something suitable to be used as a XDP program with a socket.
	///
	/// If `xdp_extended_bpf_program` is specified it is attached to `network_interface_index`.
	/// Ordinarily, _do not specify_ `xdp_extended_bpf_program`; a suitable instance will be created and bound to `network_interface_index` and `queue_identifier` (the latter only if `ring_queue_depths` is `ReceiveOnly` or `Both`).
	#[inline(always)]
	pub fn to_receive_transmit(self, xdp_extended_bpf_program: Option<RedirectMapAndAttachedProgram>, network_interface_index: NetworkInterfaceIndex, ring_queue_depths: ReceiveOrTransmitOrBoth<RingQueueDepth>, queue_identifier: QueueIdentifier, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<OwnedReceiveTransmitMemoryRingQueues, AttachProgramError>
	{
		OwnedReceiveTransmitMemoryRingQueues::new(self, xdp_extended_bpf_program, network_interface_index, ring_queue_depths, queue_identifier, defaults)
	}
	
	/// Statistics.
	#[inline(always)]
	pub fn statistics(&self) -> xdp_statistics
	{
		self.user_memory_socket_file_descriptor.statistics()
	}
	
	/// Options.
	#[inline(always)]
	pub fn options(&self) -> xdp_options
	{
		self.user_memory_socket_file_descriptor.options()
	}
}
