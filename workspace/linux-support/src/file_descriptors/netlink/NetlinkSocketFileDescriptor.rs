// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A netlink socket file descriptor.
///
/// Not much use on its own!
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NetlinkSocketFileDescriptor<Protocol: NetlinkProtocol>(RawFd, SequenceNumber, PhantomData<Protocol>);

impl<Protocol: NetlinkProtocol> Drop for NetlinkSocketFileDescriptor<Protocol>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		// Please see <http://austingroupbugs.net/view.php?id=529> and <http://austingroupbugs.net/view.php?id=529> for why ignoring the `EINTR` error on close is actually sane.
		//
		// Frankly, the defects here are those of POSIX: (a) signals, and (b) using a file descriptor so small that it isn't thread safe.
		//
		// To be far, both signals and file descriptors predate threads by a long way.
		unsafe { close(self.0) };
	}
}

impl<Protocol: NetlinkProtocol> AsRawFd for NetlinkSocketFileDescriptor<Protocol>
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl<Protocol: NetlinkProtocol> IntoRawFd for NetlinkSocketFileDescriptor<Protocol>
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl<Protocol: NetlinkProtocol> FromRawFd for NetlinkSocketFileDescriptor<Protocol>
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		let mut protocol: c_int = unsafe_uninitialized();
		let mut size = size_of::<c_int>() as u32;
		let result = getsockopt(fd, SOL_SOCKET, SO_PROTOCOL, &mut protocol as *mut i32 as *mut c_void, &mut size);
		if likely!(result == 0)
		{
			assert_eq!(protocol, Protocol::Protocol, "Protocol mismatch");
			Self(fd, SequenceNumber::Zero, PhantomData)
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => panic!("The argument sockfd is not a valid file descriptor"),
				EFAULT => panic!(" The address pointed to by optval is not in a valid part of the process address space. For getsockopt(), this error may also be returned if optlen is not in a valid part of the process address space."),
				EINVAL => panic!("optlen invalid in setsockopt().  In some cases this error can also occur for an invalid value in optval (e.g., for the IP_ADD_MEMBERSHIP option described in ip(7))"),
				ENOPROTOOPT => panic!("The option is unknown at the level indicated"),
				ENOTSOCK => panic!("The file descriptor sockfd does not refer to a socket"),
				
				unexpected @ _ => unreachable_code(format_args!("Unexpected error {} from getsockopt()", unexpected)),
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from getsockopt()", result));
		}
	}
}

impl<Protocol: NetlinkProtocol> FileDescriptor for NetlinkSocketFileDescriptor<Protocol>
{
}

impl<Protocol: NetlinkProtocol> NetlinkSocketFileDescriptor<Protocol>
{
	/// Open.
	///
	/// Binds the port so no multicast messages are received.
	#[inline(always)]
	pub fn open() -> Result<Self, SocketCreationOrBindError>
	{
		let this = Self(new_socket(AF_NETLINK, SOCK_RAW, Protocol::Protocol, false)?, SequenceNumber::One, PhantomData);
		
		let socket_address = sockaddr_nl::default();
		bind_socket(&this, &socket_address)?;
		
		Ok(this)
	}
	
	/// Returns message identification.
	#[inline(always)]
	fn send_request<Body: NetlinkRequestMessageBody>(&mut self, request: &mut NetlinkRequestMessage<Body>) -> io::Result<SequenceNumber>
	{
		const NoSendFlags: i32 = 0;
		
		let current_sequence_number = self.1.get_then_increment();
		request.header.nlmsg_seq = current_sequence_number;
		
		let length = request.length();
		let result = unsafe { send(self.as_raw_fd(), request as *const NetlinkRequestMessage<Body> as *const c_void, length, NoSendFlags) };
		if likely!(result >= 0)
		{
			if likely!((result as usize) == length)
			{
				Ok(current_sequence_number)
			}
			else if likely!((result as usize) < length)
			{
				unreachable_code(format_args!("Short ({}) send()", result))
			}
			else
			{
				unreachable_code(format_args!("Sent too many bytes ({}) from send()", result))
			}
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from send()", result))
		}
	}
	
