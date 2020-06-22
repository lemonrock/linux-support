// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A completion response code.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompletionResponse(i32);

impl CompletionResponse
{
	/// A successful result returning `0` means end-of-input, unless `prepare_splice()` was called with `maximum_number_of_bytes_to_transfer` was `0`.
	///
	/// Can not return `Ok(Some(value))` with `value` greater than `i32::MAX as u32`.
	/// Returns `Ok(None)` if cancelled.
	#[inline(always)]
	pub fn splice(self) -> Result<Option<u32>, StructWriteError>
	{
		let result = self.0;
		
		if likely!(result >= 0 )
		{
			Ok(Some(result as u32))
		}
		else if likely!(self.is_error())
		{
			use self::StructWriteError::*;
			
			match -self.0
			{
				ECANCELED => Ok(None),
				
				EAGAIN | ENOMEM => Err(WouldBlock),
				EINTR => Err(Interrupted),
				
				EBADF => panic!("One or both file descriptors are not valid, or do not have proper read-write mode"),
				EINVAL => panic!("The target filesystem doesn't support splicing; or the target file is opened in append mode; or neither of the file descriptors refers to a pipe; or an offset was given for nonseekable device (eg, a pipe); or `fd_in` and `fd_out` refer to the same pipe"),
				ESPIPE => panic!("Either `off_in` or `off_out` was not `NULL`, but the corresponding file descriptor refers to a pipe"),
				
				unexpected @ _ => unreachable!("Unexpected error code from splice completion of {}", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result from splice completion of {}", result)
		}
	}
	
	/// Can not return `Ok(Some(value))` with `value` greater than `i32::MAX as u32`.
	///
	/// Returns `Err(Cancelled)` for `EIO`.
	/// Returns `Ok(None)` if cancelled.
	/// Returns `Err(WouldBlock)` if in progress with the `TCP_FASTOPEN_CONNECT` socket option which expects the client to write-first (if connect() is called but write() is not called yet OR if connection is established but no message is received yet).
	#[inline(always)]
	pub fn read_vectored(self) -> Result<Option<u32>, StructReadError>
	{
		let result = self.0;
		
		if likely!(result >= 0 )
		{
			Ok(Some(result as u32))
		}
		else if likely!(self.is_error())
		{
			use self::StructReadError::*;
			
			match -self.0
			{
				ECANCELED => Ok(None),
				EAGAIN | ENOMEM => Err(WouldBlock),
				EINTR => Err(Interrupted),
				EIO => Err(Cancelled),
				
				EFAULT => panic!("buf is outside your accessible address space"),
				EISDIR => panic!("fd refers to a directory"),
				EBADF => panic!("fd is not a valid file descriptor or is not open for reading"),
				ENXIO => panic!("whence is SEEK_DATA or SEEK_HOLE, and offset is beyond the end of the file, or whence is SEEK_DATA and offset is within a hole at the end of the file."),
				EOVERFLOW => panic!("The resulting file offset cannot be represented in an off_t."),
				ESPIPE => panic!("fd is associated with a pipe, socket, or FIFO."),
				EINVAL => panic!("fd is attached to an object which is unsuitable for reading; or the file was opened with the O_DIRECT flag, and either the address specified in buf, the value specified in count, or the file offset is not suitably aligned. Or, fd was created via a call to timerfd_create(2) and the wrong size buffer was given to read(); see timerfd_create(2) for further information. Or, whence is not valid. Or: the resulting file offset would be negative, or beyond the end of a seekable device. Or the sum of the iov_len values overflows an ssize_t value. Or, the vector count, iovcnt, is less than zero or greater than the permitted maximum."),
				EOPNOTSUPP=> panic!("An unknown flag is specified in flags."),
				
				unexpected @ _ => unreachable!("Unexpected error code from read_vectored completion of {}", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result from read_vectored completion of {}", result)
		}
	}
	
	/// Can not return `Ok(Some(value))` with `value` greater than `i32::MAX as u32`.
	///
	/// Returns `Err(Cancelled)` for `EIO`.
	/// Returns `Ok(None)` if cancelled.
	/// Returns `Err(WouldBlock)` if in progress with the `TCP_FASTOPEN_CONNECT` socket option which expects the client to write-first (if connect() is called but write() is not called yet OR if connection is established but no message is received yet).
	#[inline(always)]
	pub fn read_fixed(self) -> Result<Option<u32>, StructReadError>
	{
		let result = self.0;
		
		if likely!(result >= 0 )
		{
			Ok(Some(result as u32))
		}
		else if likely!(self.is_error())
		{
			use self::StructReadError::*;
			
			match -self.0
			{
				ECANCELED => Ok(None),
				EAGAIN | ENOMEM => Err(WouldBlock),
				EINTR => Err(Interrupted),
				EIO => Err(Cancelled),
				
				EFAULT => panic!("buf is outside your accessible address space"),
				EISDIR => panic!("fd refers to a directory"),
				EBADF => panic!("fd is not a valid file descriptor or is not open for reading"),
				ENXIO => panic!("whence is SEEK_DATA or SEEK_HOLE, and offset is beyond the end of the file, or whence is SEEK_DATA and offset is within a hole at the end of the file."),
				EOVERFLOW => panic!("The resulting file offset cannot be represented in an off_t."),
				ESPIPE => panic!("fd is associated with a pipe, socket, or FIFO."),
				EINVAL => panic!("fd is attached to an object which is unsuitable for reading; or the file was opened with the O_DIRECT flag, and either the address specified in buf, the value specified in count, or the file offset is not suitably aligned. Or, fd was created via a call to timerfd_create(2) and the wrong size buffer was given to read(); see timerfd_create(2) for further information. Or, whence is not valid. Or: the resulting file offset would be negative, or beyond the end of a seekable device. Or the sum of the iov_len values overflows an ssize_t value. Or, the vector count, iovcnt, is less than zero or greater than the permitted maximum."),
				EOPNOTSUPP=> panic!("An unknown flag is specified in flags."),
				
				unexpected @ _ => unreachable!("Unexpected error code from read_fixed completion of {}", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result from read_fixed completion of {}", result)
		}
	}
	
	/// Can not return `Ok(Some(value))` with `value` greater than `i32::MAX as u32`.
	///
	/// Equivalent to either `read()` (if offset zero) or `pread()` (if specified).
	///
	/// Returns `Err(Cancelled)` for `EIO`.
	/// Returns `Ok(None)` if cancelled.
	/// Returns `Err(WouldBlock)` if in progress with the `TCP_FASTOPEN_CONNECT` socket option which expects the client to write-first (if connect() is called but write() is not called yet OR if connection is established but no message is received yet).
	/// Returns `Err(WouldBlock)` if `EAGAIN` or `ENOMEM`.
	#[inline(always)]
	pub fn read(self) -> Result<Option<u32>, StructReadError>
	{
		let result = self.0;
		
		if likely!(result >= 0 )
		{
			Ok(Some(result as u32))
		}
		else if likely!(self.is_error())
		{
			use self::StructReadError::*;
			
			match -self.0
			{
				ECANCELED => Ok(None),
				EAGAIN | ENOMEM => Err(WouldBlock),
				EINTR => Err(Interrupted),
				EIO => Err(Cancelled),
				
				ENXIO => panic!("whence is SEEK_DATA or SEEK_HOLE, and offset is beyond the end of the file, or whence is SEEK_DATA and offset is within a hole at the end of the file."),
				EOVERFLOW => panic!("The resulting file offset cannot be represented in an off_t."),
				ESPIPE => panic!("fd is associated with a pipe, socket, or FIFO."),
				EBADF => panic!("fd is not a valid file descriptor or is not open for reading"),
				EINVAL => panic!("fd is attached to an object which is unsuitable for reading; or the file was opened with the O_DIRECT flag, and either the address specified in buf, the value specified in count, or the file offset is not suitably aligned. Or, fd was created via a call to timerfd_create(2) and the wrong size buffer was given to read(); see timerfd_create(2) for further information. Or, whence is not valid. Or: the resulting file offset would be negative, or beyond the end of a seekable device."),
				EISDIR => panic!("fd refers to a directory"),
				
				unexpected @ _ => unreachable!("Unexpected error code from read completion of {}", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result from read completion of {}", result)
		}
	}
	
	/// Can not return `Ok(Some(value))` with `value` greater than `i32::MAX as u32`.
	///
	/// Returns `Err(Cancelled)` for `EIO`, `EDQUOT` and `ENOSPC`.
	/// Returns `Ok(None)` if cancelled.
	/// Returns `Err(WouldBlock)` if in progress with the `TCP_FASTOPEN_CONNECT` socket option which expects the client to write-first.
	/// Returns `Err(WouldBlock)` if `EAGAIN` or `ENOMEM`.
	#[inline(always)]
	pub fn write_vectored(self) -> Result<Option<u32>, StructWriteError>
	{
		let result = self.0;
		
		if likely!(result >= 0 )
		{
			Ok(Some(result as u32))
		}
		else if likely!(self.is_error())
		{
			use self::StructWriteError::*;
			
			match -self.0
			{
				ECANCELED => Ok(None),
				EAGAIN | ENOMEM | EINPROGRESS => Err(WouldBlock),
				EINTR => Err(Interrupted),
				EIO | EDQUOT | ENOSPC => Err(Cancelled),
				
				EBADF => panic!("fd is not a valid file descriptor or is not open for writing."),
				EDESTADDRREQ => panic!("fd refers to a datagram socket for which a peer address has not been set using connect(2)"),
				EFAULT => panic!("buf is outside your accessible address space"),
				EFBIG => panic!("An attempt was made to write a file that exceeds the implementation-defined maximum file size or the process's file size limit, or to write at a position past the maximum allowed offset"),
				EINVAL => panic!("fd is attached to an object which is unsuitable for writing; or the file was opened with the O_DIRECT flag, and either the address specified in buf, the value specified in count, or the file offset is not suitably aligned. Or, whence is not valid. Or: the resulting file offset would be negative, or beyond the end of a seekable device. Or the sum of the iov_len values overflows an ssize_t value. Or the vector count, iovcnt, is less than zero or greater than the permitted maximum"),
				EPERM => panic!("The operation was prevented by a file seal; see fcntl(2)."),
				EPIPE => panic!("fd is connected to a pipe or socket whose reading end is closed. When this happens the writing process will also receive a SIGPIPE signal. (Thus, the write return value is seen only if the program catches, blocks or ignores this signal)."),
				ENXIO=> panic!("whence is SEEK_DATA or SEEK_HOLE, and offset is beyond the end of the file, or whence is SEEK_DATA and offset is within a hole at the end of the file."),
				EOVERFLOW=> panic!("The resulting file offset cannot be represented in an off_t"),
				ESPIPE=> panic!("fd is associated with a pipe, socket, or FIFO"),
				EOPNOTSUPP=> panic!(" An unknown flag is specified in flags"),
				
				unexpected @ _ => unreachable!("Unexpected error code from write_vectored completion of {}", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result from write_vectored completion of {}", result)
		}
	}
	
	/// Can not return `Ok(Some(value))` with `value` greater than `i32::MAX as u32`.
	///
	/// Returns `Err(Cancelled)` for `EIO`, `EDQUOT` and `ENOSPC`.
	/// Returns `Ok(None)` if cancelled.
	/// Returns `Err(WouldBlock)` if in progress with the `TCP_FASTOPEN_CONNECT` socket option which expects the client to write-first.
	/// Returns `Err(WouldBlock)` if `EAGAIN` or `ENOMEM`.
	#[inline(always)]
	pub fn write_fixed(self) -> Result<Option<u32>, StructWriteError>
	{
		let result = self.0;
		
		if likely!(result >= 0 )
		{
			Ok(Some(result as u32))
		}
		else if likely!(self.is_error())
		{
			use self::StructWriteError::*;
			
			match -self.0
			{
				ECANCELED => Ok(None),
				EAGAIN | ENOMEM | EINPROGRESS => Err(WouldBlock),
				EINTR => Err(Interrupted),
				EIO | EDQUOT | ENOSPC => Err(Cancelled),
				
				EBADF => panic!("fd is not a valid file descriptor or is not open for writing."),
				EDESTADDRREQ => panic!("fd refers to a datagram socket for which a peer address has not been set using connect(2)"),
				EFAULT => panic!("buf is outside your accessible address space"),
				EFBIG => panic!("An attempt was made to write a file that exceeds the implementation-defined maximum file size or the process's file size limit, or to write at a position past the maximum allowed offset"),
				EINVAL => panic!("fd is attached to an object which is unsuitable for writing; or the file was opened with the O_DIRECT flag, and either the address specified in buf, the value specified in count, or the file offset is not suitably aligned. Or, whence is not valid. Or: the resulting file offset would be negative, or beyond the end of a seekable device. Or the sum of the iov_len values overflows an ssize_t value. Or the vector count, iovcnt, is less than zero or greater than the permitted maximum"),
				EPERM => panic!("The operation was prevented by a file seal; see fcntl(2)."),
				EPIPE => panic!("fd is connected to a pipe or socket whose reading end is closed. When this happens the writing process will also receive a SIGPIPE signal. (Thus, the write return value is seen only if the program catches, blocks or ignores this signal)."),
				ENXIO=> panic!("whence is SEEK_DATA or SEEK_HOLE, and offset is beyond the end of the file, or whence is SEEK_DATA and offset is within a hole at the end of the file."),
				EOVERFLOW=> panic!("The resulting file offset cannot be represented in an off_t"),
				ESPIPE=> panic!("fd is associated with a pipe, socket, or FIFO"),
				EOPNOTSUPP=> panic!(" An unknown flag is specified in flags"),
				
				unexpected @ _ => unreachable!("Unexpected error code from write_fixed completion of {}", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result from write_fixed completion of {}", result)
		}
	}
	
	/// Can not return `Ok(Some(value))` with `value` greater than `i32::MAX as u32`.
	///
	/// Equivalent to either `write()` (if offset zero) or `pwrite()` (if specified).
	///
	/// Returns `Err(Cancelled)` for `EIO`, `EDQUOT` and `ENOSPC`.
	/// Returns `Ok(None)` if cancelled.
	/// Returns `Err(WouldBlock)` if in progress with the `TCP_FASTOPEN_CONNECT` socket option which expects the client to write-first.
	/// Returns `Err(WouldBlock)` if `EAGAIN` or `ENOMEM`.
	#[inline(always)]
	pub fn write(self) -> Result<Option<u32>, StructReadError>
	{
		let result = self.0;
		
		if likely!(result >= 0 )
		{
			Ok(Some(result as u32))
		}
		else if likely!(self.is_error())
		{
			use self::StructReadError::*;
			
			match -self.0
			{
				ECANCELED => Ok(None),
				EAGAIN | ENOMEM | EINPROGRESS => Err(WouldBlock),
				EINTR => Err(Interrupted),
				EIO | EDQUOT | ENOSPC => Err(Cancelled),
				
				EBADF => panic!("fd is not a valid file descriptor or is not open for writing."),
				EDESTADDRREQ => panic!("fd refers to a datagram socket for which a peer address has not been set using connect(2)"),
				EFAULT => panic!("buf is outside your accessible address space"),
				EFBIG => panic!("An attempt was made to write a file that exceeds the implementation-defined maximum file size or the process's file size limit, or to write at a position past the maximum allowed offset"),
				EINVAL => panic!("fd is attached to an object which is unsuitable for writing; or the file was opened with the O_DIRECT flag, and either the address specified in buf, the value specified in count, or the file offset is not suitably aligned. Or, whence is not valid. Or: the resulting file offset would be negative, or beyond the end of a seekable device."),
				EPERM => panic!("The operation was prevented by a file seal; see fcntl(2)."),
				EPIPE => panic!("fd is connected to a pipe or socket whose reading end is closed. When this happens the writing process will also receive a SIGPIPE signal. (Thus, the write return value is seen only if the program catches, blocks or ignores this signal)."),
				ENXIO=> panic!("whence is SEEK_DATA or SEEK_HOLE, and offset is beyond the end of the file, or whence is SEEK_DATA and offset is within a hole at the end of the file."),
				EOVERFLOW=> panic!("The resulting file offset cannot be represented in an off_t"),
				ESPIPE=> panic!("fd is associated with a pipe, socket, or FIFO"),
				
				unexpected @ _ => unreachable!("Unexpected error code from write completion of {}", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result from write completion of {}", result)
		}
	}
	
	/// Returns `Ok(None)` if cancelled.
	#[inline(always)]
	pub fn poll_add(self) -> Result<Option<PollResponseFlags>, ()>
	{
		const U16Maximum: i32 = u16::MAX as i32;
		
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => Ok(None),
				EINTR | EAGAIN | ENOMEM => Err(()),
				
				EFAULT => panic!("The array given as argument was not contained in the calling program's address space."),
				EINVAL => panic!("The nfds value exceeds the RLIMIT_NOFILE value."),
				
				unexpected @ _ => unreachable!("Unexpected error code from poll_add completion of {}", unexpected),
			}
			
			value @ 0 ..= U16Maximum => match PollResponseFlags::from_bits(value as u16)
			{
				Some(poll_response_flags) => Ok(Some(poll_response_flags)),
				None => unreachable!("Invalid PollResponse flags from poll_add completion of {}", value),
			}
			
			unexpected @ _ => unreachable!("Unexpected result from poll_add completion of {}", unexpected)
		}
	}
	
	/// Returns `Ok(None)` if cancelled (not sure this is possible).
	/// Returns `Err(true)` if the request to cancel was already cancelled (not sure this is possible).
	/// Returns `Err(false)` if could not find the request to cancel.
	#[inline(always)]
	pub fn poll_cancel(self) -> Result<Option<()>, bool>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => Ok(None),
				
				EINTR | EAGAIN | ENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
				
				EALREADY => Err(true),
				
				ENOENT => Err(false),
				
				unexpected @ _ => unreachable!("Unexpected error code from poll_cancel completion of {}", unexpected),
			}
			
			0 => Ok(Some(())),
			
			unexpected @ _ => unreachable!("Unexpected result from poll_cancel completion of {}", unexpected)
		}
	}
	
	/// Returns `Ok(None)` if cancelled.
	/// Returns `Err(true)` if an error occurred during synchronization (`EIO`).
	/// Returns `Err(false)` if the file does not support synchronization.
	#[inline(always)]
	pub fn file_synchronize(self) -> Result<Option<()>, bool>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => Ok(None),
				
				EINTR | EAGAIN | ENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
				
				EIO => Err(true),
				
				EROFS | EINVAL => Err(false),
				
				EBADF=> panic!("fd is not a valid open file descriptor."),
				
				
				unexpected @ _ => unreachable!("Unexpected error code from file_synchronize completion of {}", unexpected),
			}
			
			0 => Ok(Some(())),
			
			unexpected @ _ => unreachable!("Unexpected result from file_synchronize completion of {}", unexpected)
		}
	}
	
	/// `pending_accept_connection` must be pinned to the same location as used in the `prepare_accept()` call.
	///
	/// It can be freed after this call.
	///
	/// Returns `Ok(None)` if cancelled.
	#[inline(always)]
	pub fn accept<SD: SocketData>(self, pending_accept_connection: PendingAcceptConnection<SD>) -> Result<Option<AcceptedConnection<SD>>, SocketAcceptError>
	{
		match self.0
		{
			error @ -4095 ..= -1 =>
			{
				use self::ConnectionFailedReason::*;
				use self::SocketAcceptError::*;
				
				let error = match -error
				{
					ECANCELED => return Ok(None),
					
					EAGAIN => Again,

					EINTR => Interrupted,

					ECONNABORTED => ConnectionFailed(Aborted),
					EPERM => ConnectionFailed(FirewallPermissionDenied),
					ETIMEDOUT => ConnectionFailed(TimedOut),
					EPROTO => ConnectionFailed(Protocol),

					EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,
					ENOBUFS | ENOMEM | ENOSR => KernelWouldBeOutOfMemory,

					EINVAL => panic!("Socket is not listening for connections, or `addrlen` is invalid, or the `flags` are invalid"),
					EFAULT => panic!("`addr` points outside the user's accessible address space"),
					EBADF => panic!("`sockfd` is not a valid descriptor"),
					ENOTSOCK => panic!("`sockfd` is not a socket file descriptor"),
					EOPNOTSUPP => panic!("The socket is not of a type that supports the `accept()` operation"),
					ESOCKTNOSUPPORT => panic!("ESOCKTNOSUPPORT"),
					EPROTONOSUPPORT => panic!("EPROTONOSUPPORT"),
					
					unexpected @ _ => unreachable!("Unexpected error code from accept4 completion of {}", unexpected),
				};
				Err(error)
			}
			
			file_descriptor if file_descriptor >= 0 =>
			{
				debug_assert_eq!(pending_accept_connection.peer_address_length, PendingAcceptConnection::<SD>::SocketDataLength(), "peer_address was truncated");
				
				Ok
				(
					Some
					(
						AcceptedConnection
						{
							streaming_socket_file_descriptor: unsafe { StreamingSocketFileDescriptor::from_raw_fd(file_descriptor) },
							peer: SocketDataWithLength
							{
								address: pending_accept_connection.peer_address,
								address_length: pending_accept_connection.peer_address_length as usize,
							},
						}
					)
				)
			}
			
			unexpected @ _ => unreachable!("Unexpected result from accept4 completion of {}", unexpected)
		}
	}
	
	/// `_peer_address` must be pinned to the same location as used in the `prepare_connect()` call.
	///
	/// It can be freed after this call.
	///
	/// Returns `Ok(None)` if cancelled.
	///
	/// Returns `Err(InProgress)` if the `TCP_FASTOPEN_CONNECT` socket option was specified, see `https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=19f6d3f3`.
	#[inline(always)]
	pub fn connect<SD: SocketData>(self, _peer_address: &SD) -> Result<Option<()>, SocketConnectError>
	{
		match self.0
		{
			error @ -4095 ..= -1 =>
			{
				use self::SocketConnectError::*;
				
				let error = match -error
				{
					ECANCELED => return Ok(None),
					
					ENOMEM => OutOfKernelMemory,
					
					EACCES | EPERM => PermissionDenied,
					EADDRINUSE => AddressInUse,
					EAGAIN => NoMoreFreeLocalPorts,
					ECONNREFUSED => ConnectionRefused,
					EINPROGRESS => InProgress,
					EINTR => Interrupted,
					ETIMEDOUT => TimedOut,
					ENETUNREACH => NetworkUnreachable,
					EISCONN => panic!("The socket is already connected."),
					EALREADY => panic!("The socket is nonblocking and a previous connection attempt has not yet been completed."),
					EBADF => panic!("`sockfd` is not a valid descriptor"),
					EINVAL => panic!("already bound, or the `addrlen` is wrong, or the socket was not in the `AF_UNIX` family"),
					ENOTSOCK => panic!("`sockfd` is not a socket file descriptor"),
					EFAULT => panic!("`addr` points outside the user's accessible address space"),
					EAFNOSUPPORT => panic!("Invalid `sa_family_t` value"),
					
					unexpected @ _ => unreachable!("Unexpected error code from connect completion of {}", unexpected),
				};
				Err(error)
			}
			
			0 => Ok(Some(())),
			
			unexpected @ _ => unreachable!("Unexpected result from connect completion of {}", unexpected)
		}
	}
	
	/// `buffer` must be pinned to the same location as used in the `prepare_receive()` call.
	///
	/// It can be freed after this call.
	///
	/// Returns `Ok(None)` if cancelled.
	///
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `UnexpectedEof` (if received a length of `0` *and* the buffer was not empty).
	/// * `WouldBlock`.
	/// * `Interrupted`.
	/// * `Other` (which is for when the kernel reports `ENOMEM`, ie it is out of memory).
	/// * `ConnectionReset` (seems to be posible in some circumstances for Unix domain sockets).
	/// * `ConnectionRefused` (only can happen for TCP client sockets; can not happen for sockets `accept()`ed by a server listener).
	///
	/// Can not return `Ok(Some(value))` with `value` greater than `i32::MAX as u32`.
	#[inline(always)]
	pub fn receive(self, buffer: &[u8]) -> io::Result<Option<u32>>
	{
		use crate::ErrorKind::*;
		
		match self.0
		{
			error @ -4095 ..= -1 =>
			{
				let error = match -error
				{
					ECANCELED => return Ok(None),
					
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
					
					unexpected @ _ => unreachable!("Unexpected error code from receive completion of {}", unexpected),
				};
				Err(io::Error::from(error))
			}
			
			0 => if buffer.is_empty()
			{
				Ok(Some(0))
			}
			else
			{
				Err(io::Error::from(UnexpectedEof))
			}
			
			length if length > 0 => Ok(Some(length as u32)),
			
			unexpected @ _ => unreachable!("Unexpected result from receive completion of {}", unexpected)
		}
	}
	
	/// `buffer` must be pinned to the same location as used in the `prepare_send()` call.
	///
	/// It can be freed after this call.
	///
	/// Returns `Ok(None)` if cancelled.
	///
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `WriteZero` (write returned an amount of zero; this is *NOT* an error if the caller originally called `prepare_send()` with an empty buffer).
	/// * `WouldBlock`.
	/// * `Interrupted`.
	/// * `Other` (which is for when the kernel reports `ENOMEM` or `ENOBUFS`, ie it is out of memory).
	/// * `BrokenPipe`.
	/// * `PermissionDenied` (only for Unix domain sockets).
	/// * `ConnectionReset`.
	/// * `ConnectionRefused`.
	///
	/// Can not return `Ok(Some(value))` with `value` greater than `i32::MAX as u32`.
	#[inline(always)]
	pub fn send(self, buffer: &[u8]) -> io::Result<Option<u32>>
	{
		use crate::ErrorKind::*;
		
		match self.0
		{
			error @ -4095 ..= -1 =>
			{
				let error = match -error
				{
					ECANCELED => return Ok(None),
					
					EAGAIN => WouldBlock,
					EINTR => Interrupted,
					ENOMEM | ENOBUFS => Other,
					EPIPE => BrokenPipe,
					EACCES => PermissionDenied,
					ECONNRESET => ConnectionReset,
					ECONNREFUSED => ConnectionRefused,
					EBADF => panic!("The argument `sockfd` is an invalid descriptor"),
					EFAULT => panic!("The receive buffer pointer(s) point outside the process's address space"),
					EINVAL => panic!("Invalid argument passed"),
					ENOTCONN => panic!("The socket is associated with a connection-oriented protocol and has not been connected"),
					ENOTSOCK => panic!("The argument `sockfd` does not refer to a socket"),
					EOPNOTSUPP => panic!("Some flags in the `flags` argument are inappropriate for the socket type"),
					EMSGSIZE => panic!("The socket type requires that message be sent atomically, and the size of the message to be sent made this impossible"),
					EISCONN => panic!("The connection-mode socket was connected already but a recipient was specified"),
					EDESTADDRREQ => panic!("The socket is not connection-mode, and no peer address is set"),
					
					unexpected @ _ => unreachable!("Unexpected error code from send completion of {}", unexpected),
				};
				Err(io::Error::from(error))
			}
			
			0 => if buffer.is_empty()
			{
				Ok(Some(0))
			}
			else
			{
				Err(io::Error::from(WriteZero))
			}
			
			length if length > 0 => Ok(Some(length as u32)),
			
			unexpected @ _ => unreachable!("Unexpected result from send completion of {}", unexpected)
		}
	}
	
	/// `_message` must be pinned to the same location as used in the `prepare_receive_message()` call.
	///
	/// It can be freed after this call.
	///
	/// Returns `Ok(None)` if cancelled.
	///
	/// Care must be taken in interpreting the result of Ok(Some(0))`:-
	///
	/// * When a stream socket peer has performed an orderly shutdown, the return value will be 0 (the traditional "end-of-file" return).
	/// * Datagram sockets in various domains (e.g., the UNIX and Internet domains) permit zero-length datagrams; when such a datagram is received, the return value is 0.
	/// * The value 0 may also be returned if the requested number of bytes to receive from a stream socket was 0.
	///
	/// Can not return `Ok(Some(value))` with `value` greater than `i32::MAX as u32`.
	#[inline(always)]
	pub fn receive_message<SD: SocketData>(self, _message: &mut ReceiveMessage<SD>) -> Result<Option<u32>, StructReadError>
	{
		match self.0
		{
			error @ -4095 ..= -1 =>
			{
				use self::StructReadError::*;
				
				let error = match -error
				{
					ECANCELED => return Ok(None),
					
					EAGAIN => WouldBlock,
					EINTR => Interrupted,
					EIO | ENOMEM => Cancelled,
					
					EBADF | ENOTSOCK => panic!("`fd` is not a valid socket file descriptor"),
					EFAULT => panic!("`buf` is outside your accessible address space"),
					EINVAL => panic!("`fd` is attached to an object which is unsuitable for reading OR was created via a call to `timerfd_create()` and the wrong size buffer was given to `read()`"),
					EISDIR => panic!("`fd` refers to a directory"),
					
					ENOTCONN=> panic!("Using recvmsg() for a connected socket which hasn't been connected"),
					ECONNREFUSED=> panic!("Using recvmsg() for a connected socket"),
					
					unexpected @ _ => unreachable!("Unexpected error code from receive_message completion of {}", unexpected),
				};
				Err(error)
			}
			
			length if length >= 0 => Ok(Some(length as u32)),
			
			unexpected @ _ => unreachable!("Unexpected result from receive_message completion of {}", unexpected)
		}
	}
	
	/// `_message` must be pinned to the same location as used in the `prepare_send_message()` call.
	///
	/// It can be freed after this call.
	///
	/// Returns `Ok(None)` if cancelled.
	///
	/// Care must be taken in interpreting the result of Ok(Some(0))`:-
	///
	/// * When a stream socket peer has performed an orderly shutdown, the return value will be 0 (the traditional "end-of-file" return).
	/// * Datagram sockets in various domains (e.g., the UNIX and Internet domains) permit zero-length datagrams; when such a datagram is received, the return value is 0.
	/// * The value 0 may also be returned if the requested number of bytes to receive from a stream socket was 0.
	///
	/// Can not return `Ok(Some(value))` with `value` greater than `i32::MAX as u32`.
	///
	/// Never returns `WriteZero`.
	#[inline(always)]
	pub fn send_message<SD: SocketData>(self, _message: &mut ReceiveMessage<SD>) -> io::Result<Option<u32>>
	{
		match self.0
		{
			error @ -4095 ..= -1 =>
			{
				use crate::ErrorKind::*;
				
				let error = match -error
				{
					ECANCELED => return Ok(None),
					
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
					
					unexpected @ _ => unreachable!("Unexpected error code from send_message completion of {}", unexpected),
				};
				Err(io::Error::from(error))
			}
			
			length if length >= 0 => Ok(Some(length as u32)),
			
			unexpected @ _ => unreachable!("Unexpected result from send_message completion of {}", unexpected)
		}
	}
	
	/// `_epoll_event` must be pinned to the same location as used in the `prepare_epoll_control_add()` call.
	///
	/// It can be freed after this call.
	///
	/// Returns `Ok(None)` if cancelled.
	#[inline(always)]
	pub fn epoll_control_add(self, _epoll_event: &epoll_event) -> Result<Option<()>, EPollAddError>
	{
		match self.0
		{
			error @ -4095 ..= -1 =>
			{
				use self::EPollAddError::*;
				
				match -error
				{
					ECANCELED => Ok(None),
					
					EINTR | EAGAIN => Err(TryAgain),
					
					ENOMEM => Err(ThereWasInsufficientKernelMemory),
					
					ENOSPC => Err(LimitOnWatchesWouldBeExceeded),
					
					EBADF => panic!("The supplied file descriptor was not a valid file descriptor"),
					EEXIST => panic!("The supplied file descriptor was already registered with this epoll instance"),
					EINVAL => panic!("Can not add epoll file descriptor to its self, or can not make wait on an epoll file descriptor `EPOLLEXCLUSIVE`"),
					ELOOP => panic!("The supplied file descriptor is for an epoll instance and this operation would result in a circular loop of epoll instances monitoring one another"),
					EPERM => panic!("The supplied file descriptor does not support epoll (perhaps it is an open regular file or the like)"),
					
					unexpected @ _ => unreachable!("Unexpected error code from epoll_control_add completion of {}", unexpected),
				}
			}
			
			0 => Ok(Some(())),
			
			unexpected @ _ => unreachable!("Unexpected result from epoll_control_add completion of {}", unexpected)
		}
	}
	
	/// `_epoll_event` must be pinned to the same location as used in the `prepare_epoll_control_add()` call.
	///
	/// It can be freed after this call.
	///
	/// Returns `Ok(None)` if cancelled.
	#[inline(always)]
	pub fn epoll_control_modify(self, _epoll_event: &epoll_event) -> Result<Option<()>, EPollModifyError>
	{
		match self.0
		{
			error @ -4095 ..= -1 =>
			{
				use self::EPollModifyError::*;
				
				match -error
				{
					ECANCELED => Ok(None),
					
					EINTR | EAGAIN => Err(TryAgain),
					
					ENOMEM => Err(ThereWasInsufficientKernelMemory),
					
					EBADF => panic!("The supplied file descriptor was not a valid file descriptor"),
					EINVAL => panic!("Supplied file descriptor was not usable or there was the presence or absence of `Exclusive` when required"),
					ENOENT => panic!("The supplied file descriptor is not registered with this epoll instance"),
					EPERM => panic!("The supplied file descriptor does not support epoll (perhaps it is an open regular file or the like)"),
					
					unexpected @ _ => unreachable!("Unexpected error code from epoll_control_modify completion of {}", unexpected),
				}
			}
			
			0 => Ok(Some(())),
			
			unexpected @ _ => unreachable!("Unexpected result from epoll_control_modify completion of {}", unexpected)
		}
	}
	
	/// Returns `Ok(None)` if cancelled.
	///
	/// Returns `Err(())` if the interupted or out of kernel memory.
	#[inline(always)]
	pub fn epoll_control_delete(self) -> Result<Option<()>, ()>
	{
		match self.0
		{
			error @ -4095 ..= -1 =>
			{
				match -error
				{
					ECANCELED => Ok(None),
					
					EINTR | EAGAIN => Err(()),
					
					ENOMEM => panic!("Examination of the Linux source code fs/eventpoll.c suggests `ENOMEM` should not occur for `EPOLL_CTL_DEL`"),
					
					EBADF => panic!("The supplied file descriptor was not a valid file descriptor"),
					EINVAL => panic!("Supplied file descriptor was not usable"),
					ENOENT => panic!("The supplied file descriptor is not registered with this epoll instance"),
					EPERM => panic!("The supplied file descriptor does not support epoll (perhaps it is an open regular file or the like)"),
					
					unexpected @ _ => unreachable!("Unexpected error code from epoll_control_delete completion of {}", unexpected),
				}
			}
			
			0 => Ok(Some(())),
			
			unexpected @ _ => unreachable!("Unexpected result from epoll_control_delete completion of {}", unexpected)
		}
	}
	
	/// Returns `Ok(None)` if cancelled (not sure this is possible).
	/// Returns `Err(true)` if the request to cancel was already cancelled (not sure this is possible).
	/// Returns `Err(false)` if could not find the request to cancel.
	#[inline(always)]
	pub fn cancel(self) -> Result<Option<()>, bool>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => Ok(None),
				
				EALREADY => Err(true),
				
				ENOENT => Err(false),
				
				EINTR | EAGAIN | ENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
				
				unexpected @ _ => unreachable!("Unexpected error code from cancel completion of {}", unexpected),
			}
			
			0 => Ok(Some(())),
			
			unexpected @ _ => unreachable!("Unexpected result from cancel completion of {}", unexpected)
		}
	}
	
	/// Returns `Ok(None)` if cancelled.
	#[inline(always)]
	pub fn nop(self) -> Option<()>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => None,
				
				EINTR | EAGAIN | ENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
				
				unexpected @ _ => unreachable!("Unexpected error code from nop completion of {}", unexpected),
			}
			
			0 => Some(()),
			
			unexpected @ _ => unreachable!("Unexpected result from nop completion of {}", unexpected)
		}
	}
	
