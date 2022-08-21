// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a streaming socket instance between a local peer and a remote peer.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct StreamingSocketFileDescriptor<SD: SocketData>(SocketFileDescriptor<SD>);

impl<SD: SocketData> AsRawFd for StreamingSocketFileDescriptor<SD>
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0.as_raw_fd()
	}
}

impl<SD: SocketData> FromRawFd for StreamingSocketFileDescriptor<SD>
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(SocketFileDescriptor::from_raw_fd(fd))
	}
}

impl<SD: SocketData> IntoRawFd for StreamingSocketFileDescriptor<SD>
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0.into_raw_fd()
	}
}

impl<SD: SocketData> FileDescriptor for StreamingSocketFileDescriptor<SD>
{
}

impl<SD: SocketData> Deref for StreamingSocketFileDescriptor<SD>
{
	type Target = SocketFileDescriptor<SD>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<SD: SocketData> VectoredRead for StreamingSocketFileDescriptor<SD>
{
	vectored_read!();
}

impl<SD: SocketData> VectoredWrite for StreamingSocketFileDescriptor<SD>
{
	vectored_write!();
}

impl<SD: SocketData> PipeLikeFileDescriptor for StreamingSocketFileDescriptor<SD>
{

}

impl<SD: SocketData> SpliceRecipient for StreamingSocketFileDescriptor<SD>
{
}

impl<SD: SocketData> SpliceSender for StreamingSocketFileDescriptor<SD>
{
}

impl<SD: SocketData> SendFile for StreamingSocketFileDescriptor<SD>
{
	#[inline(always)]
	fn write_output_from_file<F: AsRef<File>>(&self, from_file: &F, maximum_number_of_bytes_to_transfer: usize) -> Result<usize, StructWriteError>
	{
		if unlikely!(maximum_number_of_bytes_to_transfer == 0)
		{
			return Ok(0)
		}

		let result = unsafe { sendfile(self.as_raw_fd(), from_file.as_ref().as_raw_fd(), null_mut(), maximum_number_of_bytes_to_transfer) };
		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else if likely!(result == -1)
		{
			use self::StructWriteError::*;

			Err
			(
				match SystemCallErrorNumber::from_errno_panic()
				{
					EAGAIN | ENOMEM => WouldBlock,
					EINTR => Interrupted,
					EIO => Cancelled,

					EBADF => panic!("The input file was not opened for reading or the output file was not opened for writing"),
					EFAULT => panic!("Bad address"),
					EINVAL => panic!("Descriptor is not valid or locked, or an mmap(2)-like operation is not available for in_fd"),
					
					unexpected_error @ _ => unexpected_error!(sendfile, "streaming socket file descriptor", unexpected_error),
				}
			)
		}
		else
		{
			unexpected_result!(sendfile, "streaming socket file descriptor", result)
		}
	}

	#[inline(always)]
	fn write_output_from_file_with_offset<F: AsRef<File>>(&self, from_file: &F, mut offset: i64, maximum_number_of_bytes_to_transfer: usize) -> Result<(usize, i64), StructWriteError>
	{
		if unlikely!(maximum_number_of_bytes_to_transfer == 0)
		{
			return Ok((0, offset))
		}

		let result = unsafe { sendfile(self.as_raw_fd(), from_file.as_ref().as_raw_fd(), &mut offset, maximum_number_of_bytes_to_transfer) };
		if likely!(result >= 0)
		{
			Ok((result as usize, offset))
		}
		else if likely!(result == -1)
		{
			use self::StructWriteError::*;

			Err
			(
				match SystemCallErrorNumber::from_errno_panic()
				{
					EAGAIN | ENOMEM => WouldBlock,
					EINTR => Interrupted,
					EIO => Cancelled,

					EBADF => panic!("The input file was not opened for reading or the output file was not opened for writing"),
					EFAULT => panic!("Bad address"),
					EINVAL => panic!("Descriptor is not valid or locked, or an mmap(2)-like operation is not available for in_fd"),
					
					unexpected_error @ _ => unexpected_error!(sendfile, "streaming socket file descriptor", unexpected_error),
				}
			)
		}
		else
		{
			unexpected_result!(sendfile, "streaming socket file descriptor", result)
		}
	}
}

impl<SD: SocketData> Read for StreamingSocketFileDescriptor<SD>
{
	/// This particular implementation wraps `self.receive_from()`.
	///
	/// See its documentation for the restricted range of errors that can be returned.
	#[inline(always)]
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
	{
		self.receive_from(buf)
	}

