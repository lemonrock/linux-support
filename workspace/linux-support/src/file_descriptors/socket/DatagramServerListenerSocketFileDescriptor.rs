// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a datagram socket instance server listener.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DatagramServerListenerSocketFileDescriptor<SD: SocketData>(SocketFileDescriptor<SD>);

impl<SD: SocketData> AsRawFd for DatagramServerListenerSocketFileDescriptor<SD>
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0.as_raw_fd()
	}
}

impl<SD: SocketData> IntoRawFd for DatagramServerListenerSocketFileDescriptor<SD>
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0.into_raw_fd()
	}
}

impl<SD: SocketData> FromRawFd for DatagramServerListenerSocketFileDescriptor<SD>
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(SocketFileDescriptor::from_raw_fd(fd))
	}
}

impl<SD: SocketData> FileDescriptor for DatagramServerListenerSocketFileDescriptor<SD>
{
}

impl<SD: SocketData> ListenerSocketFileDescriptor for DatagramServerListenerSocketFileDescriptor<SD>
{
}

impl<SD: SocketData> Deref for DatagramServerListenerSocketFileDescriptor<SD>
{
	type Target = SocketFileDescriptor<SD>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DatagramServerListenerSocketFileDescriptor<sockaddr_un>
{
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
	/// Useful for complex scenarios where a priveleged (eg root) process wants to use different credentials to those it would default to.
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
