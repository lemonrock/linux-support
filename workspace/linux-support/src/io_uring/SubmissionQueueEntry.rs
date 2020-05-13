// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Submission queue entry, `SQE`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubmissionQueueEntry
{
	pointer: NonNull<io_uring_sqe>,
	#[cfg(debug_assertions)] using_kernel_submission_queue_poll: bool,
	#[cfg(debug_assertions)] using_io_poll: bool,
}

impl SubmissionQueueEntry
{
	#[cfg(debug_assertions)]
	fn using_kernel_submission_queue_poll(&self) -> bool
	{
		self.using_kernel_submission_queue_poll
	}
	
	#[cfg(not(debug_assertions))]
	const fn using_kernel_submission_queue_poll(&self) -> bool
	{
		false
	}
	
	/// Can not return `Ok(value)` with `value` greater than `i32::MAX as u32`, thus `maximum_number_of_bytes_to_transfer` can not exceed `i32::MAX as u32`.
	///
	/// if `file_descriptor_in` is a pipe (or ?character device), `offset_in` is 0.
	#[inline(always)]
	pub fn prepare_splice<'a>(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, io_priority: CompressedIoPriority, file_descriptor_in: Either<SpliceWithOffset<'a, impl Seek + SpliceSender>, SpliceWithoutOffset<'a, impl PipeLikeFileDescriptor + SpliceSender>>, file_descriptor_out: Either<SpliceWithOffset<'a, impl Seek + SpliceRecipient>, SpliceWithoutOffset<'a, impl PipeLikeFileDescriptor + SpliceRecipient>>, maximum_number_of_bytes_to_transfer: u32, splice_flags: SpliceFlags)
	{
		const NoOffset: u64 = -1i64 as u64;
		
		use self::Either::*;
		
		debug_assert!(maximum_number_of_bytes_to_transfer <= i32::MAX as u32);
		
		let ((splice_fd_in, splice_flags), offset_in) = match file_descriptor_in
		{
			Left(SpliceWithOffset { file_descriptor_origin, offset }) =>
			{
				debug_assert_ne!(offset, NoOffset);
				(file_descriptor_origin.into_raw_splice_flags(splice_flags, self.using_kernel_submission_queue_poll()), offset)
			}
			
			Right(SpliceWithoutOffset { file_descriptor_origin }) => (file_descriptor_origin.into_raw_splice_flags(splice_flags, self.using_kernel_submission_queue_poll()), NoOffset),
		};
		
		let (file_descriptor_out, offset_out) = match file_descriptor_out
		{
			Left(SpliceWithOffset { file_descriptor_origin, offset }) =>
			{
				debug_assert_ne!(offset, NoOffset);
				(file_descriptor_origin.into_raw(self.using_kernel_submission_queue_poll), offset)
			}
			
			Right(SpliceWithoutOffset { file_descriptor_origin }) => (file_descriptor_origin.into_raw(self.using_kernel_submission_queue_poll), NoOffset),
		};
		
		self.prepare(user_data, options, personality, io_priority, IORING_OP_SPLICE, file_descriptor_out, Self::Null, maximum_number_of_bytes_to_transfer, offset_out);
		self.anonymous_2().splice_off_in = offset_in;
		self.anonymous_3().splice_flags = splice_flags;
		self.anonymous_4().anonymous_1.splice_fd_in = splice_fd_in
	}
	
	/// Caller must hold onto, and not move, `buffers` until completion.
	///
	/// Equivalent to `preadv2()`.
	#[inline(always)]
	pub fn prepare_read_vectored(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, io_priority: CompressedIoPriority, file_descriptor: FileDescriptorOrigin<impl SeekableFileDescriptor>, buffers: &[&mut [u8]], offset: ReadOrWriteOffset, read_vectored_flags: ReadVectoredFlags)
	{
		let length = buffers.len();
		debug_assert!(length <= u32::MAX as usize);
		
		self.prepare(user_data, options, personality, io_priority, IORING_OP_READV, file_descriptor, Self::to_u64_buffers_mut(buffers), length as u32, offset.into());
		self.anonymous_3().read_vectored_flags = read_vectored_flags
	}
	
