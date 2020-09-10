// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// TODO: How do we use this program?
/// Receive and transmit memory ring queues.
#[derive(Debug)]
pub struct OwnedReceiveTransmitMemoryRingQueues
{
	user_memory: ManuallyDrop<UserMemory>,
	
	/// receive is `xsk_ring_cons`.
	/// transmit is `xsk_ring_prod`.
	receive_and_transmit: ManuallyDrop<ReceiveOrTransmitOrBoth<XskRingQueue>>,
	
	xdp_extended_bpf_program: ManuallyDrop<RedirectMapAndAttachedProgram>,
	
	queue_identifier: QueueIdentifier,
}

impl Drop for OwnedReceiveTransmitMemoryRingQueues
{
	/// Based on `libbpf`'s `xsk_socket__delete()`.
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.receive_and_transmit.is_receive_or_both()
		{
			// Based on `libbpf`'s `xsk_delete_bpf_maps()`.
			let _ignored = self.xdp_extended_bpf_program.redirect_map.delete(self.queue_identifier);
		}
		
		unsafe
		{
			ManuallyDrop::drop(&mut self.xdp_extended_bpf_program);
			ManuallyDrop::drop(&mut self.receive_and_transmit);
			ManuallyDrop::drop(&mut self.user_memory);
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
	fn new(user_memory: UserMemory, xdp_extended_bpf_program: Either<OwnedRedirectMapAndAttachedProgramSettings, RedirectMapAndAttachedProgram>, network_interface_index: NetworkInterfaceIndex, ring_queue_depths: ReceiveOrTransmitOrBoth<RingQueueDepth>, queue_identifier: QueueIdentifier, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, AttachProgramError>
	{
		let user_memory_socket_file_descriptor = &user_memory.user_memory_socket_file_descriptor;
		let receive_and_transmit = Self::construct(user_memory_socket_file_descriptor, network_interface_index, ring_queue_depths, XdpSocketAddressFlags::empty(), user_memory_socket_file_descriptor.as_raw_fd(), queue_identifier, defaults)?;
		
		let xdp_extended_bpf_program = match xdp_extended_bpf_program
		{
			Left(settings) =>
			{
				let insert_into_redirect_map_if_receive = if ring_queue_depths.is_receive_or_both()
				{
					Some((queue_identifier, user_memory_socket_file_descriptor))
				}
				else
				{
					None
				};
				
				RedirectMapAndAttachedProgram::new_suitable_for_owned_or_reuse_already_attached(network_interface_index, settings, insert_into_redirect_map_if_receive)?
			}
			
			Right(xdp_extended_bpf_program) => xdp_extended_bpf_program,
		};
		
		Ok
		(
			Self
			{
				user_memory: ManuallyDrop::new(user_memory),
				receive_and_transmit: ManuallyDrop::new(receive_and_transmit),
				xdp_extended_bpf_program: ManuallyDrop::new(xdp_extended_bpf_program),
				queue_identifier,
			}
		)
	}
	
	/// Treats `self` as master; returns a slave.
	///
	/// When all slaves have been dropped the master is dropped.
	/// This ensures the correct ordering of close for socket file descriptors.
	///
	/// The `xdp_extended_bpf_program` in use with `self` must be suitable for use with shared user memory; if not an error of `Err(AttachProgramError::AttachedXdpProgramNotSuitableForSharing)` is returned.
	///
	/// A potential bug: ***`queue_identifier` is not checked to see if it used by another instance of `SharedReceiveTransmitMemoryRingQueues`.***.
	/// Adding such a check is possible using a `RefCell<Vec<QueueIdentifier>>` field but is tedious.
	pub fn share(&self, network_interface_index: NetworkInterfaceIndex, ring_queue_depths: ReceiveOrTransmitOrBoth<RingQueueDepth>, queue_identifier: QueueIdentifier, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<SharedReceiveTransmitMemoryRingQueues, AttachProgramError>
	{
		debug_assert_ne!(queue_identifier, self.queue_identifier, "Re-use of owned queue identifier is not permitted");
		
		if self.xdp_extended_bpf_program.deref().is_our_owned_program_and_thus_can_not_be_shared()
		{
			return Err(AttachProgramError::AttachedXdpProgramNotSuitableForSharing)
		}
		
		let xsk_socket_file_descriptor = ExpressDataPathSocketFileDescriptor::new()?;
		let receive_and_transmit = Self::construct(&xsk_socket_file_descriptor, network_interface_index, ring_queue_depths, XdpSocketAddressFlags::SharedUserMemory, self.user_memory.user_memory_socket_file_descriptor.as_raw_fd(), queue_identifier, defaults)?;
		Ok
		(
			SharedReceiveTransmitMemoryRingQueues
			{
				user_memory: &self.user_memory,
				xdp_extended_bpf_program: &self.xdp_extended_bpf_program,
				receive_and_transmit: ManuallyDrop::new(receive_and_transmit),
				xsk_socket_file_descriptor: ManuallyDrop::new(xsk_socket_file_descriptor),
				queue_identifier,
			}
		)
	}
	
	/// Based on `libbpf`'s `xsk_socket__create()`.
	fn construct(user_memory_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, network_interface_index: NetworkInterfaceIndex, ring_queue_depths: ReceiveOrTransmitOrBoth<RingQueueDepth>, sxdp_flags: XdpSocketAddressFlags, sxdp_shared_umem_fd: RawFd, queue_identifier: QueueIdentifier, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<ReceiveOrTransmitOrBoth<XskRingQueue>, SocketBindError>
	{
		ring_queue_depths.use_value
		(
			|receive_ring_queue_depth| user_memory_socket_file_descriptor.set_xdp_socket_option_receive_ring(receive_ring_queue_depth),
			|transmit_ring_queue_depth| user_memory_socket_file_descriptor.set_xdp_socket_option_transmit_ring(transmit_ring_queue_depth),
		);
		
		// NOTE: Valid memory map offsets are not available until the socket options above have been set.
		let memory_map_offsets = user_memory_socket_file_descriptor.get_memory_map_offsets();
		
		let receive_and_transmit = ring_queue_depths.map
		(
			|receive_ring_queue_depth| XskRingQueue::from_receive_memory_map_offsets(&user_memory_socket_file_descriptor, &memory_map_offsets, receive_ring_queue_depth, defaults),
			|transmit_ring_queue_depth| XskRingQueue::from_transmit_memory_map_offsets(&user_memory_socket_file_descriptor, &memory_map_offsets, transmit_ring_queue_depth, defaults),
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
		bind_socket(user_memory_socket_file_descriptor, &socket_address)?;
		
		Ok(receive_and_transmit)
	}
}
