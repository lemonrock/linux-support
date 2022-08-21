// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! unexpected_error_in_completion
{
	($self: ident, $function_name: tt) =>
	{
		{
			const Literal: &'static str = concat!($function_name, " (completion)");
			unexpected_error!(Literal, unsafe { SystemCallErrorNumber::from_unchecked($self.0) })
		}
	}
}

macro_rules! unexpected_result_in_completion
{
	($self: ident, $function_name: tt) =>
	{
		{
			const Literal: &'static str = concat!($function_name, " (completion)");
			unexpected_result!(Literal, unsafe { SystemCallErrorNumber::from_unchecked($self.0) })
		}
	}
}

/// A completion response code.
#[repr(transparent)]
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
		use self::StructWriteError::*;
		
		match self.0
		{
			ok @ 0 ..= i32::MAX => Self::ok_u32(ok),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => Err(WouldBlock),
			SystemCallErrorNumber::NegativeEINTR => Err(Interrupted),
			SystemCallErrorNumber::NegativeEBADF => panic!("One or both file descriptors are not valid, or do not have proper read-write mode"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("The target filesystem doesn't support splicing; or the target file is opened in append mode; or neither of the file descriptors refers to a pipe; or an offset was given for a non-seekable device (eg, a pipe); or `fd_in` and `fd_out` refer to the same pipe"),
			SystemCallErrorNumber::NegativeESPIPE => panic!("Either `off_in` or `off_out` was not `NULL`, but the corresponding file descriptor refers to a pipe"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, splice),
			
			_ => unexpected_result_in_completion!(self, splice),
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
		use self::StructReadError::*;
		
		match self.0
		{
			ok @ 0 ..= i32::MAX => Self::ok_u32(ok),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => Err(WouldBlock),
			SystemCallErrorNumber::NegativeEINTR => Err(Interrupted),
			SystemCallErrorNumber::NegativeEIO => Err(Cancelled),
			SystemCallErrorNumber::NegativeEFAULT => panic!("buf is outside your accessible address space"),
			SystemCallErrorNumber::NegativeEISDIR => panic!("fd refers to a directory"),
			SystemCallErrorNumber::NegativeEBADF => panic!("fd is not a valid file descriptor or is not open for reading"),
			SystemCallErrorNumber::NegativeENXIO => panic!("whence is SEEK_DATA or SEEK_HOLE, and offset is beyond the end of the file, or whence is SEEK_DATA and offset is within a hole at the end of the file."),
			SystemCallErrorNumber::NegativeEOVERFLOW => panic!("The resulting file offset cannot be represented in an off_t."),
			SystemCallErrorNumber::NegativeESPIPE => panic!("fd is associated with a pipe, socket, or FIFO."),
			SystemCallErrorNumber::NegativeEINVAL => panic!("fd is attached to an object which is unsuitable for reading; or the file was opened with the O_DIRECT flag, and either the address specified in buf, the value specified in count, or the file offset is not suitably aligned. Or, fd was created via a call to timerfd_create(2) and the wrong size buffer was given to read(); see timerfd_create(2) for further information. Or, whence is not valid. Or: the resulting file offset would be negative, or beyond the end of a seekable device. Or the sum of the iov_len values overflows an ssize_t value. Or, the vector count, iovcnt, is less than zero or greater than the permitted maximum."),
			SystemCallErrorNumber::NegativeEOPNOTSUPP => panic!("An unknown flag is specified in flags."),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, read_vectored),
			
			_ => unexpected_result_in_completion!(self, read_vectored),
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
		use self::StructReadError::*;
		
		match self.0
		{
			ok @ 0 ..= i32::MAX => Self::ok_u32(ok),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => Err(WouldBlock),
			SystemCallErrorNumber::NegativeEINTR => Err(Interrupted),
			SystemCallErrorNumber::NegativeEIO => Err(Cancelled),
			SystemCallErrorNumber::NegativeEFAULT => panic!("buf is outside your accessible address space"),
			SystemCallErrorNumber::NegativeEISDIR => panic!("fd refers to a directory"),
			SystemCallErrorNumber::NegativeEBADF => panic!("fd is not a valid file descriptor or is not open for reading"),
			SystemCallErrorNumber::NegativeENXIO => panic!("whence is SEEK_DATA or SEEK_HOLE, and offset is beyond the end of the file, or whence is SEEK_DATA and offset is within a hole at the end of the file."),
			SystemCallErrorNumber::NegativeEOVERFLOW => panic!("The resulting file offset cannot be represented in an off_t."),
			SystemCallErrorNumber::NegativeESPIPE => panic!("fd is associated with a pipe, socket, or FIFO."),
			SystemCallErrorNumber::NegativeEINVAL => panic!("fd is attached to an object which is unsuitable for reading; or the file was opened with the O_DIRECT flag, and either the address specified in buf, the value specified in count, or the file offset is not suitably aligned. Or, fd was created via a call to timerfd_create(2) and the wrong size buffer was given to read(); see timerfd_create(2) for further information. Or, whence is not valid. Or: the resulting file offset would be negative, or beyond the end of a seekable device. Or the sum of the iov_len values overflows an ssize_t value. Or, the vector count, iovcnt, is less than zero or greater than the permitted maximum."),
			SystemCallErrorNumber::NegativeEOPNOTSUPP => panic!("An unknown flag is specified in flags."),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, read_fixed),
			
			_ => unexpected_result_in_completion!(self, read_fixed),
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
		use self::StructReadError::*;
		
		match self.0
		{
			ok @ 0 ..= i32::MAX => Self::ok_u32(ok),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => Err(WouldBlock),
			SystemCallErrorNumber::NegativeEINTR => Err(Interrupted),
			SystemCallErrorNumber::NegativeEIO => Err(Cancelled),
			SystemCallErrorNumber::NegativeENXIO => panic!("whence is SEEK_DATA or SEEK_HOLE, and offset is beyond the end of the file, or whence is SEEK_DATA and offset is within a hole at the end of the file."),
			SystemCallErrorNumber::NegativeEOVERFLOW => panic!("The resulting file offset cannot be represented in an off_t."),
			SystemCallErrorNumber::NegativeESPIPE => panic!("fd is associated with a pipe, socket, or FIFO."),
			SystemCallErrorNumber::NegativeEBADF => panic!("fd is not a valid file descriptor or is not open for reading"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("fd is attached to an object which is unsuitable for reading; or the file was opened with the O_DIRECT flag, and either the address specified in buf, the value specified in count, or the file offset is not suitably aligned. Or, fd was created via a call to timerfd_create(2) and the wrong size buffer was given to read(); see timerfd_create(2) for further information. Or, whence is not valid. Or: the resulting file offset would be negative, or beyond the end of a seekable device."),
			SystemCallErrorNumber::NegativeEISDIR => panic!("fd refers to a directory"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, read),
			
			_ => unexpected_result_in_completion!(self, read),
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
		use self::StructWriteError::*;
		
		match self.0
		{
			ok @ 0 ..= i32::MAX => Self::ok_u32(ok),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM | SystemCallErrorNumber::NegativeEINPROGRESS => Err(WouldBlock),
			SystemCallErrorNumber::NegativeEINTR => Err(Interrupted),
			SystemCallErrorNumber::NegativeEIO | SystemCallErrorNumber::NegativeEDQUOT | SystemCallErrorNumber::NegativeENOSPC => Err(Cancelled),
			SystemCallErrorNumber::NegativeEBADF => panic!("fd is not a valid file descriptor or is not open for writing."),
			SystemCallErrorNumber::NegativeEDESTADDRREQ => panic!("fd refers to a datagram socket for which a peer address has not been set using connect(2)"),
			SystemCallErrorNumber::NegativeEFAULT => panic!("buf is outside your accessible address space"),
			SystemCallErrorNumber::NegativeEFBIG => panic!("An attempt was made to write a file that exceeds the implementation-defined maximum file size or the process's file size limit, or to write at a position past the maximum allowed offset"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("fd is attached to an object which is unsuitable for writing; or the file was opened with the O_DIRECT flag, and either the address specified in buf, the value specified in count, or the file offset is not suitably aligned. Or, whence is not valid. Or: the resulting file offset would be negative, or beyond the end of a seekable device. Or the sum of the iov_len values overflows an ssize_t value. Or the vector count, iovcnt, is less than zero or greater than the permitted maximum"),
			SystemCallErrorNumber::NegativeEPERM => panic!("The operation was prevented by a file seal; see fcntl(2)."),
			SystemCallErrorNumber::NegativeEPIPE => panic!("fd is connected to a pipe or socket whose reading end is closed. When this happens the writing process will also receive a SIGPIPE signal. (Thus, the write return value is seen only if the program catches, blocks or ignores this signal)."),
			SystemCallErrorNumber::NegativeENXIO => panic!("whence is SEEK_DATA or SEEK_HOLE, and offset is beyond the end of the file, or whence is SEEK_DATA and offset is within a hole at the end of the file."),
			SystemCallErrorNumber::NegativeEOVERFLOW => panic!("The resulting file offset cannot be represented in an off_t"),
			SystemCallErrorNumber::NegativeESPIPE => panic!("fd is associated with a pipe, socket, or FIFO"),
			SystemCallErrorNumber::NegativeEOPNOTSUPP => panic!(" An unknown flag is specified in flags"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, write_vectored),
			
			_ => unexpected_result_in_completion!(self, write_vectored),
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
		use self::StructWriteError::*;
		
		match self.0
		{
			ok @ 0 ..= i32::MAX => Self::ok_u32(ok),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM | SystemCallErrorNumber::NegativeEINPROGRESS => Err(WouldBlock),
			SystemCallErrorNumber::NegativeEINTR => Err(Interrupted),
			SystemCallErrorNumber::NegativeEIO | SystemCallErrorNumber::NegativeEDQUOT | SystemCallErrorNumber::NegativeENOSPC => Err(Cancelled),
			SystemCallErrorNumber::NegativeEBADF => panic!("fd is not a valid file descriptor or is not open for writing."),
			SystemCallErrorNumber::NegativeEDESTADDRREQ => panic!("fd refers to a datagram socket for which a peer address has not been set using connect(2)"),
			SystemCallErrorNumber::NegativeEFAULT => panic!("buf is outside your accessible address space"),
			SystemCallErrorNumber::NegativeEFBIG => panic!("An attempt was made to write a file that exceeds the implementation-defined maximum file size or the process's file size limit, or to write at a position past the maximum allowed offset"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("fd is attached to an object which is unsuitable for writing; or the file was opened with the O_DIRECT flag, and either the address specified in buf, the value specified in count, or the file offset is not suitably aligned. Or, whence is not valid. Or: the resulting file offset would be negative, or beyond the end of a seekable device. Or the sum of the iov_len values overflows an ssize_t value. Or the vector count, iovcnt, is less than zero or greater than the permitted maximum"),
			SystemCallErrorNumber::NegativeEPERM => panic!("The operation was prevented by a file seal; see fcntl(2)."),
			SystemCallErrorNumber::NegativeEPIPE => panic!("fd is connected to a pipe or socket whose reading end is closed. When this happens the writing process will also receive a SIGPIPE signal. (Thus, the write return value is seen only if the program catches, blocks or ignores this signal)."),
			SystemCallErrorNumber::NegativeENXIO => panic!("whence is SEEK_DATA or SEEK_HOLE, and offset is beyond the end of the file, or whence is SEEK_DATA and offset is within a hole at the end of the file."),
			SystemCallErrorNumber::NegativeEOVERFLOW => panic!("The resulting file offset cannot be represented in an off_t"),
			SystemCallErrorNumber::NegativeESPIPE => panic!("fd is associated with a pipe, socket, or FIFO"),
			SystemCallErrorNumber::NegativeEOPNOTSUPP => panic!(" An unknown flag is specified in flags"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, write_fixed),
			
			_ => unexpected_result_in_completion!(self, write_fixed),
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
	pub fn write(self) -> Result<Option<u32>, StructWriteError>
	{
		use self::StructWriteError::*;
		
		match self.0
		{
			ok @ 0 ..= i32::MAX => Self::ok_u32(ok),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM | SystemCallErrorNumber::NegativeEINPROGRESS => Err(WouldBlock),
			SystemCallErrorNumber::NegativeEINTR => Err(Interrupted),
			SystemCallErrorNumber::NegativeEIO | SystemCallErrorNumber::NegativeEDQUOT | SystemCallErrorNumber::NegativeENOSPC => Err(Cancelled),
			SystemCallErrorNumber::NegativeEBADF => panic!("fd is not a valid file descriptor or is not open for writing."),
			SystemCallErrorNumber::NegativeEDESTADDRREQ => panic!("fd refers to a datagram socket for which a peer address has not been set using connect(2)"),
			SystemCallErrorNumber::NegativeEFAULT => panic!("buf is outside your accessible address space"),
			SystemCallErrorNumber::NegativeEFBIG => panic!("An attempt was made to write a file that exceeds the implementation-defined maximum file size or the process's file size limit, or to write at a position past the maximum allowed offset"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("fd is attached to an object which is unsuitable for writing; or the file was opened with the O_DIRECT flag, and either the address specified in buf, the value specified in count, or the file offset is not suitably aligned. Or, whence is not valid. Or: the resulting file offset would be negative, or beyond the end of a seekable device."),
			SystemCallErrorNumber::NegativeEPERM => panic!("The operation was prevented by a file seal; see fcntl(2)."),
			SystemCallErrorNumber::NegativeEPIPE => panic!("fd is connected to a pipe or socket whose reading end is closed. When this happens the writing process will also receive a SIGPIPE signal. (Thus, the write return value is seen only if the program catches, blocks or ignores this signal)."),
			SystemCallErrorNumber::NegativeENXIO => panic!("whence is SEEK_DATA or SEEK_HOLE, and offset is beyond the end of the file, or whence is SEEK_DATA and offset is within a hole at the end of the file."),
			SystemCallErrorNumber::NegativeEOVERFLOW => panic!("The resulting file offset cannot be represented in an off_t"),
			SystemCallErrorNumber::NegativeESPIPE => panic!("fd is associated with a pipe, socket, or FIFO"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, write),
			
			_ => unexpected_result_in_completion!(self, write),
		}
	}
	
	/// Returns `Ok(None)` if cancelled.
	#[inline(always)]
	pub fn poll_add(self) -> Result<Option<PollResponseFlags>, ()>
	{
		const U16Maximum: i32 = u16::MAX as i32;
		
		match self.0
		{
			ok @ 0 ..= U16Maximum => match PollResponseFlags::from_bits(ok as u16)
			{
				Some(poll_response_flags) => Ok(Some(poll_response_flags)),
				
				None => unreachable_code(format_args!("Invalid PollResponse flags from poll_add completion of {}", ok)),
			}
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => Err(()),
			SystemCallErrorNumber::NegativeEFAULT => panic!("The array given as argument was not contained in the calling program's address space."),
			SystemCallErrorNumber::NegativeEINVAL => panic!("The nfds value exceeds the RLIMIT_NOFILE value."),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, poll_add),
			
			_ => unexpected_result_in_completion!(self, poll_add),
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
			0 => Ok(Some(())),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
			SystemCallErrorNumber::NegativeEALREADY => Err(true),
			SystemCallErrorNumber::NegativeENOENT => Err(false),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, poll_cancel),
			
			_ => unexpected_result_in_completion!(self, poll_cancel),
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
			0 => Ok(Some(())),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
			SystemCallErrorNumber::NegativeEIO => Err(true),
			SystemCallErrorNumber::NegativeEROFS | SystemCallErrorNumber::NegativeEINVAL => Err(false),
			SystemCallErrorNumber::NegativeEBADF => panic!("fd is not a valid open file descriptor."),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, file_synchronize),
			
			_ => unexpected_result_in_completion!(self, file_synchronize),
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
		use self::ConnectionFailedReason::*;
		use self::SocketAcceptError::*;
		
		match self.0
		{
			file_descriptor @ 0 ..= i32::MAX =>
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
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEAGAIN => Err(Again),
			SystemCallErrorNumber::NegativeEINTR => Err(Interrupted),
			SystemCallErrorNumber::NegativeECONNABORTED => Err(ConnectionFailed(Aborted)),
			SystemCallErrorNumber::NegativeEPERM => Err(ConnectionFailed(FirewallPermissionDenied)),
			SystemCallErrorNumber::NegativeETIMEDOUT => Err(ConnectionFailed(TimedOut)),
			SystemCallErrorNumber::NegativeEPROTO => Err(ConnectionFailed(Protocol)),
			SystemCallErrorNumber::NegativeEMFILE => Err(PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded),
			SystemCallErrorNumber::NegativeENFILE => Err(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded),
			SystemCallErrorNumber::NegativeENOBUFS | SystemCallErrorNumber::NegativeENOMEM | SystemCallErrorNumber::NegativeENOSR => Err(KernelWouldBeOutOfMemory),
			SystemCallErrorNumber::NegativeEINVAL => panic!("Socket is not listening for connections, or `addrlen` is invalid, or the `flags` are invalid"),
			SystemCallErrorNumber::NegativeEFAULT => panic!("`addr` points outside the user's accessible address space"),
			SystemCallErrorNumber::NegativeEBADF => panic!("`sockfd` is not a valid descriptor"),
			SystemCallErrorNumber::NegativeENOTSOCK => panic!("`sockfd` is not a socket file descriptor"),
			SystemCallErrorNumber::NegativeEOPNOTSUPP => panic!("The socket is not of a type that supports the `accept()` operation"),
			SystemCallErrorNumber::NegativeESOCKTNOSUPPORT => panic!("ESOCKTNOSUPPORT"),
			SystemCallErrorNumber::NegativeEPROTONOSUPPORT => panic!("EPROTONOSUPPORT"),
			unexpected_error @ SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unreachable_code(format_args!("Unexpected error code from accept4 completion of {}", unexpected_error)),
			
			_ => unexpected_result_in_completion!(self, accept4),
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
		use self::SocketConnectError::*;
		
		match self.0
		{
			0 => Ok(Some(())),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeENOMEM => Err(OutOfKernelMemory),
			SystemCallErrorNumber::NegativeEACCES | SystemCallErrorNumber::NegativeEPERM => Err(PermissionDenied),
			SystemCallErrorNumber::NegativeEADDRINUSE => Err(AddressInUse),
			SystemCallErrorNumber::NegativeEAGAIN => Err(NoMoreFreeLocalPorts),
			SystemCallErrorNumber::NegativeECONNREFUSED => Err(ConnectionRefused),
			SystemCallErrorNumber::NegativeEINPROGRESS => Err(InProgress),
			SystemCallErrorNumber::NegativeEINTR => Err(Interrupted),
			SystemCallErrorNumber::NegativeETIMEDOUT => Err(TimedOut),
			SystemCallErrorNumber::NegativeENETUNREACH => Err(NetworkUnreachable),
			SystemCallErrorNumber::NegativeEISCONN => panic!("The socket is already connected."),
			SystemCallErrorNumber::NegativeEALREADY => panic!("The socket is nonblocking and a previous connection attempt has not yet been completed."),
			SystemCallErrorNumber::NegativeEBADF => panic!("`sockfd` is not a valid descriptor"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("already bound, or the `addrlen` is wrong, or the socket was not in the `AF_UNIX` family"),
			SystemCallErrorNumber::NegativeENOTSOCK => panic!("`sockfd` is not a socket file descriptor"),
			SystemCallErrorNumber::NegativeEFAULT => panic!("`addr` points outside the user's accessible address space"),
			SystemCallErrorNumber::NegativeEAFNOSUPPORT => panic!("Invalid `sa_family_t` value"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, connect),
			
			_ => unexpected_result_in_completion!(self, connect),
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
	/// * `ConnectionReset` (seems to be possible in some circumstances for Unix domain sockets).
	/// * `ConnectionRefused` (only can happen for TCP client sockets; can not happen for sockets `accept()`ed by a server listener).
	///
	/// Can not return `Ok(Some(value))` with `value` greater than `i32::MAX as u32`.
	#[inline(always)]
	pub fn receive(self, buffer: &[u8]) -> io::Result<Option<u32>>
	{
		use std::io::ErrorKind::*;
		
		match self.0
		{
			length @ 1 ..= i32::MAX => Self::ok_u32(length),
			
			0 => if buffer.is_empty()
			{
				Ok(Some(0))
			}
			else
			{
				Self::io_error_from_error_kind(UnexpectedEof)
			}
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEAGAIN => Self::io_error_from_error_kind(WouldBlock),
			SystemCallErrorNumber::NegativeEINTR => Self::io_error_from_error_kind(Interrupted),
			SystemCallErrorNumber::NegativeENOMEM => Self::io_error_from_error_kind(Other),
			SystemCallErrorNumber::NegativeECONNRESET => Self::io_error_from_error_kind(ConnectionReset),
			SystemCallErrorNumber::NegativeECONNREFUSED => Self::io_error_from_error_kind(ConnectionRefused),
			SystemCallErrorNumber::NegativeEBADF => panic!("The argument `sockfd` is an invalid descriptor"),
			SystemCallErrorNumber::NegativeEFAULT => panic!("The receive buffer pointer(s) point outside the process's address space"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("Invalid argument passed"),
			SystemCallErrorNumber::NegativeENOTCONN => panic!("The socket is associated with a connection-oriented protocol and has not been connected"),
			SystemCallErrorNumber::NegativeENOTSOCK => panic!("The argument `sockfd` does not refer to a socket"),
			SystemCallErrorNumber::NegativeEOPNOTSUPP => panic!("Some flags in the `flags` argument are inappropriate for the socket type"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, receive),
			
			_ => unexpected_result_in_completion!(self, receive),
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
		use std::io::ErrorKind::*;
		
		match self.0
		{
			length @ 1 ..= i32::MAX => Self::ok_u32(length),
			
			0 => if buffer.is_empty()
			{
				Ok(Some(0))
			}
			else
			{
				Self::io_error_from_error_kind(WriteZero)
			}
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEAGAIN => Self::io_error_from_error_kind(WouldBlock),
			SystemCallErrorNumber::NegativeEINTR => Self::io_error_from_error_kind(Interrupted),
			SystemCallErrorNumber::NegativeENOMEM | SystemCallErrorNumber::NegativeENOBUFS => Self::io_error_from_error_kind(Other),
			SystemCallErrorNumber::NegativeEPIPE => Self::io_error_from_error_kind(BrokenPipe),
			SystemCallErrorNumber::NegativeEACCES => Self::io_error_from_error_kind(PermissionDenied),
			SystemCallErrorNumber::NegativeECONNRESET => Self::io_error_from_error_kind(ConnectionReset),
			SystemCallErrorNumber::NegativeECONNREFUSED => Self::io_error_from_error_kind(ConnectionRefused),
			SystemCallErrorNumber::NegativeEBADF => panic!("The argument `sockfd` is an invalid descriptor"),
			SystemCallErrorNumber::NegativeEFAULT => panic!("The receive buffer pointer(s) point outside the process's address space"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("Invalid argument passed"),
			SystemCallErrorNumber::NegativeENOTCONN => panic!("The socket is associated with a connection-oriented protocol and has not been connected"),
			SystemCallErrorNumber::NegativeENOTSOCK => panic!("The argument `sockfd` does not refer to a socket"),
			SystemCallErrorNumber::NegativeEOPNOTSUPP => panic!("Some flags in the `flags` argument are inappropriate for the socket type"),
			SystemCallErrorNumber::NegativeEMSGSIZE => panic!("The socket type requires that message be sent atomically, and the size of the message to be sent made this impossible"),
			SystemCallErrorNumber::NegativeEISCONN => panic!("The connection-mode socket was connected already but a recipient was specified"),
			SystemCallErrorNumber::NegativeEDESTADDRREQ => panic!("The socket is not connection-mode, and no peer address is set"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, send),
			
			_ => unexpected_result_in_completion!(self, send),
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
		use self::StructReadError::*;
		
		match self.0
		{
			length@ 0 ..= i32::MAX => Self::ok_u32(length),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEAGAIN => Err(WouldBlock),
			SystemCallErrorNumber::NegativeEINTR => Err(Interrupted),
			SystemCallErrorNumber::NegativeEIO | SystemCallErrorNumber::NegativeENOMEM => Err(Cancelled),
			SystemCallErrorNumber::NegativeEBADF | SystemCallErrorNumber::NegativeENOTSOCK => panic!("`fd` is not a valid socket file descriptor"),
			SystemCallErrorNumber::NegativeEFAULT => panic!("`buf` is outside your accessible address space"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("`fd` is attached to an object which is unsuitable for reading OR was created via a call to `timerfd_create()` and the wrong size buffer was given to `read()`"),
			SystemCallErrorNumber::NegativeEISDIR => panic!("`fd` refers to a directory"),
			SystemCallErrorNumber::NegativeENOTCONN => panic!("Using recvmsg() for a connected socket which hasn't been connected"),
			SystemCallErrorNumber::NegativeECONNREFUSED => panic!("Using recvmsg() for a connected socket"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, receive_message),
			
			_ => unexpected_result_in_completion!(self, receive_message),
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
		use std::io::ErrorKind::*;
		
		match self.0
		{
			length @ 0 ..= i32::MAX => Self::ok_u32(length),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEAGAIN => Self::io_error_from_error_kind(WouldBlock),
			SystemCallErrorNumber::NegativeEINTR => Self::io_error_from_error_kind(Interrupted),
			SystemCallErrorNumber::NegativeENOMEM | SystemCallErrorNumber::NegativeENOBUFS => Self::io_error_from_error_kind(Other),
			SystemCallErrorNumber::NegativeEPIPE => Self::io_error_from_error_kind(BrokenPipe),
			SystemCallErrorNumber::NegativeEACCES => Self::io_error_from_error_kind(PermissionDenied),
			SystemCallErrorNumber::NegativeECONNRESET => Self::io_error_from_error_kind(ConnectionReset),
			SystemCallErrorNumber::NegativeEBADF => panic!("The argument `sockfd` is an invalid descriptor"),
			SystemCallErrorNumber::NegativeEFAULT => panic!("The receive buffer pointer(s) point outside the process's address space"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("Invalid argument passed"),
			SystemCallErrorNumber::NegativeENOTCONN => panic!("The socket is associated with a connection-oriented protocol and has not been connected"),
			SystemCallErrorNumber::NegativeENOTSOCK => panic!("The argument `sockfd` does not refer to a socket"),
			SystemCallErrorNumber::NegativeEOPNOTSUPP => panic!("Some flags in the `flags` argument are inappropriate for the socket type"),
			SystemCallErrorNumber::NegativeEMSGSIZE => panic!("The socket type requires that message be sent atomically, and the size of the message to be sent made this impossible"),
			SystemCallErrorNumber::NegativeEISCONN => panic!("The connection-mode socket was connected already but a recipient was specified"),
			SystemCallErrorNumber::NegativeEDESTADDRREQ => panic!("The socket is not connection-mode, and no peer address is set"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, send_message),
			
			_ => unexpected_result_in_completion!(self, send_message),
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
		use self::EPollAddError::*;
		
		match self.0
		{
			0 => Ok(Some(())),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN => Err(TryAgain),
			SystemCallErrorNumber::NegativeENOMEM => Err(ThereWasInsufficientKernelMemory),
			SystemCallErrorNumber::NegativeENOSPC => Err(LimitOnWatchesWouldBeExceeded),
			SystemCallErrorNumber::NegativeEBADF => panic!("The supplied file descriptor was not a valid file descriptor"),
			SystemCallErrorNumber::NegativeEEXIST => panic!("The supplied file descriptor was already registered with this epoll instance"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("Can not add epoll file descriptor to its self, or can not make wait on an epoll file descriptor `EPOLLEXCLUSIVE`"),
			SystemCallErrorNumber::NegativeELOOP => panic!("The supplied file descriptor is for an epoll instance and this operation would result in a circular loop of epoll instances monitoring one another"),
			SystemCallErrorNumber::NegativeEPERM => panic!("The supplied file descriptor does not support epoll (perhaps it is an open regular file or the like)"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, epoll_control_add),
			
			_ => unexpected_result_in_completion!(self, epoll_control_add),
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
		use self::EPollModifyError::*;
		
		match self.0
		{
			0 => Ok(Some(())),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN => Err(TryAgain),
			SystemCallErrorNumber::NegativeENOMEM => Err(ThereWasInsufficientKernelMemory),
			SystemCallErrorNumber::NegativeEBADF => panic!("The supplied file descriptor was not a valid file descriptor"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("Supplied file descriptor was not usable or there was the presence or absence of `Exclusive` when required"),
			SystemCallErrorNumber::NegativeENOENT => panic!("The supplied file descriptor is not registered with this epoll instance"),
			SystemCallErrorNumber::NegativeEPERM => panic!("The supplied file descriptor does not support epoll (perhaps it is an open regular file or the like)"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, epoll_control_modify),
			
			_ => unexpected_result_in_completion!(self, epoll_control_modify),
		}
	}
	
	/// Returns `Ok(None)` if cancelled.
	///
	/// Returns `Err(())` if the interrupted or out of kernel memory.
	#[inline(always)]
	pub fn epoll_control_delete(self) -> Result<Option<()>, ()>
	{
		match self.0
		{
			0 => Ok(Some(())),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN => Err(()),
			SystemCallErrorNumber::NegativeENOMEM => panic!("Examination of the Linux source code fs/eventpoll.c suggests `ENOMEM` should not occur for `EPOLL_CTL_DEL`"),
			SystemCallErrorNumber::NegativeEBADF => panic!("The supplied file descriptor was not a valid file descriptor"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("Supplied file descriptor was not usable"),
			SystemCallErrorNumber::NegativeENOENT => panic!("The supplied file descriptor is not registered with this epoll instance"),
			SystemCallErrorNumber::NegativeEPERM => panic!("The supplied file descriptor does not support epoll (perhaps it is an open regular file or the like)"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, epoll_control_delete),
			
			_ => unexpected_result_in_completion!(self, epoll_control_delete),
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
			0 => Ok(Some(())),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEALREADY => Err(true),
			SystemCallErrorNumber::NegativeENOENT => Err(false),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, cancel),
			
			_ => unexpected_result_in_completion!(self, cancel),
		}
	}
	
	/// Returns `Ok(None)` if cancelled.
	#[inline(always)]
	pub fn nop(self) -> Option<()>
	{
		match self.0
		{
			0 => Some(()),
			
			SystemCallErrorNumber::NegativeECANCELED => None,
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, nop),
			
			_ => unexpected_result_in_completion!(self, nop),
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
			0 => Ok(Some(true)),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeETIME => Ok(Some(false)),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => Err(()),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, timeout),
			
			_ => unexpected_result_in_completion!(self, timeout),
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
			0 => Some(true),
			
			SystemCallErrorNumber::NegativeECANCELED => None,
			SystemCallErrorNumber::NegativeETIME => Some(false),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, linked_timeout),
			
			_ => unexpected_result_in_completion!(self, linked_timeout),
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
			0 => Ok(Some(())),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEALREADY => Err(Some(true)),
			SystemCallErrorNumber::NegativeENOENT => Err(Some(false)),
			SystemCallErrorNumber::NegativeEBUSY => Err(None),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, cancel_timeout),
			
			_ => unexpected_result_in_completion!(self, cancel_timeout),
		}
	}
	
	/// Returns `Ok(None)` if cancelled (not sure this is possible).
	/// Returns an `Err(true)` if interrupted by a signal or file is currently being executed.
	/// Returns an `Err(false)` if out-of-disk space.
	#[inline(always)]
	pub fn file_allocate(self) -> Result<Option<()>, bool>
	{
		match self.0
		{
			0 => Ok(Some(())),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => Err(true),
			SystemCallErrorNumber::NegativeENOSPC => Err(false),
			SystemCallErrorNumber::NegativeETXTBSY => Err(true), // `mode` specifies `FALLOC_FL_COLLAPSE_RANGE` or `FALLOC_FL_INSERT_RANGE`, but the file referred to by fd is currently being executed.
			SystemCallErrorNumber::NegativeEBADF => panic!("fd is not a valid file descriptor, or is not opened for writing"),
			SystemCallErrorNumber::NegativeEFBIG => panic!("`offset + length` exceeds the maximum file size"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("offset was less than 0, or len was less than or equal to 0, or the underlying filesystem does not support the operation"),
			SystemCallErrorNumber::NegativeENODEV => panic!("fd does not refer to a regular file"),
			SystemCallErrorNumber::NegativeESPIPE => panic!("fd refers to a pipe"),
			SystemCallErrorNumber::NegativeENOSYS => panic!("This kernel does not implement fallocate()"),
			SystemCallErrorNumber::NegativeEOPNOTSUPP => panic!("The filesystem containing the file referred to by fd does not support this operation; or the mode is not supported by the filesystem containing the file referred to by fd"),
			SystemCallErrorNumber::NegativeEPERM => panic!("The file referred to by fd is marked immutable (see chattr(1)). Or mode specifies FALLOC_FL_PUNCH_HOLE or FALLOC_FL_COLLAPSE_RANGE or FALLOC_FL_INSERT_RANGE and the file referred to by fd is marked append-only (see chattr(1)). Or the operation was prevented by a file seal; see fcntl(2)."),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, file_allocate),
			
			_ => unexpected_result_in_completion!(self, file_allocate),
		}
	}
	
	/// Returns `None` if cancelled.
	/// Doesn't return errors that can be usefully handled.
	#[inline(always)]
	pub fn file_advise(self) -> Option<()>
	{
		match self.0
		{
			0 => Some(()),
			
			SystemCallErrorNumber::NegativeECANCELED => None,
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("An invalid value was specified for advice"),
			SystemCallErrorNumber::NegativeEBADF => panic!("fd is not a valid file descriptor"),
			SystemCallErrorNumber::NegativeESPIPE => panic!("The specified file descriptor refers to a pipe or FIFO (before Linux 2.6.16, this was EINVAL)"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, file_advise),
			
			_ => unexpected_result_in_completion!(self, file_advise),
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
			0 => Ok(Some(())),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEIO => Err(true),
			SystemCallErrorNumber::NegativeENOMEM | SystemCallErrorNumber::NegativeENOSPC => Err(false),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN => panic!("EINTR / EAGAIN - are these possible?"),
			SystemCallErrorNumber::NegativeEBADF => panic!("fd is not a valid file descriptor"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("flags specifies an invalid bit; or offset or n bytes is invalid"),
			SystemCallErrorNumber::NegativeESPIPE => panic!("fd refers to something other than a regular file, a block device, a directory, or a symbolic link"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, synchronize_file_range),
			
			_ => unexpected_result_in_completion!(self, synchronize_file_range),
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
			0 => Ok(Some(())),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM => panic!("EINTR / EAGAIN / ENOMEM - are these possible?"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => self.as_io_error(),
			
			_ => unexpected_result_in_completion!(self, memory_advise),
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
			0 => Some(true),
			
			SystemCallErrorNumber::NegativeECANCELED => None,
			SystemCallErrorNumber::NegativeEIO | SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN | SystemCallErrorNumber::NegativeENOMEM | SystemCallErrorNumber::NegativeENOSPC => Some(false),
			SystemCallErrorNumber::NegativeEBADF => panic!("fd isn't a valid open file descriptor."),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, close),
			
			_ => unexpected_result_in_completion!(self, close),
		}
	}
	
	/// Returns `Ok(None)` if cancelled.
	#[deprecated(since = "0.0.0", note = "Only use this if previously used the deprecated SubmissionQueueEntry::prepare_openat()")]
	#[inline(always)]
	pub fn openat<Open: OnDiskFileDescriptor>(self) -> io::Result<Option<Open>>
	{
		match self.0
		{
			raw_file_descriptor @ 0 ..= i32::MAX => Self::raw_file_descriptor_ok(raw_file_descriptor),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => self.as_io_error(),
			
			_ => unexpected_result_in_completion!(self, openat),
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
			raw_file_descriptor @ 0 ..= i32::MAX => Self::raw_file_descriptor_ok(raw_file_descriptor),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => self.as_io_error(),
			
			_ => unexpected_result_in_completion!(self, openat2),
		}
	}
	
	/// Extended metadata.
	#[inline(always)]
	pub fn extended_metadata_for_path(self, _path: &CStr) -> io::Result<Option<()>>
	{
		match self.0
		{
			0 => Ok(Some(())),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEACCES | SystemCallErrorNumber::NegativeELOOP | SystemCallErrorNumber::NegativeENAMETOOLONG | SystemCallErrorNumber::NegativeENOMEM | SystemCallErrorNumber::NegativeENOTDIR | SystemCallErrorNumber::NegativeENOENT => self.as_io_error(),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN => panic!("EINTR / EAGAIN - are these possible?"),
			SystemCallErrorNumber::NegativeEBADF => panic!("dirfd is not a valid open file descriptor"),
			SystemCallErrorNumber::NegativeEFAULT => panic!("pathname or statxbuf is NULL or points to a location outside the process's accessible address space"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("Invalid flag specified in flags. Or, reserved flag specified in mask. (Currently, there is one such flag, designated by the constant STATX__RESERVED, with the value 0x80000000U)."),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, extended_metadata_for_path),
			
			_ => unexpected_result_in_completion!(self, extended_metadata_for_path),
		}
	}
	
	/// Extended metadata.
	#[inline(always)]
	pub fn extended_metadata_for_directory(self) -> io::Result<Option<()>>
	{
		match self.0
		{
			0 => Ok(Some(())),
			
			SystemCallErrorNumber::NegativeECANCELED => Ok(None),
			SystemCallErrorNumber::NegativeEACCES | SystemCallErrorNumber::NegativeELOOP | SystemCallErrorNumber::NegativeENAMETOOLONG | SystemCallErrorNumber::NegativeENOMEM => self.as_io_error(),
			SystemCallErrorNumber::NegativeEINTR | SystemCallErrorNumber::NegativeEAGAIN => panic!("EINTR / EAGAIN - are these possible?"),
			SystemCallErrorNumber::NegativeEBADF => panic!("dirfd is not a valid open file descriptor"),
			SystemCallErrorNumber::NegativeEFAULT => panic!("pathname or statxbuf is NULL or points to a location outside the process's accessible address space"),
			SystemCallErrorNumber::NegativeEINVAL => panic!("Invalid flag specified in flags. Or, reserved flag specified in mask. (Currently, there is one such flag, designated by the constant STATX__RESERVED, with the value 0x80000000U)."),
			SystemCallErrorNumber::NegativeENOENT => panic!("ENOENT A component of pathname does not exist, or pathname is an empty string and AT_EMPTY_PATH was not specified in flags."),
			SystemCallErrorNumber::NegativeENOTDIR => panic!("A component of the path prefix of pathname is not a directory or pathname is relative and dirfd is a file descriptor referring to a file other than a directory"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, extended_metadata_for_directory),
			
			_ => unexpected_result_in_completion!(self, extended_metadata_for_directory),
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
			count @ 0 ..= i32::MAX =>
			{
				debug_assert!(count <= replace_with_files_descriptors.len() as i32);
				Some(count as u32)
			}
			
			SystemCallErrorNumber::NegativeECANCELED => None,
			SystemCallErrorNumber::NegativeEINVAL => panic!("Unsupported"),
			SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error_in_completion!(self, registered_file_descriptors_update),
			
			_ => unexpected_result_in_completion!(self, registered_file_descriptors_update),
		}
	}
	
	#[inline(always)]
	const fn ok_u32<E>(ok: i32) -> Result<Option<u32>, E>
	{
		if cfg!(debug_assertions)
		{
			if ok < 0
			{
				panic!("Not a positive value")
			}
		}
		
		Ok(Some(ok as u32))
	}
	
	#[inline(always)]
	fn as_io_error<X>(self) -> io::Result<X>
	{
		Err(SystemCallErrorNumber::from_negative_i32_unchecked(self.0).into())
	}
	
	#[inline(always)]
	fn io_error_from_error_kind<X>(kind: ErrorKind) -> io::Result<X>
	{
		Err(io::Error::from(kind))
	}
	
	#[inline(always)]
	const fn raw_file_descriptor_ok<Open: OnDiskFileDescriptor>(raw_file_descriptor: i32) -> io::Result<Option<Open>>
	{
		if cfg!(debug_assertions)
		{
			if raw_file_descriptor < 0
			{
				panic!("Not a raw file descriptor")
			}
		}
		
		Ok(Some(unsafe { Open::from_raw_fd(raw_file_descriptor) }))
	}
}
