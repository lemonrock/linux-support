// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Owned socket.
#[derive(Debug)]
pub struct OwnedExpressDataPathSocket<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment>
{
	user_memory: ManuallyDrop<UserMemory<CA>>,
	
	redirect_map_and_attached_program: ManuallyDrop<RedirectMapAndAttachedProgram>,

	common: CommonExpressDataPathSocket<ROTOB>,
	
	network_interface_index: NetworkInterfaceIndex,
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Receives<CommonReceiveOnly<RP>>, CA: ChunkAlignment, RP: ReceivePoll> Drop for OwnedExpressDataPathSocket<RingQueueDepths, CA>
{
	/// Based on `libbpf`'s `xsk_socket__delete()`.
	#[inline(always)]
	fn drop(&mut self)
	{
		self.common.remove_receive_map_queue_identifier(&self.redirect_map_and_attached_program);
		self.manually_drop()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Transmits<CommonTransmitOnly>, CA: ChunkAlignment> Drop for OwnedExpressDataPathSocket<RingQueueDepths, CA>
{
	/// Based on `libbpf`'s `xsk_socket__delete()`.
	#[inline(always)]
	fn drop(&mut self)
	{
		self.manually_drop()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment> OwnedExpressDataPathSocket<ROTOB, CA>
{
	/// Based on `libbpf`'s `xsk_socket__delete()`.
	#[inline(always)]
	fn manually_drop(&mut self)
	{
		unsafe
		{
			ManuallyDrop::drop(&mut self.common);
			ManuallyDrop::drop(&mut self.user_memory);
			ManuallyDrop::drop(&mut self.redirect_map_and_attached_program);
		}
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment> ExpressDataPathSocket<ROTOB, CA> for OwnedExpressDataPathSocket<RingQueueDepths, CA>
{
	#[inline(always)]
	fn user_memory(&self) -> &UserMemory<CA>
	{
		&self.user_memory
	}
	
	#[inline(always)]
	fn common(&self) -> &CommonExpressDataPathSocket<RingQueueDepths::ReceiveOrTransmitOrBoth>
	{
		&self.common
	}
	
	#[inline(always)]
	fn express_data_path_socket_file_descriptor(&self) -> &ExpressDataPathSocketFileDescriptor
	{
		&self.user_memory().user_memory_socket_file_descriptor
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Receives<CommonReceiveOnly<RP>>, CA: ChunkAlignment, RP: ReceivePoll> ReceivesExpressDataPathSocket<ROTOB, CA> for OwnedExpressDataPathSocket<RingQueueDepths, CA>
{
	#[inline(always)]
	fn lock_fill_queue(&self)
	{
	}
	
	#[inline(always)]
	fn unlock_fill_queue(&self)
	{
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Transmits<CommonTransmitOnly>, CA: ChunkAlignment> TransmitsExpressDataPathSocket<ROTOB, CA> for OwnedExpressDataPathSocket<RingQueueDepths, CA>
{
	#[inline(always)]
	fn lock_completion_queue(&self)
	{
	}
	
	#[inline(always)]
	fn unlock_completion_queue(&self)
	{
	}
}

impl<RingQueueDepths: FillOrCompletionOrBothRingQueueDepths + CreateReceiveOrTransmitOrBoth, CA: ChunkAlignment> OwnedExpressDataPathSocket<RingQueueDepths::ReceiveOrTransmitOrBoth, CA>
{
	// TODO: Two different kinds of queues - fill/complete, rx/tx - need two types, not just RingQueueDepths
	// TODO: insert receive_queue_identifier after binding socket!
	// TODO: Sort out number of frames - either fill + completion depth, or, if fwd'ing, either fill or completion but fill == completion
		// TODO: Can be larger, eg for scratch space, but what's the point?
	// TODO: Sort out frame_headroom (can be used for application-specific storage); need to check if xdp_desc.len includes frame headroom.
		// TODO: Current logic creating a headroom slice seems WRONG;
		// Looking at __xsk_rcv_zv it seems that xdp_desc.addr is after headroom.
	// TODO: Sort out sharing - the OwnedExpressDataPathSocket still needs to be used as one-of the shared instances!
	
	/// If flags contains `XdpUmemRegFlags::UnalignedChunks`, then `huge_memory_page_size` can not be `None`.
	///
	/// `number_of_frames` should be `fill_ring_queue_depth` + `completion_ring_queue_depth`; if it is just `fill_ring_queue_depth` then at start up it will be impossible to transmit frames as all possible frames will have been 'gifted' to the Linux kernel.
	/// However, if doing packet forwarding, `number_of_frames == fill_ring_queue_depth == completion_ring_queue_depth`.
	///
	/// `ring_queue_depths.received()` must be the same as `fill_ring_queue_depth` (with shared queues, it could be less).
	/// `ring_queue_depths.transmit()` ought to be the same as `completion_ring_queue_depth`.
	///
	/// When this method returns, the inner `UserMemory`'s `fill_queue` is fully populated with (Ethernet) frames for receiving ethernet packets matching relative frame numbers `0` to `fill_ring_queue_depth - 1` having been 'gifted' to the Linux kernel.
	///
	/// Even if we are never transmitting, we still have to specify and create a `completion_ring_queue_depth`!
	#[inline(always)]
	pub fn new(number_of_frames: NonZeroU32, chunk_size: ChunkSize, frame_headroom: FrameHeadroom, chunk_alignment: CA, fill_or_completion_or_both_ring_queue_depths: RingQueueDepths, huge_memory_page_size: Option<Option<HugePageSize>>, defaults: &DefaultPageSizeAndHugePageSizes, redirect_map_and_attached_program: Either<RedirectMapAndAttachedProgramSettings, RedirectMapAndAttachedProgram>, network_interface_name: NetworkInterfaceName, receive_or_transmit_or_both_ring_queue_depths: RingQueueDepths, queue_identifier: QueueIdentifier, force_copy: bool, force_zero_copy: bool, arguments: RingQueueDepths::Arguments) -> Result<Self, ExpressDataPathSocketCreationError>
	{
		use self::ExpressDataPathSocketCreationError::*;
		
		let network_device_control = NetworkDeviceInputOutputControl::new(Cow::Owned(network_interface_name)).map_err(CouldNotCreateNetworkDeviceControlSocket)?;
		let network_interface_index = network_device_control.network_interface_name_to_network_interface_index().map_err(CouldNotGetValidNetworkInterfaceName)?.ok_or(NoSuchNetworkInterfaceName)?;
		
		let network_interface_maximum_transmission_unit_including_frame_check_sequence = Self::calculate_maximum_transmission_unit_including_frame_check_sequence(frame_headroom, chunk_size).map_err(|reason| CouldNotFindAnAcceptableMaximumTransmissionUnit { reason, frame_headroom, chunk_size })?;
		network_device_control.set_maximum_transmission_unit(network_interface_maximum_transmission_unit_including_frame_check_sequence)?.ok_or(NoSuchNetworkInterfaceName)?;
		
		let user_memory = UserMemory::new(number_of_frames, chunk_size, frame_headroom, network_interface_maximum_transmission_unit_including_frame_check_sequence, chunk_alignment, fill_or_completion_or_both_ring_queue_depths, huge_memory_page_size, defaults)?;
		
		Self::owned(user_memory, redirect_map_and_attached_program, network_interface_index, receive_or_transmit_or_both_ring_queue_depths, queue_identifier, defaults, force_copy, force_zero_copy, arguments)
	}
	
	#[inline(always)]
	fn owned(user_memory: UserMemory, redirect_map_and_attached_program: Either<RedirectMapAndAttachedProgramSettings, RedirectMapAndAttachedProgram>, network_interface_index: NetworkInterfaceIndex, receive_or_transmit_or_both_ring_queue_depths: RingQueueDepths, queue_identifier: QueueIdentifier, defaults: &DefaultPageSizeAndHugePageSizes, force_copy: bool, force_zero_copy: bool, arguments: RingQueueDepths::Arguments) -> Result<Self, ExpressDataPathSocketCreationError>
	{
		let redirect_map_and_attached_program = match redirect_map_and_attached_program
		{
			Left(settings) => RedirectMapAndAttachedProgram::new_suitable_for_owned_or_reuse_already_attached::<RingQueueDepths>(network_interface_index, settings, queue_identifier, user_memory_socket_file_descriptor)?,
			
			Right(redirect_map_and_attached_program) => redirect_map_and_attached_program,
		};
		
		Ok
		(
			Self
			{
				common:
				{
					let user_memory_socket_file_descriptor = user_memory.user_memory_socket_file_descriptor();
					let express_data_path_socket_file_descriptor = user_memory_socket_file_descriptor;
					
					CommonExpressDataPathSocket::new(express_data_path_socket_file_descriptor, network_interface_index, receive_or_transmit_or_both_ring_queue_depths, XdpSocketAddressFlags::empty(), force_copy, force_zero_copy, user_memory_socket_file_descriptor, queue_identifier, defaults, &redirect_map_and_attached_program, arguments)?
				},
				
				user_memory: ManuallyDrop::new(user_memory),
				
				redirect_map_and_attached_program: ManuallyDrop::new(redirect_map_and_attached_program),
				
				network_interface_index,
			}
		)
	}
	
	fn shared(&self, receive_or_transmit_or_both_ring_queue_depths: RingQueueDepths, queue_identifier: QueueIdentifier, force_copy: bool, force_zero_copy: bool, defaults: &DefaultPageSizeAndHugePageSizes, arguments: RingQueueDepths::Arguments) -> Result<(CommonExpressDataPathSocket<RingQueueDepths::ReceiveOrTransmitOrBoth>, ManuallyDrop<ExpressDataPathSocketFileDescriptor>), ExpressDataPathSocketCreationError>
	{
		use self::ExpressDataPathSocketCreationError::*;
		
		debug_assert_ne!(queue_identifier, self.queue_identifier, "Re-use of owned queue identifier is not permitted");
		
		if self.redirect_map_and_attached_program.is_our_owned_program_and_thus_can_not_be_shared()
		{
			return Err(AttachedExpressDataPathProgramNotSuitableForSharing)
		}
		
		let express_data_path_socket_file_descriptor = ExpressDataPathSocketFileDescriptor::new().map_err(CouldNotCreateUserMemorySocketFileDescriptor)?;
		
		Ok
		(
			(
				CommonExpressDataPathSocket::new(&express_data_path_socket_file_descriptor, self.network_interface_index, receive_or_transmit_or_both_ring_queue_depths, XdpSocketAddressFlags::SharedUserMemory, force_copy, force_zero_copy, self.user_memory.user_memory_socket_file_descriptor(), queue_identifier, defaults, &self.redirect_map_and_attached_program, arguments)?,
				
				ManuallyDrop::new(express_data_path_socket_file_descriptor)
			)
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
}
