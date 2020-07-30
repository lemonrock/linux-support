// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive and transmit memory ring queues.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnedReceiveTransmitMemoryRingQueues
{
	user_memory: ManuallyDrop<UserMemory>,
	
	/// receive is `xsk_ring_cons`.
	/// transmit is `xsk_ring_prod`.
	receive_and_transmit: ReceiveOrTransmitOrBoth<XskRingQueue>,
	
	xdp_extended_bpf_program: ManuallyDrop<Option<ExtendedBpfProgramFileDescriptor>>,
}

impl Drop for OwnedReceiveTransmitMemoryRingQueues
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe
		{
			ManuallyDrop::drop(&mut self.user_memory);
			ManuallyDrop::drop(&mut self.xdp_extended_bpf_program);
		}
	}
}

impl Deref for OwnedReceiveTransmitMemoryRingQueues
{
	type Target = UserMemory;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.user_memory.deref()
	}
}

impl ReceiveTransmitMemoryRingQueues for OwnedReceiveTransmitMemoryRingQueues
{
	#[inline(always)]
	fn user_memory_and_receive_transmit(&self) -> (&UserMemory, &ReceiveOrTransmitOrBoth<XskRingQueue>)
	{
		(&self.user_memory, &self.receive_and_transmit)
	}
}

impl OwnedReceiveTransmitMemoryRingQueues
{
	#[inline(always)]
	fn new(user_memory: UserMemory, xdp_extended_bpf_program: Option<ExtendedBpfProgramFileDescriptor>, network_interface_index: NetworkInterfaceIndex, queue_identifier: QueueIdentifier, ring_queue_depths: ReceiveOrTransmitOrBoth<RingQueueDepth>, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, SocketCreationOrBindError>
	{
		let user_memory_socket_file_descriptor = &user_memory.user_memory_socket_file_descriptor;
		let receive_and_transmit = Self::construct(user_memory_socket_file_descriptor, network_interface_index, queue_identifier, ring_queue_depths, XdpSocketAddressFlags::empty(), user_memory_socket_file_descriptor.as_raw_fd(), defaults, xdp_extended_bpf_program.as_ref())?;
		Ok
		(
			Self
			{
				user_memory: ManuallyDrop::new(user_memory),
				receive_and_transmit,
				xdp_extended_bpf_program: ManuallyDrop::new(xdp_extended_bpf_program),
			}
		)
	}
	
	/// Treats `self` as master; returns a slave.
	///
	/// When all slaves have been dropped the master is dropped.
	/// This ensures the correct ordering of close for socket file descriptors.
	///
	/// The `xdp_extended_bpf_program` in use with `self` must be suitable for use with shared user memory.
	pub fn share(&self, network_interface_index: NetworkInterfaceIndex, queue_identifier: QueueIdentifier, ring_queue_depths: ReceiveOrTransmitOrBoth<RingQueueDepth>, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<SharedReceiveTransmitMemoryRingQueues, SocketCreationOrBindError>
	{
		let xsk_socket_file_descriptor = ExpressDataPathSocketFileDescriptor::new()?;
		let receive_and_transmit = Self::construct(&xsk_socket_file_descriptor, network_interface_index, queue_identifier, ring_queue_depths, XdpSocketAddressFlags::SharedUserMemory, self.user_memory.user_memory_socket_file_descriptor.as_raw_fd(), defaults)?;
		Ok
		(
			SharedReceiveTransmitMemoryRingQueues
			{
				user_memory: &self.user_memory,
				receive_and_transmit,
				xsk_socket_file_descriptor,
			}
		)
	}
	
	/// Based on `libbpf`'s `xsk_socket__create()`.
	fn construct(xsk_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, network_interface_index: NetworkInterfaceIndex, queue_identifier: QueueIdentifier, ring_queue_depths: ReceiveOrTransmitOrBoth<RingQueueDepth>, sxdp_flags: XdpSocketAddressFlags, sxdp_shared_umem_fd: RawFd, defaults: &DefaultPageSizeAndHugePageSizes, xdp_extended_bpf_program: Option<&ExtendedBpfProgramFileDescriptor>) -> Result<ReceiveOrTransmitOrBoth<XskRingQueue>, SocketBindError>
	{
		ring_queue_depths.use_value
		(
			|receive_ring_queue_depth| xsk_socket_file_descriptor.set_xdp_socket_option_receive_ring(receive_ring_queue_depth),
			|transmit_ring_queue_depth| xsk_socket_file_descriptor.set_xdp_socket_option_transmit_ring(transmit_ring_queue_depth),
		);
		
		// NOTE: Valid memory map offsets are not available until the socket options above have been set.
		let memory_map_offsets = xsk_socket_file_descriptor.get_memory_map_offsets();
		
		let receive_and_transmit = ring_queue_depths.map
		(
			|receive_ring_queue_depth| XskRingQueue::from_receive_memory_map_offsets(&xsk_socket_file_descriptor, &memory_map_offsets, receive_ring_queue_depth, defaults),
			|transmit_ring_queue_depth| XskRingQueue::from_transmit_memory_map_offsets(&xsk_socket_file_descriptor, &memory_map_offsets, transmit_ring_queue_depth, defaults),
		);
		
		let socket_address = sockaddr_xdp
		{
			sxdp_family: AF_XDP as u16,
			sxdp_flags,
			sxdp_ifindex: network_interface_index,
			sxdp_queue_id: queue_identifier,
			sxdp_shared_umem_fd,
		};
		
		const len: u32 = size_of::<sockaddr_xdp>() as u32;
		bind_socket(xsk_socket_file_descriptor, &socket_address)?;
		
		if let Some(xdp_extended_bpf_program) = xdp_extended_bpf_program
		{
			if (xsk->rx)
			{
				xsk_set_bpf_maps(xsk);
			}
		}
		
		if owned
		{
			xsk_setup_xdp_prog(xsk);
		}
		
		Ok(receive_and_transmit)
	}
}