	/// `_timeout` must be pinned to the same location as used in the `prepare_timeout()` call.
	///
	/// It can be freed after this call.
	///
	/// Returns `Ok(None)` if cancelled.
	/// Returns `Ok(Some(true))` if the timeout completed without the timeout firing (because enough completions occurred).
	/// Returns `Ok(Some(false))` if the timeout expired without enough completions occurring.
	/// Returns `Err(())` if kernel ran out of memory or failed in some other way.
	#[inline(always)]
	pub fn timeout(self, _timeout: &__kernel_timespec) -> Result<Option<bool>, ()>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => Ok(None),
				
				ETIME => Ok(Some(false)),
				
				EINTR | EAGAIN | ENOMEM => Err(()),
				
				unexpected @ _ => unreachable!("Unexpected error code from timeout completion of {}", unexpected),
			}
			
			0 => Ok(Some(true)),
			
			unexpected @ _ => unreachable!("Unexpected result from timeout completion of {}", unexpected)
		}
	}
	
	/// `_timeout` must be pinned to the same location as used in the `prepare_linked_timeout()` call.
	///
	/// It can be freed after this call.
	///
	/// Returns `None` if cancelled.
	/// Returns `Err(true)` if the timeout completed without the timeout firing (because the linked `SQE` completed in time).
	/// Returns `Err(false)` if the timeout expired.
	pub fn linked_timeout(self, _timeout: &__kernel_timespec) -> Option<bool>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => None,
				
				ETIME => Some(false),
				
				EINTR | EAGAIN | ENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
				
				unexpected @ _ => unreachable!("Unexpected error code from linked_timeout completion of {}", unexpected),
			}
			
			0 => Some(true),
			
			unexpected @ _ => unreachable!("Unexpected result from linked_timeout completion of {}", unexpected)
		}
	}
	
	/// Returns `Ok(None)` if cancelled (not sure this is possible).
	/// Returns `Err(None)` if the request to cancel was not possible as the timeout expiration was already in progress.
	/// Returns `Err(Some(true))` if the request to cancel was already cancelled (not sure this is possible).
	/// Returns `Err(Some(false))` if could not find the request to cancel.
	#[inline(always)]
	pub fn cancel_timeout(self) -> Result<Option<()>, Option<bool>>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => Ok(None),
				
				EALREADY => Err(Some(true)),
				
				ENOENT => Err(Some(false)),
				
				EBUSY => Err(None),
				
				EINTR | EAGAIN | ENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
				
				unexpected @ _ => unreachable!("Unexpected error code from cancel_timeout completion of {}", unexpected),
			}
			
			0 => Ok(Some(())),
			
			unexpected @ _ => unreachable!("Unexpected result from cancel_timeout completion of {}", unexpected)
		}
	}
	
	/// Returns `Ok(None)` if cancelled (not sure this is possible).
	/// Returns an `Err(true)` if interupted by a signal or file is currently being executed.
	/// Returns an `Err(false)` if out-of-disk space.
	#[inline(always)]
	pub fn file_allocate(self) -> Result<Option<()>, bool>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => Ok(None),
				
				EINTR | EAGAIN| ENOMEM => Err(true),
				ENOSPC => Err(false),
				
				// mode specifies FALLOC_FL_COLLAPSE_RANGE or FALLOC_FL_INSERT_RANGE, but the file referred to by fd is currently being executed.
				ETXTBSY => Err(true),
				
				EBADF => panic!("fd is not a valid file descriptor, or is not opened for writing"),
				EFBIG => panic!("`offset + length` exceeds the maximum file size"),
				EINVAL => panic!("offset was less than 0, or len was less than or equal to 0, or the underlying filesystem does not support the operation"),
				ENODEV => panic!("fd does not refer to a regular file"),
				ESPIPE => panic!("fd refers to a pipe"),
				ENOSYS => panic!("This kernel does not implement fallocate()"),
				EOPNOTSUPP => panic!("The filesystem containing the file referred to by fd does not support this operation; or the mode is not supported by the filesystem containing the file referred to by fd"),
				EPERM => panic!("The file referred to by fd is marked immutable (see chattr(1)). Or mode specifies FALLOC_FL_PUNCH_HOLE or FALLOC_FL_COLLAPSE_RANGE or FALLOC_FL_INSERT_RANGE and the file referred to by fd is marked append-only (see chattr(1)). Or the operation was prevented by a file seal; see fcntl(2)."),
				
				unexpected @ _ => unreachable!("Unexpected error code from file_allocate completion of {}", unexpected),
			}
			
			0 => Ok(Some(())),
			
			unexpected @ _ => unreachable!("Unexpected result from file_allocate completion of {}", unexpected)
		}
	}
	
	/// Returns `None` if cancelled.
	/// Doesn't return errors that can be usefully handled.
	#[inline(always)]
	pub fn file_advise(self) -> Option<()>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => None,
				
				EINTR | EAGAIN | ENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
				
				EINVAL => panic!("An invalid value was specified for advice"),
				EBADF => panic!("fd is not a valid file descriptor"),
				ESPIPE => panic!("The specified file descriptor refers to a pipe or FIFO (befor Linux 2.6.16, this was EINVAL)"),
				
				unexpected @ _ => unreachable!("Unexpected error code from file_advise completion of {}", unexpected),
			}
			
			0 => Some(()),
			
			unexpected @ _ => unreachable!("Unexpected result from file_advise completion of {}", unexpected)
		}
	}
	
	/// Returns `Ok(None)` if cancelled.
	/// Returns an `Err(true)` if `EIO`.
	/// Returns an `Err(false)` if out-of-disk space (`ENOMEM` or `ENOSPC`).
	#[inline(always)]
	pub fn synchronize_file_range(self) -> Result<Option<()>, bool>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => Ok(None),
				
				EIO => Err(true),
				ENOMEM | ENOSPC => Err(false),
				
				EINTR | EAGAIN => panic!("EINTR / EAGAIN - are these possible?"),
				
				EBADF => panic!("fd is not a valid file descriptor"),
				EINVAL => panic!("flags specifies an invalid bit; or offset or nbytes is invalid"),
				ESPIPE => panic!("fd refers to something other than a regular file, a block device, a directory, or a symbolic link"),
				
				unexpected @ _ => unreachable!("Unexpected error code from synchronize_file_range completion of {}", unexpected),
			}
			
			0 => Ok(Some(())),
			
			unexpected @ _ => unreachable!("Unexpected result from synchronize_file_range completion of {}", unexpected)
		}
	}
	
	/// `_mapped_memory` must be pinned to the same location as used in the `prepare_memory_advise()` call.
	///
	/// Returns `Ok(None)` if cancelled.
	#[inline(always)]
	pub fn memory_advise(self, _mapped_memory: &MappedMemory) -> io::Result<Option<()>>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => Ok(None),
				
				EINTR | EAGAIN | ENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
				
				code @ _ => Err(io::Error::from_raw_os_error(code)),
			}
			
			0 => Ok(Some(())),
			
			unexpected @ _ => unreachable!("Unexpected result from memory_advise completion of {}", unexpected)
		}
	}
	
	/// Returns `None` if cancelled.
	/// Returns an `Some(true)` if successful.
	/// Returns `Some(false)` for `EIO`, `EINTR`, `EAGAIN`, `ENOMEM` or `ENOSPC`.
	///
	/// Regardless of the outcome, the caller should consider the underlying file descriptor unusable.
	#[inline(always)]
	pub fn close(self) -> Option<bool>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => None,
				
				EIO | EINTR | EAGAIN | ENOMEM | ENOSPC => Some(false),
				
				EBADF=> panic!("fd isn't a valid open file descriptor."),
				
				unexpected @ _ => unreachable!("Unexpected error code from close completion of {}", unexpected),
			}
			
			0 => Some(true),
			
			unexpected @ _ => unreachable!("Unexpected result from close completion of {}", unexpected)
		}
	}
	
	/// Returns `Ok(None)` if cancelled.
	#[deprecated(since = "0.0.0", note = "Only use this if previously used the deprecated SubmissionQueueEntry::prepare_openat()")]
	#[inline(always)]
	pub fn openat<Open: OnDiskFileDescriptor>(self) -> io::Result<Option<Open>>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => Ok(None),
				
				code @ _ => Err(io::Error::from_raw_os_error(code)),
			}
			
			raw_file_descriptor if raw_file_descriptor >= 0 => Ok(Some(unsafe { Open::from_raw_fd(raw_file_descriptor) })),
			
			unexpected @ _ => unreachable!("Unexpected result from openat completion of {}", unexpected)
		}
	}
	
	/// `_open_on_disk` must be pinned to the same location as used in the `prepare_openat2()` call.
	///
	/// Returns `Ok(None)` if cancelled.
	#[inline(always)]
	pub fn openat2<Open: OnDiskFileDescriptor>(self, _open_on_disk: &OpenOnDisk<Open>) -> io::Result<Option<Open>>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => Ok(None),
				
				code @ _ => Err(io::Error::from_raw_os_error(code)),
			}
			
			raw_file_descriptor if raw_file_descriptor >= 0 => Ok(Some(unsafe { Open::from_raw_fd(raw_file_descriptor) })),
			
			unexpected @ _ => unreachable!("Unexpected result from openat2 completion of {}", unexpected)
		}
	}
	
	/// Extended metadata.
	#[inline(always)]
	pub fn extended_metadata_for_path(self, _path: &CStr) -> io::Result<Option<()>>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => Ok(None),
				
				EINTR | EAGAIN => panic!("EINTR / EAGAIN - are these possible?"),
				
				EACCES | ELOOP | ENAMETOOLONG | ENOMEM | ENOTDIR | ENOENT => Err(io::Error::from_raw_os_error(-error)),
				
				EBADF => panic!("dirfd is not a valid open file descriptor"),
				EFAULT => panic!("pathname or statxbuf is NULL or points to a location outside the process's accessible address space"),
				EINVAL => panic!("Invalid flag specified in flags. Or, reserved flag specified in mask. (Currently, there is one such flag, designated by the constant STATX__RESERVED, with the value 0x80000000U)."),
				
				unexpected @ _ => unreachable!("Unexpected error code from extended_metadata_for_path completion of {}", unexpected),
			}
			
			0 => Ok(Some(())),
			
			unexpected @ _ => unreachable!("Unexpected result from extended_metadata_for_path completion of {}", unexpected)
		}
	}
	
	/// Extended metadata.
	#[inline(always)]
	pub fn extended_metadata_for_directory(self) -> io::Result<Option<()>>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => Ok(None),
				
				EINTR | EAGAIN => panic!("EINTR / EAGAIN - are these possible?"),
				
				EACCES | ELOOP | ENAMETOOLONG | ENOMEM => Err(io::Error::from_raw_os_error(-error)),
				
				EBADF => panic!("dirfd is not a valid open file descriptor"),
				EFAULT => panic!("pathname or statxbuf is NULL or points to a location outside the process's accessible address space"),
				EINVAL => panic!("Invalid flag specified in flags. Or, reserved flag specified in mask. (Currently, there is one such flag, designated by the constant STATX__RESERVED, with the value 0x80000000U)."),
				ENOENT => panic!("ENOENT A component of pathname does not exist, or pathname is an empty string and AT_EMPTY_PATH was not specified in flags."),
				ENOTDIR => panic!("A component of the path prefix of pathname is not a directory or pathname is relative and dirfd is a file descriptor referring to a file other than a directory"),
				
				unexpected @ _ => unreachable!("Unexpected error code from extended_metadata_for_directory completion of {}", unexpected),
			}
			
			0 => Ok(Some(())),
			
			unexpected @ _ => unreachable!("Unexpected result from extended_metadata_for_directory completion of {}", unexpected)
		}
	}
	
	/// Registered file descriptors update.
	///
	/// `replace_with_files_descriptors` must be pinned to the same location as used in the `prepare_registered_file_descriptors_update()` call.
	#[inline(always)]
	pub fn registered_file_descriptors_update(self, replace_with_files_descriptors: &[SupportedFileDescriptor]) -> Option<u32>
	{
		match self.0
		{
			error @ -4095 ..= -1 => match -error
			{
				ECANCELED => None,
				
				EINVAL => panic!("Unsupported"),
				
				unexpected @ _ => unreachable!("Unexpected error code from registered_file_descriptors_update completion of {}", unexpected),
			}
			
			count if count >= 0 =>
			{
				debug_assert!(count <= replace_with_files_descriptors.len() as i32);
				Some(count as u32)
			}
			
			unexpected @ _ => unreachable!("Unexpected result from registered_file_descriptors_update completion of {}", unexpected)
		}
	}
	
	#[inline(always)]
	fn is_error(self) -> bool
	{
		Self::ErrorResponseRange.contains(&self.0)
	}
	
	const ErrorResponseRange: RangeInclusive<i32> = -4095 ..= -1;
}
