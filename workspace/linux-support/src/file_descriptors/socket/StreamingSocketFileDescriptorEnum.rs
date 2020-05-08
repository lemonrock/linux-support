// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a streaming socket instance between two peers.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StreamingSocketFileDescriptorEnum
{
	/// An Internet Protocol (IP) version 4 streaming socket.
	InternetProtocolVersion4(StreamingSocketInternetProtocolVersion4FileDescriptor),

	/// An Internet Protocol (IP) version 6 streaming socket.
	InternetProtocolVersion6(StreamingSocketInternetProtocolVersion6FileDescriptor),

	/// An Unix Domain streaming socket.
	UnixDomain(StreamingSocketUnixDomainFileDescriptor),
}

impl AsRawFd for StreamingSocketFileDescriptorEnum
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		use self::StreamingSocketFileDescriptorEnum::*;

		match self
		{
			&InternetProtocolVersion4(ref streaming_socket_file_descriptor) => streaming_socket_file_descriptor.as_raw_fd(),
			&InternetProtocolVersion6(ref streaming_socket_file_descriptor) => streaming_socket_file_descriptor.as_raw_fd(),
			&UnixDomain(ref streaming_socket_file_descriptor) => streaming_socket_file_descriptor.as_raw_fd(),
		}
	}
}

impl IntoRawFd for StreamingSocketFileDescriptorEnum
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		use self::StreamingSocketFileDescriptorEnum::*;

		match self
		{
			InternetProtocolVersion4(streaming_socket_file_descriptor) => streaming_socket_file_descriptor.into_raw_fd(),
			InternetProtocolVersion6(streaming_socket_file_descriptor) => streaming_socket_file_descriptor.into_raw_fd(),
			UnixDomain(streaming_socket_file_descriptor) => streaming_socket_file_descriptor.into_raw_fd(),
		}
	}
}

impl FileDescriptor for StreamingSocketFileDescriptorEnum
{
}

impl SocketConnect for StreamingSocketFileDescriptorEnum
{
}

impl NonServerSocket for StreamingSocketFileDescriptorEnum
{
}

impl FromRawFd for StreamingSocketFileDescriptorEnum
{
	#[inline(always)]
	unsafe fn from_raw_fd(socket_file_descriptor: RawFd) -> Self
	{
		use self::StreamingSocketFileDescriptorEnum::*;
		from_raw_socket_file_descriptor
		(
			socket_file_descriptor,
			|socket_file_descriptor| InternetProtocolVersion4(StreamingSocketFileDescriptor(socket_file_descriptor)),
			|socket_file_descriptor| InternetProtocolVersion6(StreamingSocketFileDescriptor(socket_file_descriptor)),
			|socket_file_descriptor| UnixDomain(StreamingSocketFileDescriptor(socket_file_descriptor))
		)
	}
}

impl Read for StreamingSocketFileDescriptorEnum
{
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `UnexpectedEof`
	/// * `WouldBlock`
	/// * `Interrupted`
	/// * `Other` (which is for when the kernel reports `ENOMEM`, ie it is out of memory).
	/// * `ConnectionReset` (seems to be posible in some circumstances for Unix domain sockets).
	/// * `ConnectionRefused` (only can happen for TCP client sockets; can not happen for sockets `accept()`ed by a server listener).
	#[inline(always)]
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
	{
		use self::StreamingSocketFileDescriptorEnum::*;

		match self
		{
			&mut InternetProtocolVersion4(ref mut streaming_socket_file_descriptor) => streaming_socket_file_descriptor.read(buf),
			&mut InternetProtocolVersion6(ref mut streaming_socket_file_descriptor) => streaming_socket_file_descriptor.read(buf),
			&mut UnixDomain(ref mut streaming_socket_file_descriptor) => streaming_socket_file_descriptor.read(buf),
		}
	}

	#[inline(always)]
	fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>
	{
		use self::StreamingSocketFileDescriptorEnum::*;

		match self
		{
			&mut InternetProtocolVersion4(ref mut streaming_socket_file_descriptor) => streaming_socket_file_descriptor.read_vectored(bufs),
			&mut InternetProtocolVersion6(ref mut streaming_socket_file_descriptor) => streaming_socket_file_descriptor.read_vectored(bufs),
			&mut UnixDomain(ref mut streaming_socket_file_descriptor) => streaming_socket_file_descriptor.read_vectored(bufs),
		}
	}

	#[inline(always)]
	unsafe fn initializer(&self) -> Initializer
	{
		Initializer::nop()
	}
}

impl Write for StreamingSocketFileDescriptorEnum
{
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `WriteZero`
	/// * `WouldBlock`
	/// * `Interrupted`
	/// * `Other` (which is for when the kernel reports `ENOMEM` or `ENOBUFS`, ie it is out of memory).
	/// * `BrokenPipe`
	/// * `PermissionDenied` (only for Unix domain sockets).
	/// * `ConnectionReset`
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize>
	{
		use self::StreamingSocketFileDescriptorEnum::*;

		match self
		{
			&mut InternetProtocolVersion4(ref mut streaming_socket_file_descriptor) => streaming_socket_file_descriptor.write(buf),
			&mut InternetProtocolVersion6(ref mut streaming_socket_file_descriptor) => streaming_socket_file_descriptor.write(buf),
			&mut UnixDomain(ref mut streaming_socket_file_descriptor) => streaming_socket_file_descriptor.write(buf),
		}
	}

	#[inline(always)]
	fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize>
	{
		use self::StreamingSocketFileDescriptorEnum::*;

		match self
		{
			&mut InternetProtocolVersion4(ref mut streaming_socket_file_descriptor) => streaming_socket_file_descriptor.write_vectored(bufs),
			&mut InternetProtocolVersion6(ref mut streaming_socket_file_descriptor) => streaming_socket_file_descriptor.write_vectored(bufs),
			&mut UnixDomain(ref mut streaming_socket_file_descriptor) => streaming_socket_file_descriptor.write_vectored(bufs),
		}
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()>
	{
		Ok(())
	}
}

impl StreamingSocketFileDescriptorEnum
{
	/// Sends a TCP `FIN` for TCP sockets.
	///
	/// Should be done prior to drop(), ideally.
	#[inline(always)]
	pub fn shutdown(&self)
	{
		use self::StreamingSocketFileDescriptorEnum::*;

		match self
		{
			&InternetProtocolVersion4(ref streaming_socket_file_descriptor) => streaming_socket_file_descriptor.shutdown(),
			&InternetProtocolVersion6(ref streaming_socket_file_descriptor) => streaming_socket_file_descriptor.shutdown(),
			&UnixDomain(ref streaming_socket_file_descriptor) => streaming_socket_file_descriptor.shutdown(),
		}
	}
}
