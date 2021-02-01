// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Instance.
#[derive(Debug)]
pub struct ExpressDataPathInstance<ROTOB: ReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue>
{
	user_memory: ManuallyDrop<UserMemory<FFQ>>,
	
	redirect_map_and_attached_program: ManuallyDrop<RedirectMapAndAttachedProgram>,
	
	network_interface_index: NetworkInterfaceIndex,
	
	force_copy: bool,
	
	force_zero_copy: bool,

	marker: PhantomData<ROTOB>,
}

impl<ROTOB: ReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue> ExpressDataPathInstance<ROTOB, FFQ>
{
	/// Converts to single-threaded owned instance.
	pub fn owned<RingQueueDepths: CreateReceiveOrTransmitOrBoth<ReceiveOrTransmitOrBoth=ROTOB>>(self, receive_or_transmit_or_both_ring_queue_depths: RingQueueDepths, queue_identifier: QueueIdentifier, default_page_size: PageSize, arguments: RingQueueDepths::Arguments) -> Result<OwnedExpressDataPathSocket<ROTOB, FFQ>, ExpressDataPathSocketCreationError>
	{
		Ok
		(
			OwnedExpressDataPathSocket
			{
				common:
				{
					let user_memory_socket_file_descriptor = self.user_memory.user_memory_socket_file_descriptor();
					let express_data_path_socket_file_descriptor = user_memory_socket_file_descriptor;
				
					CommonExpressDataPathSocket::new(express_data_path_socket_file_descriptor, self.network_interface_index, receive_or_transmit_or_both_ring_queue_depths, XdpSocketAddressFlags::empty(), self.force_copy, self.force_zero_copy, user_memory_socket_file_descriptor, queue_identifier, default_page_size, &self.redirect_map_and_attached_program, arguments)?
				},
				
				instance: ManuallyDrop::new(self),
			}
		)
	}
	
	/// Converts to instance that can then create per-thread instances (sic).
	///
	/// If using a shared instance, then each shared instance should:-
	///
	/// * Have a transmit queue depth equal to the completion queue depth;
	/// * Have a receive queue depth equal to the fill queue depth;
	///
	/// Additionally, the `number_of_chunks` should be equal to the total of all shared instances transmit queue depths + receive queue depths, unless only forwarding immediately, in which case it can be half this value.
	#[inline(always)]
	pub fn shareable(self, number_of_threads_guess: NonZeroUsize) -> ShareableExpressDataPathInstance<ROTOB, FFQ>
	{
		ShareableExpressDataPathInstance(Arc::new((self, BestForCompilationTargetSpinLock::default(), BestForCompilationTargetSpinLock::default(), Mutex::new(HashSet::with_capacity(number_of_threads_guess.get())))))
	}
	
