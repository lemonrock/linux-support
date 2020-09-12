// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// User memory ring queues.
#[derive(Debug)]
pub struct UserMemory
{
	/// `xsk_ring_prod`.
	fill: XskRingQueue<ProducerXskRingQueueKind, UmemDescriptor>,
	
	/// `xsk_ring_cons`.
	completion: XskRingQueue<ConsumerXskRingQueueKind, UmemDescriptor>,
	
	user_memory: UserMemoryArea,
	
	user_memory_socket_file_descriptor: ExpressDataPathSocketFileDescriptor,
}

impl UserMemory
{
	/// Based on `libbpf`'s `xsk_umem__create_v0_0_4()` (also known as `xsk_umem__create()`) in Linux source `tools/lib/bpf/xsk.c` and also `main()` and `xsk_configure_umem()` in Linux source `samples/bpf/xdp_sockuser.c`.
	///
	/// If flags contains `XdpUmemRegFlags::UnalignedFrames`, then `huge_memory_page_size` can not be `None`.
	pub fn new(number_of_frames: NonZeroU64, frame_size: FrameSize, frame_headroom: FrameHeadroom, flags: XdpUmemRegFlags, fill_ring_queue_depth: RingQueueDepth, completion_ring_queue_depth: RingQueueDepth, huge_memory_page_size: Option<Option<HugePageSize>>, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, CreationError>
	{
		if cfg!(debug_assertions)
		{
			if flags.contains(XdpUmemRegFlags::UnalignedFrames)
			{
				debug_assert!(huge_memory_page_size.is_some(), "When using XdpUmemRegFlagsUnalignedFrames in `flags`, Huge Pages must be used")
			}
		}
		
		let user_memory = UserMemoryArea::new(number_of_frames, frame_size, huge_memory_page_size, defaults)?;
		
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
	/// `needs_wake_up` should normally be `true`.
	#[inline(always)]
	pub fn to_receive_transmit(self, xdp_extended_bpf_program: Either<OwnedRedirectMapAndAttachedProgramSettings, RedirectMapAndAttachedProgram>, network_interface_index: NetworkInterfaceIndex, ring_queue_depths: ReceiveOrTransmitOrBoth<RingQueueDepth, RingQueueDepth>, queue_identifier: QueueIdentifier, defaults: &DefaultPageSizeAndHugePageSizes, force_copy: bool, force_zero_copy: bool, needs_wake_up: bool) -> Result<OwnedReceiveTransmitMemoryRingQueues, AttachProgramError>
	{
		OwnedReceiveTransmitMemoryRingQueues::new(self, xdp_extended_bpf_program, network_interface_index, ring_queue_depths, queue_identifier, defaults, force_copy, force_zero_copy, needs_wake_up)
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
