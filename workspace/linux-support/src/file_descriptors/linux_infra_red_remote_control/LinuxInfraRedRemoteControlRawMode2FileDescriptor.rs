// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a Linux InfraRed Remote Control (LIRC) raw mode 2 file descriptor which is backed by a device `File` such as `/dev/lircN` where `N` is a number.
#[derive(Debug)]
pub struct LinuxInfraRedRemoteControlRawMode2FileDescriptor(RawFd);

impl From<File> for LinuxInfraRedRemoteControlRawMode2FileDescriptor
{
	#[inline(always)]
	fn from(value: File) -> Self
	{
		unsafe { Self::from_raw_fd(value.into_raw_fd()) }
	}
}

impl Into<File> for LinuxInfraRedRemoteControlRawMode2FileDescriptor
{
	#[inline(always)]
	fn into(self) -> File
	{
		unsafe { File::from_raw_fd(self.into_raw_fd()) }
	}
}

impl Drop for LinuxInfraRedRemoteControlRawMode2FileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for LinuxInfraRedRemoteControlRawMode2FileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for LinuxInfraRedRemoteControlRawMode2FileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for LinuxInfraRedRemoteControlRawMode2FileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for LinuxInfraRedRemoteControlRawMode2FileDescriptor
{
}

impl UsedAsValueInArrayMapDescriptor for LinuxInfraRedRemoteControlRawMode2FileDescriptor
{
	#[inline(always)]
	fn transmute_to_file_descriptor_copies(values: Vec<RawFd>) -> Vec<FileDescriptorCopy<Self>>
	{
		unsafe { transmute(values) }
	}
	
	#[inline(always)]
	fn transmute_from_file_descriptor_copies(values: &[FileDescriptorCopy<Self>]) -> &[RawFd]
	{
		unsafe { transmute(values) }
	}
	
	#[inline(always)]
	fn transmute_to_file_descriptor_copy(value: RawFd) -> FileDescriptorCopy<Self>
	{
		unsafe { transmute(value) }
	}
}

impl ExtendedBpfProgramCanBeAttachedFileDescriptor for LinuxInfraRedRemoteControlRawMode2FileDescriptor
{
	type ProgramAttachmentType = LinuxInfraRedRemoteControlRawMode2ProgramAttachmentType;
	
	type ProgramQueryFlags = ();
	
	type ProgramAttachmentFlags = ();
	
	type ProgramAttachmentOptions = ();
	
	// This is the maximum allowed.
	const InitialProgramCountGuess: usize = 64;
}
