// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a datagram socket instance between a local peer and a remote peer that listens for incoming messages.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DatagramClientListenerSocketFileDescriptor<SD: SocketData>(SocketFileDescriptor<SD>);

impl<SD: SocketData> AsRawFd for DatagramClientListenerSocketFileDescriptor<SD>
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0.as_raw_fd()
	}
}

impl<SD: SocketData> IntoRawFd for DatagramClientListenerSocketFileDescriptor<SD>
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0.into_raw_fd()
	}
}

impl<SD: SocketData> FromRawFd for DatagramClientListenerSocketFileDescriptor<SD>
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(SocketFileDescriptor::from_raw_fd(fd))
	}
}

impl<SD: SocketData> FileDescriptor for DatagramClientListenerSocketFileDescriptor<SD>
{
}

impl<SD: SocketData> NonServerSocket for DatagramClientListenerSocketFileDescriptor<SD>
{
}

impl<SD: SocketData> Deref for DatagramClientListenerSocketFileDescriptor<SD>
{
	type Target = SocketFileDescriptor<SD>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<SD: SocketData> DatagramClientListenerSocketFileDescriptor<SD>
{
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `UnexpectedEof`.
	/// * `WouldBlock`.
	/// * `Interrupted`.
	/// * `Other` (which is for when the kernel reports `ENOMEM`, ie it is out of memory).
	/// * `ConnectionReset` (seems to be possible in some circumstances for Unix domain sockets).
	/// * `ConnectionRefused` (only can happen for TCP client sockets; can not happen for sockets `accept()`ed by a server listener).
	#[inline(always)]
	pub fn receive(&self, buf: &mut [u8], receive_flags: ReceiveFlags) -> io::Result<usize>
	{
		let length = buf.len();
		if unlikely!(length == 0)
		{
			return Ok(0)
		}

		let result = unsafe { recv(self.as_raw_fd(), buf.as_mut_ptr() as *mut c_void, length, receive_flags.bits) };

		if likely!(result > 0)
		{
			Ok(result as usize)
		}
		else
		{
			use crate::ErrorKind::*;

			Err
			(
				io::Error::from
				(
					if likely!(result == 0)
					{
						UnexpectedEof
					}
					else if likely!(result == -1)
					{
						match SystemCallErrorNumber::from_errno_panic()
						{
							EAGAIN => WouldBlock,
							EINTR => Interrupted,
							ENOMEM => Other,
							ECONNRESET => ConnectionReset,
							ECONNREFUSED => ConnectionRefused,
							EBADF => panic!("The argument `sockfd` is an invalid descriptor"),
							EFAULT => panic!("The receive buffer pointer(s) point outside the process's address space"),
							EINVAL => panic!("Invalid argument passed"),
							ENOTCONN => panic!("The socket is associated with a connection-oriented protocol and has not been connected"),
							ENOTSOCK => panic!("The argument `sockfd` does not refer to a socket"),
							EOPNOTSUPP => panic!("Some flags in the `flags` argument are inappropriate for the socket type"),
							unexpected_error @ _ => unexpected_error!(recv, "datagram client listener socket file descriptor", unexpected_error),
						}
					}
					else
					{
						unexpected_result!(recv, "datagram client listener socket file descriptor", result)
					}
				)
			)
		}
	}
	
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `WriteZero`.
	/// * `WouldBlock`.
	/// * `Interrupted`.
	/// * `Other` (which is for when the kernel reports `ENOMEM` or `ENOBUFS`, ie it is out of memory).
	/// * `BrokenPipe`.
	/// * `PermissionDenied` (only for Unix domain sockets).
	/// * `ConnectionReset`.
	#[inline(always)]
	pub fn send(&self, buf: &[u8], send_flags: SendFlags) -> io::Result<usize>
	{
		let length = buf.len();

		if unlikely!(length == 0)
		{
			return Ok(0)
		}

		let result = unsafe { send(self.as_raw_fd(), buf.as_ptr() as *const c_void, buf.len(), send_flags.bits) };

		if likely!(result > 0)
		{
			Ok(result as usize)
		}
		else
		{
			use crate::ErrorKind::*;

			Err
			(
				io::Error::from
				(
					if likely!(result == 0)
					{
						WriteZero
					}
					else if likely!(result == -1)
					{
						match SystemCallErrorNumber::from_errno_panic()
						{
							EAGAIN => WouldBlock,
							EINTR => Interrupted,
							ENOMEM | ENOBUFS => Other,
							EPIPE => BrokenPipe,
							EACCES => PermissionDenied,
							ECONNRESET => ConnectionReset,
							EBADF => panic!("The argument `sockfd` is an invalid descriptor"),
							EFAULT => panic!("The receive buffer pointer(s) point outside the process's address space"),
							EINVAL => panic!("Invalid argument passed"),
							ENOTCONN => panic!("The socket is associated with a connection-oriented protocol and has not been connected"),
							ENOTSOCK => panic!("The argument `sockfd` does not refer to a socket"),
							EOPNOTSUPP => panic!("Some flags in the `flags` argument are inappropriate for the socket type"),
							EMSGSIZE => panic!("The socket type requires that message be sent atomically, and the size of the message to be sent made this impossible"),
							EISCONN => panic!("The connection-mode socket was connected already but a recipient was specified"),
							EDESTADDRREQ => panic!("The socket is not connection-mode, and no peer address is set"),
							unexpected_error @ _ => unexpected_error!(send, "datagram client listener socket file descriptor", unexpected_error),
						}
					}
					else
					{
						unexpected_result!(send, "datagram client listener socket file descriptor", result)
					}
				)
			)
		}
	}
}

impl<SD: SocketData> DatagramClientListenerSocketFileDescriptor<SD>
{
	/// Receive messages up to the maximum capacity of `received_messages`.
	///
	/// Returns the number of messages received; access received messages by calling `received_messages.received_message_unchecked(index)` where `index` is less than the returned number of messages.
	///
	/// Create the value of `received_messages` using `ReceivedMessages::new()`.
	#[inline(always)]
	pub fn receive_messages<'a>(&self, received_messages: &mut ReceivedMessages<'a, SD>, receive_flags: ReceiveFlags) -> Result<usize, StructReadError>
	{
		let multi_message_headers = &mut received_messages.multi_message_headers;

		let result = unsafe { recvmmsg(self.as_raw_fd(), multi_message_headers.as_mut_ptr(), multi_message_headers.len() as u32, receive_flags.bits, null_mut()) };
		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else if likely!(result == -1)
		{
			use self::StructReadError::*;

			Err
			(
				match SystemCallErrorNumber::from_errno_panic()
				{
					EAGAIN | ENOMEM => WouldBlock,
					EINTR => Interrupted,
					EBADF => panic!("The argument `sockfd` is an invalid descriptor"),
					ECONNREFUSED => panic!("A remote host refused to allow the network connection (typically because it is not running the requested service)"),
					EFAULT => panic!("The receive buffer pointer(s) point outside the process's address space"),
					EINVAL => panic!("Invalid argument passed"),
					ENOTCONN => panic!("The socket is associated with a connection-oriented protocol and has not been connected"),
					ENOTSOCK => panic!("The argument `sockfd` does not refer to a socket"),
					unexpected_error @ _ => unexpected_error!(recvmmsg, "datagram client listener socket file descriptor", unexpected_error),
				}
			)
		}
		else
		{
			unexpected_result!(recvmmsg, "datagram client listener socket file descriptor", result)
		}
	}
}