	#[inline(always)]
	fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>
	{
		VectoredRead::read_vectored(self, unsafe { transmute(bufs) })
	}
}

impl<SD: SocketData> Write for StreamingSocketFileDescriptor<SD>
{
	/// This particular implementation wraps `self.send_to()`.
	///
	/// See its documentation for the restricted range of errors that can be returned.
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize>
	{
		self.send_to(buf)
	}

	#[inline(always)]
	fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize>
	{
		VectoredWrite::write_vectored(self, unsafe { transmute(bufs) })
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()>
	{
		Ok(())
	}
}

impl<SD: SocketData> StreamingSocketFileDescriptor<SD>
{
	/// Sends a TCP `FIN` for TCP sockets.
	///
	/// Should be done prior to drop(), ideally.
	#[inline(always)]
	pub fn shutdown(&self)
	{
		let result = unsafe { shutdown(self.as_raw_fd(), SHUT_RDWR) };
		if likely!(result == 0)
		{
			return
		}
		else if likely!(result != -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				EBADF => panic!("The argument `sockfd` is an invalid descriptor"),
				EINVAL => panic!("An invalid value was specified in `how`"),
				ENOTCONN => panic!("The socket is associated with a connection-oriented protocol and has not been connected"),
				ENOTSOCK => panic!("The argument `sockfd` does not refer to a socket"),
				unexpected_error @ _ => unexpected_error!(shutdown, "streaming socket file descriptor", unexpected_error),
			}
		}
		else
		{
			unexpected_result!(shutdown, "streaming socket file descriptor", result)
		}
	}

	/// This is wrapped by `io::Read::read()` but is exposed as it does not require a mutable reference.
	///
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `UnexpectedEof`.
	/// * `WouldBlock`.
	/// * `Interrupted`.
	/// * `Other` (which is for when the kernel reports `ENOMEM`, ie it is out of memory).
	/// * `ConnectionReset` (seems to be possible in some circumstances for Unix domain sockets).
	/// * `ConnectionRefused` (only can happen for TCP client sockets; can not happen for sockets `accept()`ed by a server listener).
	#[inline(always)]
	pub fn receive_from(&self, buf: &mut [u8]) -> io::Result<usize>
	{
		let length = buf.len();
		if unlikely!(length == 0)
		{
			return Ok(0)
		}

		let result = unsafe { recvfrom(self.as_raw_fd(), buf.as_mut_ptr() as *mut c_void, length, ReceiveFlags::empty().bits, null(), null_mut()) };

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
							unexpected_error @ _ => unexpected_error!(recvfrom, "streaming socket file descriptor", unexpected_error),
						}
					}
					else
					{
						unexpected_result!(recvfrom, "streaming socket file descriptor", result)
					}
				)
			)
		}
	}

	/// This is wrapped by `io::Read::read()` but is exposed as it does not require a mutable reference.
	///
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
	pub fn send_to(&self, buf: &[u8]) -> io::Result<usize>
	{
		let length = buf.len();

		if unlikely!(length == 0)
		{
			return Ok(0)
		}

		let result = unsafe { send(self.as_raw_fd(), buf.as_ptr() as *const c_void, buf.len(), SendFlags::NoSigPipeSignal.bits) };

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
							unexpected_error @ _ => unexpected_error!(send, "streaming socket file descriptor", unexpected_error),
						}
					}
					else
					{
						unexpected_result!(send, "streaming socket file descriptor", result)
					}
				)
			)
		}
	}
}

impl<SD: SocketData> SocketConnect for StreamingSocketFileDescriptor<SD>
{

}

impl<SD: SocketData> NonServerSocket for StreamingSocketFileDescriptor<SD>
{
}

impl StreamingSocketFileDescriptor<sockaddr_un>
{
	/// Tries to obtain remote peer credentials.
	///
	/// The returned credentials are those that were in effect at the time of the call to `connect()` or `socketpair()`.
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