	#[inline(always)]
	fn shared<RingQueueDepths: CreateReceiveOrTransmitOrBoth<ReceiveOrTransmitOrBoth=ROTOB>>(&self, receive_or_transmit_or_both_ring_queue_depths: RingQueueDepths, queue_identifier: QueueIdentifier, default_page_size: PageSize, arguments: RingQueueDepths::Arguments, instance: &ShareableExpressDataPathInstance<RingQueueDepths::ReceiveOrTransmitOrBoth, FFQ>, express_data_path_socket_file_descriptor: ExpressDataPathSocketFileDescriptor) -> Result<SharedExpressDataPathSocket<ROTOB, FFQ>, ExpressDataPathSocketCreationError>
	{
		use self::ExpressDataPathSocketCreationError::*;
		
		let redirect_map_and_attached_program = &self.redirect_map_and_attached_program;
		
		if redirect_map_and_attached_program.is_our_owned_program_and_thus_can_not_be_shared()
		{
			return Err(AttachedExpressDataPathProgramNotSuitableForSharing)
		}
		
		Ok
		(
			SharedExpressDataPathSocket
			{
				common:
				{
					let user_memory_socket_file_descriptor = self.user_memory.user_memory_socket_file_descriptor();
					
					CommonExpressDataPathSocket::new(&express_data_path_socket_file_descriptor, self.network_interface_index, receive_or_transmit_or_both_ring_queue_depths, XdpSocketAddressFlags::SharedUserMemory, self.force_copy, self.force_zero_copy, user_memory_socket_file_descriptor, queue_identifier, default_page_size, redirect_map_and_attached_program, arguments)?
				},
				
				instance: instance.clone(),
				
				express_data_path_socket_file_descriptor: ManuallyDrop::new(express_data_path_socket_file_descriptor),
				
				queue_identifier,
			}
		)
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue> ExpressDataPathInstance<ROTOB, FFQ>
{
	/// For interest.
	#[inline(always)]
	pub fn calculate_maximum_transmission_unit_payload_size(chunk_size: FFQ::CS, frame_headroom: FrameHeadroom) -> Result<MaximumTransmissionUnitPayloadSize, ParseNumberError>
	{
		chunk_size.calculate_maximum_transmission_unit_payload_size(frame_headroom)
	}
	
	/// New instance.
	#[inline(always)]
	pub fn new<RingQueueDepths: FillOrCompletionOrBothRingQueueDepths + CreateReceiveOrTransmitOrBoth<ReceiveOrTransmitOrBoth=ROTOB>>(number_of_chunks: NonZeroU32, frame_headroom: FrameHeadroom, chunk_size: FFQ::CS, fill_or_completion_or_both_ring_queue_depths: RingQueueDepths, default_page_size: PageSize, user_memory_area_page_size_or_huge_page_size_settings: &PageSizeOrHugePageSizeSettings,  redirect_map_and_attached_program: Either<RedirectMapAndAttachedProgramSettings, RedirectMapAndAttachedProgram>, network_interface_name: NetworkInterfaceName, force_copy: bool, force_zero_copy: bool) -> Result<Self, ExpressDataPathSocketCreationError>
	{
		let (network_interface_index, maximum_transmission_unit_payload_size) = Self::network_interface_index_and_maximum_transmission_unit_payload_size(network_interface_name)?;
		
		let user_memory = ManuallyDrop::new(UserMemory::new(number_of_chunks, chunk_size, frame_headroom, maximum_transmission_unit_payload_size, fill_or_completion_or_both_ring_queue_depths, user_memory_area_page_size_or_huge_page_size_settings, default_page_size)?);
		
		Ok
		(
			Self
			{
				redirect_map_and_attached_program: ManuallyDrop::new
				(
					match redirect_map_and_attached_program
					{
						Left(settings) => RedirectMapAndAttachedProgram::new_suitable_for_owned_or_reuse_already_attached(network_interface_index, settings)?,
						
						Right(redirect_map_and_attached_program) => redirect_map_and_attached_program,
					}
				),
				
				network_interface_index,
				
				force_copy,
				
				force_zero_copy,
				
				marker: PhantomData,
				
				user_memory,
			}
		)
	}
	
	#[inline(always)]
	fn network_interface_index_and_maximum_transmission_unit_payload_size(network_interface_name: NetworkInterfaceName) -> Result<(NetworkInterfaceIndex, MaximumTransmissionUnitPayloadSize), ExpressDataPathSocketCreationError>
	{
		use self::ExpressDataPathSocketCreationError::*;
		
		Ok
		(
			(
				{
					let network_device_control = NetworkDeviceInputOutputControl::new(Cow::Borrowed(&network_interface_name)).map_err(CouldNotCreateNetworkDeviceControlSocket)?;
					network_device_control.network_interface_name_to_network_interface_index().map_err(CouldNotGetValidNetworkInterfaceName)?.ok_or(NoSuchNetworkInterfaceName)?
				},
				{
					let mut netlink_socket_file_descriptor = NetlinkSocketFileDescriptor::open().map_err(CouldNotCreateNetlinkSocket)?;
					let get_link_message_data = RouteNetlinkProtocol::get_link(&mut netlink_socket_file_descriptor, |get_link_message_data| get_link_message_data.network_interface_name == network_interface_name).map_err(NetlinkGetLinksFailed )?.ok_or(NoSuchNetworkInterfaceName)?;
					get_link_message_data.maximum_transmission_unit
				}
			)
		)
	}
}