	/// Caller must hold onto, and not move, `buffer` until completion.
	///
	/// Uses a registered buffer.
	#[inline(always)]
	pub fn prepare_read_fixed(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, io_priority: CompressedIoPriority, file_descriptor: FileDescriptorOrigin<impl SeekableFileDescriptor>, buffer: &mut [u8], offset: ReadOrWriteOffset, registered_buffer_index: RegisteredBufferIndex)
	{
		let length = buffer.len();
		debug_assert!(length <= u32::MAX as usize);
		
		self.prepare(user_data, options, personality, io_priority, IORING_OP_READ_FIXED, file_descriptor, Self::to_u64_buffer_mut(buffer), length as u32, offset.into());
		self.zero_rw_flags();
		self.set_registered_buffer_index(registered_buffer_index)
	}
	
	/// Caller must hold onto, and not move, `buffer` until completion.
	///
	/// Equivalent to either `read()` (if offset zero) or `pread()` (if specified).
	#[inline(always)]
	pub fn prepare_read(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, io_priority: CompressedIoPriority, file_descriptor: FileDescriptorOrigin<impl SeekableFileDescriptor>, buffer: &mut [u8], offset: ReadOrWriteOffset)
	{
		let length = buffer.len();
		debug_assert!(length <= u32::MAX as usize);
		
		self.prepare(user_data, options, personality, io_priority, IORING_OP_READ, file_descriptor, Self::to_u64_buffer_mut(buffer), length as u32, offset.into());
		self.zero_rw_flags()
	}
	
	/// Caller must hold onto, and not move, `buffers` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	///
	/// Equivalent to `pwritev2()`.
	#[inline(always)]
	pub fn prepare_write_vectored(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, io_priority: CompressedIoPriority, file_descriptor: FileDescriptorOrigin<impl SeekableFileDescriptor>, buffers: &[&[u8]], offset: ReadOrWriteOffset, write_vectored_flags: WriteVectoredFlags)
	{
		let length = buffers.len();
		debug_assert!(length <= u32::MAX as usize);
		
		self.prepare(user_data, options, personality, io_priority, IORING_OP_WRITEV, file_descriptor, Self::to_u64_buffers(buffers), length as u32, offset.into());
		self.zero_rw_flags();
		self.anonymous_3().write_vectored_flags = write_vectored_flags
	}
	
	/// Caller must hold onto, and not move, `buffer` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	#[inline(always)]
	pub fn prepare_write_fixed(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, io_priority: CompressedIoPriority, file_descriptor: FileDescriptorOrigin<impl SeekableFileDescriptor>, buffer: &[u8], offset: ReadOrWriteOffset, registered_buffer_index: RegisteredBufferIndex)
	{
		let length = buffer.len();
		debug_assert!(length <= u32::MAX as usize);
		
		self.prepare(user_data, options, personality, io_priority, IORING_OP_WRITE_FIXED, file_descriptor, Self::to_u64_buffer(buffer), length as u32, offset.into());
		self.zero_rw_flags();
		self.set_registered_buffer_index(registered_buffer_index)
	}
	
	/// Caller must hold onto, and not move, `buffer` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	///
	/// Equivalent to either `write()` (if offset zero) or `pwrite()` (if specified).
	#[inline(always)]
	pub fn prepare_write(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, io_priority: CompressedIoPriority, file_descriptor: FileDescriptorOrigin<impl SeekableFileDescriptor>, buffer: &[u8], offset: u64)
	{
		let length = buffer.len();
		debug_assert!(length <= u32::MAX as usize);
		
		self.prepare(user_data, options, personality, io_priority, IORING_OP_WRITE, file_descriptor, Self::to_u64_buffer(buffer), length as u32, offset);
		self.zero_rw_flags();
	}
	
