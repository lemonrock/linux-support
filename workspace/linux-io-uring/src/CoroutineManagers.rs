// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Coroutine managers partial abstraction.
pub struct CoroutineManagers<HeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>, StackSizeAccept: MemorySize>
(
	CoroutineManager<HeapSize, StackSizeAccept, GTACSA, AcceptCoroutine<sockaddr_in>, AcceptCoroutineInformation>,
	CoroutineManager<HeapSize, StackSizeAccept, GTACSA, AcceptCoroutine<sockaddr_in6>, AcceptCoroutineInformation>,
	CoroutineManager<HeapSize, StackSizeAccept, GTACSA, AcceptCoroutine<UnixSocketAddress<PathBuf>>, AcceptCoroutineInformation>,
);

impl<HeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>, StackSizeAccept: MemorySize> CoroutineManagers<HeapSize, GTACSA, StackSizeAccept>
{
	pub fn new
	(
		global_allocator: &'static GTACSA,
		defaults: &DefaultPageSizeAndHugePageSizes,
		io_uring: &Rc<IoUring<'static>>,
		queues: &Queues<(), DequeuedMessageProcessingError>,
		transmission_control_protocol_over_internet_protocol_version_4_server_listeners: Vec<AcceptConnectionsCoroutineSettings<sockaddr_in>>,
		transmission_control_protocol_over_internet_protocol_version_6_server_listeners: Vec<AcceptConnectionsCoroutineSettings<sockaddr_in6>>,
		streaming_unix_domain_socket_server_listener_server_listeners: Vec<AcceptConnectionsCoroutineSettings<UnixSocketAddress<PathBuf>>>,
	) -> Result<Self, ThreadLoopInitializationError>
	{
		Ok
		(
			Self
			(
				Self::new_accept_coroutine_manager(0, global_allocator, defaults, io_uring, queues, transmission_control_protocol_over_internet_protocol_version_4_server_listeners)?,
				Self::new_accept_coroutine_manager(1, global_allocator, defaults, io_uring, queues, transmission_control_protocol_over_internet_protocol_version_6_server_listeners)?,
				Self::new_accept_coroutine_manager(2, global_allocator, defaults, io_uring, queues, streaming_unix_domain_socket_server_listener_server_listeners)?,
			)
		)
	}
	
	pub fn dispatch_io_uring<NonCoroutineHandler: FnMut(u64, CompletionResponse) -> Result<(), NonCoroutineHandlerError>, NonCoroutineHandlerError: error::Error + Into<DispatchIoUringError<NonCoroutineHandlerError>>>(&mut self, completion_queue_entry: CompletionQueueEntry, mut non_coroutine_handler: &mut NonCoroutineHandler) -> Result<(), DispatchIoUringError<NonCoroutineHandlerError>>
	{
		let user_data = completion_queue_entry.user_data();
		
		let completion_response = completion_queue_entry.completion_response();
		if CoroutineInstanceHandle::is_not_for_a_coroutine(user_data)
		{
			return Ok(non_coroutine_handler(user_data, completion_response)?)
		}
		
		let coroutine_instance_handle = CoroutineInstanceHandle::wrap(user_data);
		
		choose_coroutine_manager!
		{
			coroutine_instance_handle.coroutine_manager_index(),
			self,
			(coroutine_instance_handle, completion_response),
			0 => 0 @ Self::dispatch_transmission_control_protocol_over_internet_protocol_server_listener,
			1 => 1 @ Self::dispatch_transmission_control_protocol_over_internet_protocol_server_listener,
			2 => 2 @ Self::dispatch_transmission_control_protocol_over_internet_protocol_server_listener,
		}
	}
	
	#[inline(always)]
	fn new_accept_coroutine_manager<SA: SocketAddress>(coroutine_manager_index: u8, global_allocator: &'static GTACSA, defaults: &DefaultPageSizeAndHugePageSizes, io_uring: &Rc<IoUring<'static>>, remote_peer_adddress_based_access_control: &Rc<RemotePeerAddressBasedAccessControl<RemotePeerAddressBasedAccessControlValue>>, queues: &Queues<(), DequeuedMessageProcessingError>, mut transmission_control_protocol_server_listener_settings: Vec<TransmissionControlProtocolServerListenerSettings<SA>>) -> Result<CoroutineManager<HeapSize, StackSize, GTACSA, AcceptCoroutine<SA>, AcceptCoroutineInformation>, ThreadLoopInitializationError>
	{
		let length = transmission_control_protocol_server_listener_settings.len();
		let ideal_maximum_number_of_coroutines = if length == 0
		{
			unsafe { NonZeroU64::new_unchecked(1) }
		}
		else
		{
			unsafe { NonZeroU64::new_unchecked(length as usize) }
		};
		
		use self::ThreadLoopInitializationError::*;
		
		let coroutine_manager = CoroutineManager::new(CoroutineManagerIndex(coroutine_manager_index), global_allocator, ideal_maximum_number_of_coroutines, defaults).map_err(AcceptConnectionsCoroutineManager)?;
		
		for settings in transmission_control_protocol_server_listener_settings
		{
			let start_arguments = (io_uring.clone(), queues.clone(), settings.new_socket()?, settings.remote_peer_adddress_based_access_control(), settings.service_protocol_identifier());
			
			use self::StartOutcome::*;
			use self::AcceptYields::*;
			match coroutine_manager.start_coroutine(AcceptCoroutineInformation, start_arguments)
			{
				Err(error) => return Err(CouldNotAllocateAcceptCoroutine(error)),
				
				Ok(WouldLikeToResume(AwaitingIoUring)) => (),
				
				Ok(WouldLikeToResume(SubmissionQueueFull)) => panic!("The submission queue should not be full when starting an accept coroutine"),
				
				Ok(Complete(())) => panic!("An accept loop should never complete"),
			}
		}
		
		Ok(coroutine_manager)
	}
	
	#[inline(always)]
	fn dispatch_transmission_control_protocol_over_internet_protocol_server_listener<NonCoroutineHandlerError: error::Error + Into<DispatchIoUringError<NonCoroutineHandlerError>>, SA: SocketAddress>(coroutine_manager: &mut CoroutineManager<HeapSize, StackSizeAccept, GTACSA, AcceptCoroutine<SA>, AcceptCoroutineInformation>, (coroutine_instance_handle, completion_response): (CoroutineInstanceHandle, CompletionResponse)) -> Result<(), DispatchIoUringError<NonCoroutineHandlerError>>
	{
		let coroutine_instance_pointer = unsafe { CoroutineInstancePointer::<HeapSize, StackSizeAccept, GTACSA, AcceptCoroutine<SA>, AcceptCoroutineInformation>::from_handle(coroutine_instance_handle) };
		
		use self::ResumeOutcome::*;
		use self::AcceptYields::*;
		match coroutine_manager.resume_coroutine(coroutine_instance_pointer, AcceptResumeArguments::Accepted(coroutine_instance_handler.user_bits(), completion_response))
		{
			WouldLikeToResume(AwaitingIoUring) => Ok(()),
			
			// TODO: Add this coroutine to a list that requires re-entry when the completion queue / submission queue has acquiesced a bit.
			WouldLikeToResume(SubmissionQueueFull) => xxxx,
			
			Complete(Ok(())) => panic!("An accept loop should never complete"),
			
			Complete(Err(error)) => Err(DispatchIoUringError::NewSocketServerListener(error)),
		}
	}
}
