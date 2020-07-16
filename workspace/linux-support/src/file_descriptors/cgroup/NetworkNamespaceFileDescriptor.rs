// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a Network Namespace (netns) file descriptor which is backed by a `File` opened with a path such as `/proc/self/ns/net`.
#[derive(Debug)]
pub struct NetworkNamespaceFileDescriptor(RawFd);

impl From<File> for NetworkNamespaceFileDescriptor
{
	#[inline(always)]
	fn from(value: File) -> Self
	{
		unsafe { Self::from_raw_fd(value.into_raw_fd()) }
	}
}

impl Into<File> for NetworkNamespaceFileDescriptor
{
	#[inline(always)]
	fn into(self) -> File
	{
		unsafe { File::from_raw_fd(self.into_raw_fd()) }
	}
}

impl Drop for NetworkNamespaceFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for NetworkNamespaceFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for NetworkNamespaceFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for NetworkNamespaceFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for NetworkNamespaceFileDescriptor
{
}

impl ExtendedBpfProgramCanBeAttachedFileDescriptor for NetworkNamespaceFileDescriptor
{
	type ProgramAttachmentType = NetworkNamespaceAttachmentType;
	
	type ProgramQueryFlags = ();
	
	type ProgramAttachmentFlags = ();
	
	type ProgramAttachmentOptions = ();
	
	// This is the maximum allowed.
	const InitialProgramCountGuess: usize = 1;
}
