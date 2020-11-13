// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a server listener socket instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct StreamingServerListenerSocketFileDescriptor<SD: SocketData>(SocketFileDescriptor<SD>);

impl<SD: SocketData> AsRawFd for StreamingServerListenerSocketFileDescriptor<SD>
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0.as_raw_fd()
	}
}

impl<SD: SocketData> IntoRawFd for StreamingServerListenerSocketFileDescriptor<SD>
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0.into_raw_fd()
	}
}

impl<SD: SocketData> FromRawFd for StreamingServerListenerSocketFileDescriptor<SD>
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(SocketFileDescriptor::from_raw_fd(fd))
	}
}

impl<SD: SocketData> FileDescriptor for StreamingServerListenerSocketFileDescriptor<SD>
{
}

impl<SD: SocketData> ListenerSocketFileDescriptor for StreamingServerListenerSocketFileDescriptor<SD>
{
}

impl<SD: SocketData> Deref for StreamingServerListenerSocketFileDescriptor<SD>
{
	type Target = SocketFileDescriptor<SD>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<SD: SocketData> StreamingServerListenerSocketFileDescriptor<SD>
{
	/// Accepts any pending connections.
	#[inline(always)]
	pub fn accept(&self) -> Result<AcceptedConnection<SD>, SocketAcceptError>
	{
		use self::SocketAcceptError::*;
		use self::ConnectionFailedReason::*;

		let mut peer_address: SD = unsafe_uninitialized();
		let mut peer_address_length = PendingAcceptConnection::<SD>::SocketDataLength();

		let result = unsafe { accept4(self.as_raw_fd(), &mut peer_address as *mut _ as *mut _, &mut peer_address_length, SOCK_NONBLOCK | SOCK_CLOEXEC) };

		if likely!(result == 0)
		{
			debug_assert_eq!(peer_address_length, PendingAcceptConnection::<SD>::SocketDataLength(), "peer_address was truncated");

			Ok
			(
				AcceptedConnection
				{
					streaming_socket_file_descriptor: StreamingSocketFileDescriptor(SocketFileDescriptor(result, PhantomData)),
					peer: SocketDataWithLength
					{
						address: peer_address,
						address_length: peer_address_length as usize,
					},
				}
			)
		}
		else if likely!(result == -1)
		{
			Err
			(
				match errno().0
				{
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

					_ => unreachable_code(format_args!("")),
				}
			)
		}
		else
		{
			unreachable_code(format_args!(""))
		}
	}
}