	/// Not supported if `SetupFlags::IoPoll` was specified during setup.
	///
	/// The `CompletionQueueEntry` (CQE) contains the resultant poll flags in its result (`res`).
	#[inline(always)]
	pub fn prepare_poll_add(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<impl FileDescriptor>, poll_mask: PollRequestFlags)
	{
		self.guard_not_using_io_poll();
		
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_POLL_ADD, file_descriptor, Self::Null, 0, 0);
		self.anonymous_3().poll_events = poll_mask;
		self.zero_buf_index()
	}
	
	/// Not supported if `SetupFlags::IoPoll` was specified during setup.
	#[inline(always)]
	pub fn prepare_poll_cancel(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, cancel_for_user_data: impl UserData)
	{
		self.guard_not_using_io_poll();
		
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_POLL_REMOVE, FileDescriptorOrigin::<File>::Irrelevant, cancel_for_user_data.into_u64(), 0, 0);
		self.zero_rw_flags();
		self.zero_buf_index()
	}
	
	/// Not supported if `SetupFlags::IoPoll` was specified during setup.
	#[inline(always)]
	pub fn prepare_file_synchronize(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<File>, file_synchronize: FileSynchronize)
	{
		self.guard_not_using_io_poll();
		
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_FSYNC, file_descriptor.into(), Self::Null, 0, 0);
		self.anonymous_3().fsync_flags = file_synchronize;
		self.zero_buf_index()
	}

	/// The argument `socket` is a socket that has been created with `socket(2)`, bound to a local address with `bind(2)`, and is listening for connections after a `listen(2)`.
	/// Caller must hold onto, and not move, `pending_accept_connection` until completion.
	///
	/// The `CQE` `res` field will contain either a socket file descriptor or an error code.
	///
	/// `pending_accept_connection` is *NOT* a registered buffer.
	#[inline(always)]
	pub fn prepare_accept<SD: SocketData>(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, socket: FileDescriptorOrigin<impl SocketAccept>, pending_accept_connection: &mut PendingAcceptConnection<SD>)
	{
		let flags = SOCK_NONBLOCK | SOCK_CLOEXEC;
		
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_ACCEPT, socket, Self::to_u64_mut(&mut pending_accept_connection.peer_address), 0, Self::to_u64_mut(&mut pending_accept_connection.peer_address_length));
		self.anonymous_3().accept_flags = flags as u32;
		self.zero_buf_index()
	}

	/// Caller must hold onto, and not move, `peer_address` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	///
	/// `peer_address` is *NOT* a registered buffer.
	#[inline(always)]
	pub fn prepare_connect<SD: SocketData>(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, socket: FileDescriptorOrigin<impl SocketConnect>, peer_address: &SD, peer_address_length: socklen_t)
	{
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_CONNECT, socket, Self::to_u64(peer_address), 0, peer_address_length as u64);
		self.zero_rw_flags();
		self.zero_buf_index()
	}
	
	/// Caller must hold onto, and not move, `buffer` until completion.
	///
	/// Not supported if `SetupFlags::IoPoll` was specified during setup.
	#[inline(always)]
	pub fn prepare_receive(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, io_priority: CompressedIoPriority, socket: FileDescriptorOrigin<impl NonServerSocket>, buffer: &mut [u8], flags: ReceiveFlags)
	{
		self.guard_not_using_io_poll();
		
		let length = buffer.len();
		debug_assert!(length <= u32::MAX as usize);

		self.prepare(user_data, options, personality, io_priority, IORING_OP_RECV, socket, Self::to_u64_buffer_mut(buffer), length as u32, 0);
		self.set_receive_flags(flags)
	}

	/// Caller must hold onto, and not move, `buffer` until completion.
	///
	/// Not supported if `SetupFlags::IoPoll` was specified during setup.
	#[inline(always)]
	pub fn prepare_send(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, io_priority: CompressedIoPriority, socket: FileDescriptorOrigin<impl NonServerSocket>, buffer: &[u8], flags: SendFlags)
	{
		self.guard_not_using_io_poll();
		
		let length = buffer.len();
		debug_assert!(length <= u32::MAX as usize);

		self.prepare(user_data, options, personality, io_priority, IORING_OP_SEND, socket, Self::to_u64_buffer(buffer), length as u32, 0);
		self.set_send_flags(flags)
	}

	/// Caller must hold onto, and not move, `message` until completion.
	///
	/// Not supported if `SetupFlags::IoPoll` was specified during setup.
	#[inline(always)]
	pub fn prepare_receive_message<SD: SocketData>(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, io_priority: CompressedIoPriority, socket: FileDescriptorOrigin<impl NonServerSocket>, message: &mut ReceiveMessage<SD>, flags: ReceiveFlags)
	{
		self.guard_not_using_io_poll();
		
		self.prepare(user_data, options, personality, io_priority, IORING_OP_RECVMSG, socket, Self::to_u64_mut(&mut message.internal), 1, 0);
		self.set_receive_flags(flags)
	}

	/// Caller must hold onto, and not move, `message` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	///
	/// Not supported if `SetupFlags::IoPoll` was specified during setup.
	#[inline(always)]
	pub fn prepare_send_message<SD: SocketData>(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, io_priority: CompressedIoPriority, socket: FileDescriptorOrigin<impl NonServerSocket>, message: &SendMessage<SD>, flags: SendFlags)
	{
		self.guard_not_using_io_poll();
		
		self.prepare(user_data, options, personality, io_priority, IORING_OP_SENDMSG, socket, Self::to_u64(&message.internal), 1, 0);
		self.set_send_flags(flags)
	}

	/// Caller must hold onto, and not move, `epoll_event` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	///
	/// `epoll_event` is *not* a buffer that can be registered.
	#[inline(always)]
	pub fn prepare_epoll_control_add(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, epoll_file_descriptor: FileDescriptorOrigin<EPollFileDescriptor>, file_descriptor: &impl FileDescriptor, flags: EPollAddFlags, token: u64, epoll_event: &mut epoll_event)
	{
		epoll_event.events = flags.bits();
		epoll_event.data = epoll_data_t
		{
			u64: token,
		};

		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_EPOLL_CTL, epoll_file_descriptor, Self::to_u64_mut(epoll_event), EPOLL_CTL_ADD as u32, file_descriptor.as_raw_fd() as u32 as u64);
		self.zero_rw_flags();
		self.zero_buf_index()
	}

	/// Caller must hold onto, and not move, `epoll_event` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	///
	/// `epoll_event` is *not* a buffer that can be registered.
	#[inline(always)]
	pub fn prepare_epoll_control_modify(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, epoll_file_descriptor: FileDescriptorOrigin<EPollFileDescriptor>, file_descriptor: &impl FileDescriptor, flags: EPollModifyFlags, token: u64, epoll_event: &mut epoll_event)
	{
		epoll_event.events = flags.bits();
		epoll_event.data = epoll_data_t
		{
			u64: token,
		};
		
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_EPOLL_CTL, epoll_file_descriptor, Self::to_u64_mut(epoll_event), EPOLL_CTL_MOD as u32, file_descriptor.as_raw_fd() as u32 as u64);
		self.zero_rw_flags();
		self.zero_buf_index()
	}

	/// EPoll delete.
	#[inline(always)]
	pub fn prepare_epoll_control_delete(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, epoll_file_descriptor: FileDescriptorOrigin<EPollFileDescriptor>, file_descriptor: &impl FileDescriptor)
	{
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_EPOLL_CTL, epoll_file_descriptor, Self::Null, EPOLL_CTL_DEL as u32, file_descriptor.as_raw_fd() as u32 as u64);
		self.zero_rw_flags();
		self.zero_buf_index()
	}
	
	/// Not supported if `SetupFlags::IoPoll` was specified during setup.
	#[inline(always)]
	pub fn prepare_cancel(self, user_data: impl UserData, personality: Option<PersonalityCredentialsIdentifier>, cancel_for_user_data: impl UserData)
	{
		self.guard_not_using_io_poll();
		
		const UnusedFlags: u32 = 0;
		
		self.prepare(user_data, SubmissionQueueEntryOptions::empty(), personality, CompressedIoPriority::Irrelevant, IORING_OP_ASYNC_CANCEL, FileDescriptorOrigin::<File>::Irrelevant, cancel_for_user_data.into_u64(), 0, 0);
		self.anonymous_3().cancel_flags = UnusedFlags
	}
	
	/// Not supported if `SetupFlags::IoPoll` was specified during setup.
	#[inline(always)]
	pub fn prepare_nop(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, io_priority: CompressedIoPriority,)
	{
		self.guard_not_using_io_poll();
		
		self.prepare(user_data, options, personality, io_priority, IORING_OP_NOP, FileDescriptorOrigin::<File>::Irrelevant, Self::Null, 0, 0);
		self.zero_rw_flags()
	}
	
	/// Caller must hold onto, and not move, `timeout` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	///
	/// A timeout will trigger a wakeup event on the completion ring for anyone waiting for events (submit and wait).
	/// A timeout condition is met when either the specified timeout expires, or the specified number of events have completed.
	/// Either condition will trigger the event.
	///
	/// Uses the `CLOCK_MONOTONIC` clock source.
	///
	/// Not supported if `SetupFlags::IoPoll` was specified during setup.
	#[inline(always)]
	pub fn prepare_timeout(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, timeout: &__kernel_timespec, completion_event_count: NonZeroU64, relative_or_absolute: RelativeOrAbsoluteTimeout)
	{
		self.guard_not_using_io_poll();
		
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_TIMEOUT, FileDescriptorOrigin::<File>::Irrelevant, Self::to_u64(timeout), 1, completion_event_count.get());
		self.set_timeout_flags(relative_or_absolute)
	}
	
	/// Caller must hold onto, and not move, `timeout` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	///
	/// The previous submission queue entry should have had the flag `SubmissionQueueEntryOptions::Link` in its `options`.
	///
	/// The timeout specified will cancel the linked submission queue entry, unless the linked submission queue entry completes before the timeout.
	///
	/// Useful for timing-out attempts to connect, read or write data.
	#[inline(always)]
	pub fn prepare_linked_timeout(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, timeout: &__kernel_timespec, relative_or_absolute: RelativeOrAbsoluteTimeout)
	{
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_LINK_TIMEOUT, FileDescriptorOrigin::<File>::Irrelevant, Self::to_u64(timeout), 1, 0);
		self.set_timeout_flags(relative_or_absolute)
	}
	
	/// Not supported if `SetupFlags::IoPoll` was specified during setup.
	#[inline(always)]
	pub fn prepare_timeout_cancel(self, user_data: impl UserData, personality: Option<PersonalityCredentialsIdentifier>, cancel_for_user_data: impl UserData, relative_or_absolute: RelativeOrAbsoluteTimeout)
	{
		self.guard_not_using_io_poll();
		
		self.prepare(user_data, SubmissionQueueEntryOptions::empty(), personality, CompressedIoPriority::Irrelevant, IORING_OP_TIMEOUT_REMOVE, FileDescriptorOrigin::<File>::Irrelevant, cancel_for_user_data.into_u64(), 0, 0);
		self.set_timeout_flags(relative_or_absolute);
		self.zero_buf_index()
	}

	/// File allocate.
	#[inline(always)]
	pub fn prepare_file_allocate(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<File>, mode: AllocationMode, offset: u64, length: u64)
	{
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_FALLOCATE, file_descriptor, length, mode.bits() as u32, offset);
		self.zero_rw_flags();
		self.zero_buf_index()
	}
	
	/// File advise.
	#[inline(always)]
	pub fn prepare_file_advise(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<File>, offset: u64, length: u32, advice: Advice)
	{
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_FADVISE, file_descriptor, Self::Null, length, offset);
		self.set_advice(FileOrMemoryAdvice { file: advice });
		self.zero_buf_index()
	}

	/// Issue the equivalent of a `sync_file_range()` on the file descriptor.
	///
	/// The `fd` field is the file descriptor to sync, the `off` field holds the offset in bytes, the `len` field holds the length in bytes, and the `sync_range_flags` field holds the flags for the command.
	///
	/// See also `sync_file_range(2)` for the general description of the related system call.
	///
	/// Since Linux 5.2.
	///
	/// Not supported if `SetupFlags::IoPoll` was specified during setup.
	#[inline(always)]
	pub fn prepare_synchronize_file_range(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<File>, offset: u64, length: u32, flags: SynchronizeFileRangeFlags)
	{
		self.guard_not_using_io_poll();
		
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_SYNC_FILE_RANGE, file_descriptor, Self::Null, length, offset);
		self.anonymous_3().sync_range_flags = flags;
		self.zero_buf_index()
	}
	
	/// Caller must hold onto, *but may move*, `mapped_memory` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	#[inline(always)]
	pub fn prepare_memory_advise(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, mapped_memory: &MappedMemory, offset: usize, length: u32, advice: MemoryAdvice)
	{
		mapped_memory.guard_range(&(offset .. (length as usize)));
		
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_MADVISE, FileDescriptorOrigin::<File>::Irrelevant, mapped_memory.virtual_address().add(offset).into(), length, 0);
		self.set_advice(FileOrMemoryAdvice { memory: advice  });
		self.zero_buf_index()
	}
	
	/// `file_descriptor` can *NOT* be a registered file descriptor.
	#[inline(always)]
	pub fn prepare_close<FD: FileDescriptor>(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: &FD)
	{
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_CLOSE, FileDescriptorOrigin::<FD>::Absolute(file_descriptor), Self::Null, 0, 0);
		self.zero_rw_flags();
		self.zero_buf_index()
	}
	
	/// Caller must hold onto, and not move, `open_on_disk` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	///
	/// `directory_file_descriptor` can *NOT* be a registered file descriptor.
	/// `open_on_disk` *must not* have any `PathResolution` other than `PathResolution::empty()`.
	#[deprecated(since = "0.0.0", note = "Prefer prepare_openat2()")]
	#[inline(always)]
	pub fn prepare_openat(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, directory_file_descriptor: &DirectoryFileDescriptor, open_on_disk: &OpenOnDisk<impl OnDiskFileDescriptor>)
	{
		debug_assert_eq!(open_on_disk.open_how.resolve, 0, "Path resolution is not supported");
		
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_OPENAT, FileDescriptorOrigin::<DirectoryFileDescriptor>::Absolute(directory_file_descriptor), open_on_disk.path(), open_on_disk.open_how.mode as u32, 0);
		self.anonymous_3().open_flags = open_on_disk.open_how.flags as u32;
		self.zero_buf_index()
	}
	
	/// Caller must hold onto, and not move, `open_on_disk` until completion.
	///
	/// `directory_file_descriptor` can *NOT* be a registered file descriptor.
	#[inline(always)]
	pub fn prepare_openat2(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, directory_file_descriptor: &DirectoryFileDescriptor, open_on_disk: &mut OpenOnDisk<impl OnDiskFileDescriptor>)
	{
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_OPENAT2, FileDescriptorOrigin::<DirectoryFileDescriptor>::Absolute(directory_file_descriptor), open_on_disk.path(), size_of::<open_how>() as u32, Self::to_u64_mut(&mut open_on_disk.open_how));
		self.zero_rw_flags();
		self.zero_buf_index()
	}
	
	/// Caller must hold onto, and not move, `path` and `extended_metadata` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed for `path` (but not `extended_metadata`).
	///
	/// `directory_file_descriptor` can *NOT* be a registered file descriptor.
	/// `directory_file_descriptor` can be `DirectoryFileDescriptor::AlwaysCurrentWorkingDirectory`.
	#[inline(always)]
	pub fn prepare_extended_metadata_for_path(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, directory_file_descriptor: &DirectoryFileDescriptor, extended_metadata: &mut ExtendedMetadata, force_synchronization: Option<bool>, extended_metadata_wanted: ExtendedMetadataWanted, do_not_dereference_path_if_it_is_a_symlink: bool, do_not_automount_basename_of_path: bool, path: &CStr)
	{
		self.prepare_extended_metadata(user_data, options, personality, directory_file_descriptor, extended_metadata, force_synchronization, extended_metadata_wanted, do_not_dereference_path_if_it_is_a_symlink, do_not_automount_basename_of_path, 0, Self::non_empty_path(path));
	}
	
	/// Caller must hold onto, and not move, `extended_metadata` until completion.
	///
	/// `directory_file_descriptor` can *NOT* be a registered file descriptor.
	/// `directory_file_descriptor` can be `DirectoryFileDescriptor::AlwaysCurrentWorkingDirectory`.
	#[inline(always)]
	pub fn prepare_extended_metadata_for_directory(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, directory_file_descriptor: &DirectoryFileDescriptor, extended_metadata: &mut ExtendedMetadata, force_synchronization: Option<bool>, extended_metadata_wanted: ExtendedMetadataWanted, do_not_dereference_path_if_it_is_a_symlink: bool, do_not_automount_basename_of_path: bool)
	{
		self.prepare_extended_metadata(user_data, options, personality, directory_file_descriptor, extended_metadata, force_synchronization, extended_metadata_wanted, do_not_dereference_path_if_it_is_a_symlink, do_not_automount_basename_of_path, AT_EMPTY_PATH, Self::empty_path());
	}
	
	#[inline(always)]
	fn prepare_extended_metadata(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, directory_file_descriptor: &DirectoryFileDescriptor, extended_metadata: &mut ExtendedMetadata, force_synchronization: Option<bool>, extended_metadata_wanted: ExtendedMetadataWanted, do_not_dereference_path_if_it_is_a_symlink: bool, do_not_automount_basename_of_path: bool, flags: i32, path: NonNull<c_char>)
	{
		let mask = extended_metadata_wanted.bits();
		
		let flags = flags | match force_synchronization
		{
			None => AT_STATX_SYNC_AS_STAT,
			Some(true) => AT_STATX_FORCE_SYNC,
			Some(false) => AT_STATX_DONT_SYNC,
		}
		| if unlikely!(do_not_dereference_path_if_it_is_a_symlink)
		{
			AT_SYMLINK_NOFOLLOW
		}
		else
		{
			0
		}
		| if unlikely!(do_not_automount_basename_of_path)
		{
			AT_NO_AUTOMOUNT
		}
		else
		{
			0
		};
	
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_STATX, FileDescriptorOrigin::<DirectoryFileDescriptor>::Absolute(directory_file_descriptor), Self::to_u64_non_null(path), mask, Self::to_u64_mut(extended_metadata));
		self.anonymous_3().statx_flags = flags as u32;
		self.zero_buf_index()
	}
	
	/// Caller must hold onto, and not move, `replace_with_files_descriptors` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	///
	/// `replace_with_files_descriptors` must contain at least one element (it can not be empty).
	/// `replace_with_files_descriptors.len()` must be less than `u32::MAX as usize`.
	#[inline(always)]
	pub(crate) fn prepare_files_update(self, user_data: impl UserData, personality: Option<PersonalityCredentialsIdentifier>, replace_with_files_descriptors: &[SupportedFileDescriptor], starting_from_index_inclusive: u32)
	{
		let length = replace_with_files_descriptors.len();
		debug_assert_ne!(length, 0);
		debug_assert!(length < u32::MAX as usize);
		
		self.prepare(user_data, SubmissionQueueEntryOptions::empty(), personality, CompressedIoPriority::Irrelevant, IORING_OP_FILES_UPDATE, FileDescriptorOrigin::<File>::Irrelevant, replace_with_files_descriptors.as_ptr() as usize as u64, length as u32, starting_from_index_inclusive as u64);
		self.zero_rw_flags()
	}

	/// Give a contiguous array of buffers for read-like operations to the Linux kernel.
	///
	/// Caller must hold onto, and not move, `buffers` until completion.
	///
	/// In effect, give the Linux kernel a block of contiguous memory divided into `number_of_buffers` (any remainder is lost but recoverable when `buffers` is freed; panics though in debug).
	///
	/// Buffers so provided only work for `read()`, `readv()`, `recv()`, and `recvmsg()`.
	/// All bufffers must have the same `length`.
	/// If `number_of_buffers` is not zero, the buffer identifier will be incremented by one for `number_of_buffers - 1`.
	/// `buffer_identifier` can be zero.
	/// `buffer_identifier` is namespaced by `buffer_group_identifier`.
	/// ?`buffer_group_identifier` can be zero?
	#[inline(always)]
	pub(crate) fn prepare_provide_buffers(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, buffers: &mut [u8], number_of_buffers: u32, buffer_group: BufferGroup, registered_buffer_index: RegisteredBufferIndex)
	{
		let number_of_bytes = buffers.len();
		debug_assert_eq!(number_of_bytes % (number_of_buffers as usize), 0);
		let every_buffer_length = number_of_bytes / (number_of_buffers as usize);
		debug_assert!(every_buffer_length <= u32::MAX as usize);

		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_PROVIDE_BUFFERS, FileDescriptorOrigin::<File>::Index(number_of_buffers), Self::to_u64_buffer_mut(buffers), every_buffer_length as u32, registered_buffer_index.0 as u64);
		self.zero_rw_flags();
		self.set_buffer_group(buffer_group)
	}

	/// Take back a contiguous array of buffers given for read-like operations to the Linux kernel.
	#[inline(always)]
	pub(crate) fn prepare_remove_buffers(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, number_of_buffers: NonZeroU32, buffer_group: BufferGroup)
	{
		self.prepare(user_data, options, personality, CompressedIoPriority::Irrelevant, IORING_OP_REMOVE_BUFFERS, FileDescriptorOrigin::<File>::Index(number_of_buffers.get()), Self::Null, 0, 0);
		self.zero_rw_flags();
		self.set_buffer_group(buffer_group)
	}

	#[inline(always)]
	fn prepare(self, user_data: impl UserData, options: SubmissionQueueEntryOptions, personality: Option<PersonalityCredentialsIdentifier>, io_priority: CompressedIoPriority, ioring_operation: IORING_OP, file_descriptor: FileDescriptorOrigin<impl FileDescriptor>, address: u64, length: u32, offset: u64)
	{
		self.pointer_mut().prepare(ioring_operation, file_descriptor, address, length, offset, options.into_flags(), personality, io_priority, user_data, self.using_kernel_submission_queue_poll())
	}

	#[inline(always)]
	fn set_receive_flags(self, flags: ReceiveFlags)
	{
		self.set_message_flags(SendOrReceiveFlags { receive: flags })
	}

	#[inline(always)]
	fn set_send_flags(self, flags: SendFlags)
	{
		self.set_message_flags(SendOrReceiveFlags { send: flags })
	}

	#[inline(always)]
	fn set_message_flags(self, flags: SendOrReceiveFlags)
	{
		self.anonymous_3().msg_flags = flags
	}

	#[inline(always)]
	fn set_advice(self, advice: FileOrMemoryAdvice)
	{
		self.anonymous_3().fadvise_advice = advice
	}

	#[inline(always)]
	fn set_timeout_flags(self, relative_or_absolute: RelativeOrAbsoluteTimeout)
	{
		self.anonymous_3().timeout_flags = relative_or_absolute
	}

	#[inline(always)]
	fn set_buffer_group(self, buffer_group: BufferGroup)
	{
		self.anonymous_4().anonymous_1.anonymous_1.buf_group = buffer_group
	}

	#[inline(always)]
	fn set_registered_buffer_index(self, registered_buffer_index: RegisteredBufferIndex)
	{
		self.anonymous_4().anonymous_1.anonymous_1.buf_index = registered_buffer_index
	}
	
	#[inline(always)]
	fn zero_buf_index(self)
	{
		self.anonymous_4().anonymous_1.anonymous_1.buf_index = unsafe { zeroed() }
	}
	
	#[inline(always)]
	fn zero_rw_flags(self)
	{
		self.anonymous_3().rw_flags = 0
	}

	#[inline(always)]
	fn anonymous_2<'a>(self) -> &'a mut io_uring_sqe_anonymous_2
	{
		&mut self.pointer_mut().anonymous_2
	}

	#[inline(always)]
	fn anonymous_3<'a>(self) -> &'a mut io_uring_sqe_anonymous_3
	{
		&mut self.pointer_mut().anonymous_3
	}

	#[inline(always)]
	fn anonymous_4<'a>(self) -> &'a mut io_uring_sqe_anonymous_4
	{
		&mut self.pointer_mut().anonymous_4
	}

	const Null: u64 = unsafe { null::<()>() as usize as u64 };
	
	#[inline(always)]
	fn non_empty_path(path: &CStr) -> NonNull<c_char>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");
		
		unsafe { NonNull::new_unchecked(path.as_ptr() as *mut _) }
	}
	
	#[inline(always)]
	fn empty_path() -> NonNull<c_char>
	{
		const EmptyPath: &'static [u8] = b"\0";
		unsafe { NonNull::new_unchecked(EmptyPath.as_ptr() as *const u8 as *const c_char as *mut _) }
	}

	#[inline(always)]
	fn to_u64_buffer(value: &[u8]) -> u64
	{
		value.as_ptr() as usize as u64
	}

	#[inline(always)]
	fn to_u64_buffer_mut(value: &mut [u8]) -> u64
	{
		value.as_mut_ptr() as usize as u64
	}

	#[inline(always)]
	fn to_u64_buffers(value: &[&[u8]]) -> u64
	{
		value.as_ptr() as usize as u64
	}

	#[inline(always)]
	fn to_u64_buffers_mut(value: &[&mut [u8]]) -> u64
	{
		value.as_ptr() as usize as u64
	}

	#[inline(always)]
	fn to_u64_non_null<T>(value: NonNull<T>) -> u64
	{
		value.as_ptr() as usize as u64
	}

	#[inline(always)]
	fn to_u64<T>(value: &T) -> u64
	{
		value as *const T as usize as u64
	}

	#[inline(always)]
	fn to_u64_mut<T>(value: &mut T) -> u64
	{
		value as *mut T as usize as u64
	}
	
	#[inline(always)]
	fn guard_not_using_io_poll(&self)
	{
		debug_assert!(!self.using_io_poll, "Using SetupFlags::IoPoll");
	}
	
	#[inline(always)]
	fn pointer_mut<'a>(self) -> &'a mut io_uring_sqe
	{
		unsafe { &mut * self.pointer.as_ptr() }
	}
}
