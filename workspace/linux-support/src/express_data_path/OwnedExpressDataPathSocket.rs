// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Owned socket.
#[derive(Debug)]
pub struct OwnedExpressDataPathSocket<ROTOB: ReceiveOrTransmitOrBoth<RingQueueDepth, RingQueueDepth> + MapReceiveOrTransmitOrBoth, RP: ReceivePoll, CA: ChunkAlignment>
{
	user_memory: ManuallyDrop<UserMemory<CA>>,
	
	express_data_path_extended_bpf_program: ManuallyDrop<RedirectMapAndAttachedProgram>,

	common: CommonSharedExpressDataPathSocket<ROTOB::To, RP>,
	
	network_interface_index: NetworkInterfaceIndex,
}

impl<ROTOB: ReceiveOrTransmitOrBoth<RingQueueDepth, RingQueueDepth> + MapReceiveOrTransmitOrBoth, RP: ReceivePoll, CA: ChunkAlignment> Drop for OwnedExpressDataPathSocket<ROTOB, RP, CA>
{
	/// Based on `libbpf`'s `xsk_socket__delete()`.
	#[inline(always)]
	fn drop(&mut self)
	{
		self.common.remove_receive_map_queue_identifier(&self.express_data_path_extended_bpf_program);
		
		unsafe
		{
			ManuallyDrop::drop(&mut self.express_data_path_extended_bpf_program);
			ManuallyDrop::drop(&mut self.common);
			ManuallyDrop::drop(&mut self.user_memory);
		}
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth<RingQueueDepth, RingQueueDepth> + MapReceiveOrTransmitOrBoth, RP: ReceivePoll, CA: ChunkAlignment> ExpressDataPathSocket<ROTOB::To, RP, CA> for OwnedExpressDataPathSocket<ROTOB, RP, CA>
{
	#[inline(always)]
	fn user_memory(&self) -> &UserMemory<CA>
	{
		&self.user_memory
	}
	
	#[inline(always)]
	fn common(&self) -> &CommonSharedExpressDataPathSocket<ROTOB::To, RP>
	{
		&self.common
	}
	
	#[inline(always)]
	fn express_data_path_socket_file_descriptor(&self) -> &ExpressDataPathSocketFileDescriptor
	{
		&self.user_memory().user_memory_socket_file_descriptor
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth<RingQueueDepth, RingQueueDepth> + MapReceiveOrTransmitOrBoth, RP: ReceivePoll, CA: ChunkAlignment> OwnedExpressDataPathSocket<ROTOB, RP, CA>
{
	// TODO: Shared sockets need locks for fill queue and completion queue.
	// TODO: Merge `receive_poll_creator`.
	// TODO: Sort out ring_queue_depths and chunk_size.
	// TODO: Sort out frame_headroom (can be used for application-specific storage); need to check if xdp_desc.len includes frame headroom.
		// TODO: Current logic creating a headroom slice seems WRONG;
		// Looking at __xsk_rcv_zv it seems that xdp_desc.addr is after headroom.
	// TODO: When we have dropped UserMemory, we should unload the XDP program attached to the network device.
		// See `remove_xdp_program()` -> `bpf_set_link_xdp_fd(-1, XDP_FLAGS_UPDATE_IF_NOEXIST)
	
	/// If flags contains `XdpUmemRegFlags::UnalignedChunks`, then `huge_memory_page_size` can not be `None`.
	///
	/// `needs_wake_up` should normally be `true`.
	/// `number_of_frames` should be `fill_ring_queue_depth` + `completion_ring_queue_depth`; if it is just `fill_ring_queue_depth` then at start up it will be impossible to transmit frames as all possible frames will have been 'gifted' to the Linux kernel.
	/// However, if doing packet forwarding, `number_of_frames == fill_ring_queue_depth == completion_ring_queue_depth`.
	///
	/// `ring_queue_depths.received()` must be the same as `fill_ring_queue_depth` (with shared queues, it could be less).
	/// `ring_queue_depths.transmit()` ought to be the same as `completion_ring_queue_depth`.
	///
	/// When this method returns, the inner `UserMemory`'s `fill_queue` is fully populated with (Ethernet) frames for receiving ethernet packets matching relative frame numbers `0` to `fill_ring_queue_depth - 1` having been 'gifted' to the Linux kernel.
	#[inline(always)]
	pub fn new(number_of_frames: NonZeroU32, chunk_size: ChunkSize, frame_headroom: FrameHeadroom, chunk_alignment: CA, fill_ring_queue_depth: RingQueueDepth, completion_ring_queue_depth: RingQueueDepth, huge_memory_page_size: Option<Option<HugePageSize>>, defaults: &DefaultPageSizeAndHugePageSizes, express_data_path_extended_bpf_program: Either<RedirectMapAndAttachedProgramSettings, RedirectMapAndAttachedProgram>, network_interface_name: NetworkInterfaceName, ring_queue_depths: ROTOB, receive_poll_creator: impl FnOnce(&ExpressDataPathSocketFileDescriptor) -> RP, queue_identifier: QueueIdentifier, force_copy: bool, force_zero_copy: bool, needs_wake_up: bool) -> Result<Self, ExpressDataPathSocketCreationError>
	{
		use self::ExpressDataPathSocketCreationError::*;
		
		let network_device_control = NetworkDeviceInputOutputControl::new(Cow::Owned(network_interface_name)).map_err(CouldNotCreateNetworkDeviceControlSocket)?;
		let network_interface_index = network_device_control.network_interface_name_to_network_interface_index().map_err(CouldNotGetValidNetworkInterfaceName)?.ok_or(NoSuchNetworkInterfaceName)?;
		
		let network_interface_maximum_transmission_unit_including_frame_check_sequence = Self::calculate_maximum_transmission_unit_including_frame_check_sequence(frame_headroom, chunk_size).map_err(|reason| CouldNotFindAnAcceptableMaximumTransmissionUnit { reason, frame_headroom, chunk_size })?;
		network_device_control.set_maximum_transmission_unit(network_interface_maximum_transmission_unit_including_frame_check_sequence)?.ok_or(NoSuchNetworkInterfaceName)?;
		
		let user_memory = UserMemory::new::<ROTOB>(number_of_frames, chunk_size, frame_headroom, network_interface_maximum_transmission_unit_including_frame_check_sequence, chunk_alignment, fill_ring_queue_depth, completion_ring_queue_depth, huge_memory_page_size, defaults)?;
		
		Self::from_user_memory(user_memory, express_data_path_extended_bpf_program, network_interface_index, ring_queue_depths, receive_poll_creator, queue_identifier, defaults, force_copy, force_zero_copy, needs_wake_up)
	}
	
	/// Treats `self` as master; returns a slave.
	///
	/// When all slaves have been dropped the master is dropped.
	/// This ensures the correct ordering of close for socket file descriptors.
	///
	/// The `express_data_path_extended_bpf_program` in use with `self` must be suitable for use with shared user memory; if not an error of `Err(AttachProgramError::AttachedXdpProgramNotSuitableForSharing)` is returned.
	///
	/// A potential bug: ***`queue_identifier` is not checked to see if it used by another instance of `SharedReceiveTransmitMemoryRingQueues`***.
	/// Adding such a check is possible using a `RefCell<Vec<QueueIdentifier>>` field but is tedious.
	pub fn share(&self, ring_queue_depths: ROTOB, receive_poll_creator: impl FnOnce(&ExpressDataPathSocketFileDescriptor) -> RP, queue_identifier: QueueIdentifier, force_copy: bool, force_zero_copy: bool, needs_wake_up: bool, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<SharedExpressDataPathSocket<ROTOB::To, RP, CA>, ExpressDataPathSocketCreationError>
	{
		debug_assert_ne!(queue_identifier, self.queue_identifier, "Re-use of owned queue identifier is not permitted");
		
		use self::ExpressDataPathSocketCreationError::*;
		
		if self.express_data_path_extended_bpf_program.is_our_owned_program_and_thus_can_not_be_shared()
		{
			return Err(AttachedXdpProgramNotSuitableForSharing)
		}
		
		let sxdp_flags = XdpSocketAddressFlags::SharedUserMemory.sxdp_flags(force_copy, force_zero_copy, needs_wake_up);
		let express_data_path_socket_file_descriptor = ExpressDataPathSocketFileDescriptor::new().map_err(CouldNotCreateUserMemorySocketFileDescriptor)?;
		let receive_and_transmit = Self::construct(&express_data_path_socket_file_descriptor, self.network_interface_index, ring_queue_depths, sxdp_flags, self.user_memory.user_memory_socket_file_descriptor(), queue_identifier, defaults)?;
		
		// Based on call to `enter_xsks_into_map()` in `main()` in Linux source `samples/bpf/xdpsock_user.c`.
		self.express_data_path_extended_bpf_program.insert_into_redirect_map_if_receive::<ROTOB>(queue_identifier, &express_data_path_socket_file_descriptor)?;
		
		Ok
		(
			SharedExpressDataPathSocket
			{
				user_memory: &self.user_memory,
				express_data_path_extended_bpf_program: &self.express_data_path_extended_bpf_program,
				common: CommonSharedExpressDataPathSocket::new(receive_and_transmit, queue_identifier, needs_wake_up, receive_poll_creator(&express_data_path_socket_file_descriptor)),
				express_data_path_socket_file_descriptor: ManuallyDrop::new(express_data_path_socket_file_descriptor),
			}
		)
	}
	
	/// Calculate the maximum transmission unit (MTU) that can be supported by eXpress Data Path.
	///
	/// Before creating an eXpress Data Path socket,
	#[inline(always)]
	fn calculate_maximum_transmission_unit_including_frame_check_sequence(frame_headroom: FrameHeadroom, chunk_size: ChunkSize) -> Result<MaximumTransmissionUnit, String>
	{
		let chunk_size = chunk_size as u32;
		debug_assert!(chunk_size >= (XDP_PACKET_HEADROOM + MaximumTransmissionUnit::EthernetInclusiveMinimumIncludingFrameCheckSequence.0));
		
		let without_frame_headroom = chunk_size.checked_sub(frame_headroom.0).ok_or(format!("frame_headroom `{:?}` leaves no space whatsoever for ethernet frame", frame_headroom))?;
		match without_frame_headroom.checked_sub(MaximumTransmissionUnit::EthernetInclusiveMinimumIncludingFrameCheckSequence.0)
		{
			None => Err(format!("frame_headroom `{:?}` leaves no space for even the minimum ethernet frame size of `{:?}`", frame_headroom, MaximumTransmissionUnit::EthernetInclusiveMinimumIncludingFrameCheckSequence)),
			
			Some(value) => Ok(MaximumTransmissionUnit(value)),
		}
	}
	
	#[inline(always)]
	fn from_user_memory(user_memory: UserMemory, express_data_path_extended_bpf_program: Either<RedirectMapAndAttachedProgramSettings, RedirectMapAndAttachedProgram>, network_interface_index: NetworkInterfaceIndex, ring_queue_depths: ROTOB, receive_poll_creator: impl FnOnce(&ExpressDataPathSocketFileDescriptor) -> RP, queue_identifier: QueueIdentifier, defaults: &DefaultPageSizeAndHugePageSizes, force_copy: bool, force_zero_copy: bool, needs_wake_up: bool) -> Result<Self, ExpressDataPathSocketCreationError>
	{
		let sxdp_flags = XdpSocketAddressFlags::empty().sxdp_flags(force_copy, force_zero_copy, needs_wake_up);
		
		let user_memory_socket_file_descriptor = user_memory.user_memory_socket_file_descriptor();
		let receive_and_transmit = Self::construct(user_memory_socket_file_descriptor, network_interface_index, ring_queue_depths, sxdp_flags, user_memory_socket_file_descriptor, queue_identifier, defaults)?;
		
		let express_data_path_extended_bpf_program = match express_data_path_extended_bpf_program
		{
			Left(settings) => RedirectMapAndAttachedProgram::new_suitable_for_owned_or_reuse_already_attached::<ROTOB>(network_interface_index, settings, queue_identifier, user_memory_socket_file_descriptor)?,
			
			Right(express_data_path_extended_bpf_program) => express_data_path_extended_bpf_program,
		};
		
		Ok
		(
			Self
			{
				common: CommonSharedExpressDataPathSocket::new(receive_and_transmit, queue_identifier, needs_wake_up, receive_poll_creator(user_memory_socket_file_descriptor)),
				user_memory: ManuallyDrop::new(user_memory),
				express_data_path_extended_bpf_program: ManuallyDrop::new(express_data_path_extended_bpf_program),
				network_interface_index,
			}
		)
	}
	
	/// Based on `libbpf`'s `xsk_socket__create()`.
	fn construct(express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, network_interface_index: NetworkInterfaceIndex, ring_queue_depths: ROTOB, sxdp_flags: XdpSocketAddressFlags, user_memory_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, queue_identifier: QueueIdentifier, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<ROTOB::To, ExpressDataPathSocketCreationError>
	{
		ring_queue_depths.use_value
		(
			|receive_ring_queue_depth| express_data_path_socket_file_descriptor.set_xdp_socket_option_receive_ring(receive_ring_queue_depth),
			|transmit_ring_queue_depth| express_data_path_socket_file_descriptor.set_xdp_socket_option_transmit_ring(transmit_ring_queue_depth),
		);
		
		// NOTE: Valid memory map offsets are not available until the socket options above have been set.
		let memory_map_offsets = express_data_path_socket_file_descriptor.get_memory_map_offsets();
		
		let receive_and_transmit = ring_queue_depths.map
		(
			|receive_ring_queue_depth| XskRingQueue::from_receive_memory_map_offsets(&express_data_path_socket_file_descriptor, &memory_map_offsets, receive_ring_queue_depth, defaults),
			|transmit_ring_queue_depth| XskRingQueue::from_transmit_memory_map_offsets(&express_data_path_socket_file_descriptor, &memory_map_offsets, transmit_ring_queue_depth, defaults),
		);
		
		let socket_address = sockaddr_xdp
		{
			sxdp_family: AF_XDP as u16,
			sxdp_flags,
			sxdp_ifindex: network_interface_index,
			sxdp_queue_id: queue_identifier,
			sxdp_shared_umem_fd: user_memory_socket_file_descriptor.as_raw_fd(),
		};
		
		bind_socket(express_data_path_socket_file_descriptor, &socket_address)?;
		
		Ok(receive_and_transmit)
	}
}
