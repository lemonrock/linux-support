// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub struct SimpleNetworkTimeProtocolClient<'yielder, SD: SocketData, CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>
{
	simple_io_uring_yielder: SimpleIoUringYielder<'yielder, UnusedComplete>,
	bound_and_connected_socket_file_descriptor: DatagramClientListenerSocketFileDescriptor<SD>,
	state: State<SD>,
	io_priority: CompressedIoPriority,
	
	dog_stats_d:  SimpleNetworkTimeProtocolClientDogStatsD<CoroutineHeapSize, GTACSA>,
}

/*
	DNS:
		int getaddrinfo (const char *__restrict, const char *__restrict, const struct addrinfo *__restrict, struct addrinfo **__restrict);
		void freeaddrinfo (struct addrinfo *);




 */



impl<'yielder, SD: SocketData, CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> SimpleNetworkTimeProtocolClient<'yielder, SD, CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	pub(crate) fn new(coroutine_instance_handle: CoroutineInstanceHandle, yielder: Yielder<'yielder, SimpleIoUringResumeArguments, SimpleIoUringYields, UnusedComplete>, start_arguments: SimpleNetworkTimeProtocolClientStartArguments<SA, CoroutineHeapSize, GTACSA>) -> Self
	{
		let (io_uring, bound_and_connected_socket_file_descriptor, destination_socket_data, (dog_stats_d_publisher)) = start_arguments;
		
		let source_socket_data = bound_and_connected_socket_file_descriptor.local_address().unwrap().0;
		
		Self
		{
			simple_io_uring_yielder: SimpleIoUringYielder::new(yielder, io_uring, coroutine_instance_handle),
			bound_and_connected_socket_file_descriptor,
			state: State::new(),
			io_priority: CompressedIoPriority::from(IoPriority::Idle),
			
			dog_stats_d: SimpleNetworkTimeProtocolClientDogStatsD::new(dog_stats_d_publisher),
		}
	}
	
	#[inline(always)]
	pub(crate) fn yield_submit_send(&mut self) -> bool
	{
		let bound_and_connected_socket_file_descriptor = &self.bound_and_connected_socket_file_descriptor;
		
		let (buffer, sent_transmit_timestamp) = NetworkTimeProtocolMessage::SimpleNetworkTimeProtocolClientRequest.serialize_request_from_client_to_server();
		
		// TODO: Use registered buffers.
		xxxxx;
		
		self.state.update_on_send(sent_transmit_timestamp);
		
		self.simple_io_uring_yielder.yield_submit_io_uring(|submission_queue_entry, user_data| submission_queue_entry.prepare_send(user_data, Self::NoSubmissionOptions, None, self.io_priority, FileDescriptorOrigin::Absolute(bound_and_connected_socket_file_descriptor), &buffer[..], SendFlags::empty()))
	}
	
	#[inline(always)]
	pub(crate) fn yield_awaiting_send(&mut self) -> Result<CompletionResponse, ()>
	{
		self.simple_io_uring_yielder.yield_awaiting_io_uring()
	}
	
	pub(crate) fn prepare_receive_buffer() -> [u8; NetworkTimeProtocolMessage::PacketSize]
	{
		unsafe_uninitialized()
	}
	
	#[inline(always)]
	pub(crate) fn yield_submit_receive(&mut self, receive_buffer: &mut [u8; NetworkTimeProtocolMessage::PacketSize]) -> bool
	{
		let bound_and_connected_socket_file_descriptor = &self.bound_and_connected_socket_file_descriptor;
		
		// TODO: DNS look up of time servers
		// TODO: Re-create the connection each time? We cannot call bind, but we can call connect, via io-uring, thus we can do a DNS look up each time.
		// This is sensible.
		/*
			time.cloudflare.com
			pool.ntp.org
				LI has meaning
				Returns up to 4 servers; changes with each look up.
			metadata.google.internal
			time.google.com
			time<N>.google.com where <N> is 1 to 4.
				LI does not have meaning (smeared time) (value is 0)
			Amazon Time Sync Service at 169.254.169.123
				LI does not have meaning (smeared time) (value is 0)
				https://aws.amazon.com/about-aws/whats-new/2017/11/introducing-the-amazon-time-sync-service/
				https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/set-time.html
				minpoll 4 maxpoll 4
		
			Do not be over-eager to apply time changes
				Windows won't accept an NTP reply that moves the clock more than 15 hours and will typically only synchronise once every nine hours.
				
			See https://github.com/ioerror/tlsdate/blob/master/src/tlsdate-setter.c for code to set both time of day and RTC clock.
			
			Alternatvies
			
				tlsdate (broken with TLS1.3)
				HTTP Date header
				Roughtime (supported by CLoudflare and google and there is a Rust client)
		 */
		
		// TODO: Registered buffers?
		// TODO: Timeout.
		// TODO: Message authentication.
		
		self.simple_io_uring_yielder.yield_submit_io_uring(|submission_queue_entry, user_data| submission_queue_entry.prepare_receive(user_data, Self::NoSubmissionOptions, None, self.io_priority, FileDescriptorOrigin::Absolute(bound_and_connected_socket_file_descriptor), &mut receive_buffer[..], ReceiveFlags::Truncated))
	}
	
	#[inline(always)]
	pub(crate) fn yield_awaiting_receive(&mut self) -> Result<CompletionResponse, ()>
	{
		self.simple_io_uring_yielder.yield_awaiting_io_uring()
	}
	
	#[inline(always)]
	pub(crate) fn process_receive(&mut self, completion_response: CompletionResponse, receive_buffer: &[u8; NetworkTimeProtocolMessage::PacketSize]) -> Result<Signed3232FixedPoint, NetworkTimeProtocolMessageServerReplyParseError>
	{
		use self::NetworkTimeProtocolMessageServerReplyParseError::*;
		
		let length = completion_response.receive(receive_buffer).map_err(IoErrorOnReceive)?.expect("Should not have been cancelled") as usize;
		
		if unlikely!(length != NetworkTimeProtocolMessage::PacketSize)
		{
			return Err(InvalidLength(length))
		}
		
		let (leap_indicator, server_stratum, round_trip_delay, system_clock_offset) = self.state.simple_validate_and_parse_server_reply(&receive_buffer)?;
		
		Ok(system_clock_offset)
	}
}