	fn receive_replies(&self, reply_receiver: &mut impl ReplyReceiver<Protocol>)
	{
		/// Default used by `libnl`.
		const InitialBufferSize: usize = (4 * PageSize::default().size_in_bytes().get()) as usize;
		let mut buffer = Vec::with_capacity(InitialBufferSize);
		
		let mut multipart_message_identifier: Option<MultipartMessagePartIdentification> = None;
		let mut dump_was_interrupted = DumpCompleted;
		loop
		{
			let actual_length = loop
			{
				let result = unsafe { recvfrom(self.as_raw_fd(), null_mut(), 0, MSG_PEEK | MSG_TRUNC, null(), null_mut()) };
				
				if likely!(result > 0)
				{
					let available_length = result as usize;
					let current_buffer_length = buffer.len();
					if available_length > current_buffer_length
					{
						buffer.reserve(current_buffer_length);
					}
					
					break available_length
				}
				else if likely!(result == 0)
				{
					if multipart_message_identifier.is_some()
					{
						reply_receiver.unexpected_end_of_set_of_multipart_messages()
					}
					return
				}
				else if likely!(result == -1)
				{
					match errno().0
					{
						EINTR => continue,
						
						_ =>
						{
							let error = io::Error::last_os_error();
							if multipart_message_identifier.is_some()
							{
								reply_receiver.could_not_continue_multipart_messages(error)
							}
							else
							{
								reply_receiver.could_not_start_messages(error);
							}
							
							return
						}
					}
				}
				else
				{
					unreachable_code(format_args!("Unexpected result {} from recvfrom()", result))
				};
			};
			
			let mut remaining_length = loop
			{
				let result = unsafe { recvfrom(self.as_raw_fd(), buffer.as_mut_ptr() as *mut c_void, buffer.len(), MSG_DONTWAIT, null(), null_mut()) };
				
				if likely!(result > 0)
				{
					break result as usize
				}
				else if likely!(result == 0)
				{
					if multipart_message_identifier.is_some()
					{
						reply_receiver.unexpected_end_of_set_of_multipart_messages()
					}
					return
				}
				else if likely!(result == -1)
				{
					match errno().0
					{
						EINTR => continue,
						
						_ =>
						{
							let error = io::Error::last_os_error();
							if multipart_message_identifier.is_some()
							{
								reply_receiver.could_not_continue_multipart_messages(error)
							}
							else
							{
								reply_receiver.could_not_start_messages(error);
							}
							
							return
						}
					}
				}
				else
				{
					unreachable_code(format_args!("Unexpected result {} from recvfrom()", result))
				};
			};
			
			debug_assert_eq!(actual_length, remaining_length);
			
			let mut message_pointer = buffer.as_ptr() as *const nlmsghdr;
			while nlmsghdr::NLMSG_OK(remaining_length, message_pointer)
			{
				let reply_message = unsafe { & * message_pointer };
				
				let multipart_message_part_identification = MultipartMessagePartIdentification::new(reply_message);
				if multipart_message_identifier.is_none()
				{
					reply_receiver.start_of_set_of_messages(&multipart_message_part_identification);
					multipart_message_identifier = Some(multipart_message_part_identification)
				}
				else
				{
					let expected_sequence_of_multi_part_messages = multipart_message_identifier.as_ref().unwrap();
					if unlikely!(expected_sequence_of_multi_part_messages != &multipart_message_part_identification)
					{
						panic!("Multipart netlink message sequence of message parts terminated abrutly")
					}
				}
				
				let flags = reply_message.nlmsg_flags.common();
				
				let is_multipart = flags.is_multipart();
				if unlikely!(flags.was_dump_interrupted())
				{
					dump_was_interrupted = DumpWasInterrupted;
				}
				assert!(!flags.acknowledgment_required(), "Acknowledgments to received messages are not supported");
				
				match unsafe { reply_message.nlmsg_type.control }
				{
					ControlNetlinkMessageType::Done =>
					{
						debug_assert_eq!(is_multipart, false, "There might be kernel bugs that mean this is not true");
						
						reply_receiver.end_of_set_of_messages(Ok(dump_was_interrupted));
						multipart_message_identifier = None;
						dump_was_interrupted = DumpCompleted;
					}
					
					// This message type is also used for acknowledgments.
					ControlNetlinkMessageType::Error =>
					{
						let result = reply_message.error().error_or_acknowledgment_io_result().map(|_: ()| dump_was_interrupted);
						reply_receiver.end_of_set_of_messages(result);
					}
					
					ControlNetlinkMessageType::OverRun =>
					{
						let result = Err(reply_message.error().over_run_io_result());
						reply_receiver.end_of_set_of_messages(result);
					}
					
					ControlNetlinkMessageType::NoOp => if !is_multipart
					{
						reply_receiver.end_of_set_of_messages(Ok(dump_was_interrupted));
						multipart_message_identifier = None;
						dump_was_interrupted = DumpCompleted;
					}
					
					_ =>
					{
						reply_receiver.message(Protocol::message_type(reply_message.nlmsg_type), reply_message.data());
						
						if !is_multipart
						{
							reply_receiver.end_of_set_of_messages(Ok(dump_was_interrupted));
							multipart_message_identifier = None;
							dump_was_interrupted = DumpCompleted;
						}
					},
				}
				
				message_pointer = reply_message.NLMSG_NEXT(&mut remaining_length)
			}
		}
	}
}

impl NetlinkSocketFileDescriptor<RouteNetlinkProtocol>
{
}