impl DatagramClientListenerSocketFileDescriptor<sockaddr_un>
{
	/// Tries to obtain remote peer credentials.
	///
	/// Only available if this socket was created using `socketpair()` (ie it is anonymous).
	///
	/// The returned credentials are those that were in effect at the time of the call to `socketpair()`.
	#[inline(always)]
	pub fn remote_peer_credentials(&self) -> Credentials
	{
		self.0.remote_peer_credentials()
	}

	/// Receive file descriptors.
	pub fn receive_file_descriptors(&self, maximum_file_descriptors_to_receive: usize) -> Result<Vec<RawFd>, ReceiveFileDescriptorsError>
	{
		self.0.receive_file_descriptors(maximum_file_descriptors_to_receive)
	}

	/// Tries to send file descriptors to a remote peer over an Unix Domain Socket.
	///
	/// `file_descriptors`: File Descriptors to send.
	#[inline(always)]
	pub fn send_file_descriptors(&self, file_descriptors: &[RawFd]) -> io::Result<()>
	{
		self.0.send_file_descriptors(file_descriptors)
	}

	/// Tries to send credentials to a remote peer over an Unix Domain Socket.
	///
	/// Useful for complex scenarios where a privileged (eg root) process wants to use different credentials to those it would default to.
	///
	/// `process_identifier`: Process identifier (also known as `pid`). Unless the process has capability `CAP_SYS_ADMIN`, this must be its own `process_identifier`.
	/// `user_identifier`: User identifier (also known as `uid`). Unless the process has capability `CAP_SETUID`, this must be its own `user_identifier`, effective `user_identifier` or saved-set `user_identifier`.
	/// `group_identifier`: Group identifier (also known as `gid`). Unless the process has capability `CAP_SETGID`, this must be its own `group_identifier`, effective `group_identifier` or saved-set `group_identifier`.
	#[inline(always)]
	pub fn send_credentials(&self, credentials: Credentials) -> io::Result<()>
	{
		self.0.send_credentials(credentials)
	}
}
