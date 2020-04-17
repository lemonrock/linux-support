// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a POSIX message queue instance for sending messages.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendPosixMessageQueueFileDescriptor(PosixMessageQueueFileDescriptor);

impl AsRawFd for SendPosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0.as_raw_fd()
	}
}

impl IntoRawFd for SendPosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0.into_raw_fd()
	}
}

impl FromRawFd for SendPosixMessageQueueFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(PosixMessageQueueFileDescriptor::from_raw_fd(fd))
	}
}

impl FileDescriptor for SendPosixMessageQueueFileDescriptor
{
}

impl PosixMessageQueue for SendPosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn new(name: &CStr, open_or_create: &OpenOrCreatePosixMessageQueue) -> Result<(Self, PosixMessageQueueConstraints), CreationError>
	{
		PosixMessageQueueFileDescriptor::new(name, PosixMessageQueueCreateSendOrReceive::Send, open_or_create).map(|(message_queue_file_descriptor, posix_message_queue_constraints)| (Self(message_queue_file_descriptor), posix_message_queue_constraints))
	}

	#[inline(always)]
	fn queue_depth(&self) -> usize
	{
		self.0.queue_depth()
	}
}

impl Send for SendPosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn send(&self, message_buffer: &[u8], message_priority: PosixMessagePriority) -> Result<(), StructWriteError>
	{
		self.0.send(message_buffer, message_priority)
	}
}
