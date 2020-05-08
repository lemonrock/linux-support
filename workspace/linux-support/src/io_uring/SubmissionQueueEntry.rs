// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `SQE`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubmissionQueueEntry(NonNull<io_uring_sqe>);

impl SubmissionQueueEntry
{
	/*
		If using the registered file descriptors, there are substantive advantages
			- they are owned solely by the Linux kernel - this is the definitive list
			- if we use some bits in user data, we can rehydrate the kind of file descriptor.
				- further, we can use this as an index to a specify Arena of pre-allocated memory that contains the file descriptor and all its associated stuff.
	*/




	#[inline(always)]
	pub(crate) fn prepare_splice<'a>(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor_in: FileDescriptorOrigin<'a, impl SpliceSender + FileDescriptor>, offset_in: u64, file_descriptor_out: FileDescriptorOrigin<'a, impl SpliceRecipient + FileDescriptor>, offset_out: u64, number_of_bytes: u32, splice_flags: SpliceFlags)
	{
		let (splice_fd_in, splice_flags) = file_descriptor_in.into_and_adjust_splice_flags(splice_flags);

		self.prepare(user_data, options, io_priority, personality, IORING_OP_SPLICE, file_descriptor_out, Self::Null, number_of_bytes, offset_out);
		self.anonymous_2().splice_off_in = offset_in;
		self.anonymous_3().splice_flags = splice_flags;
		self.anonymous_4().anonymous_1.splice_fd_in =  splice_fd_in
	}

	/// Caller must hold onto, and not move, `buffers` until completion.
	///
	/// Equivalent to `preadv2()`.
	#[inline(always)]
	pub(crate) fn prepare_read_vectored(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<impl SeekableFileDescriptor>, buffers: &[&mut [u8]], offset: ReadOrWriteOffset, read_vectored_flags: ReadVectoredFlags)
	{
		let length = buffers.len();
		debug_assert!(length <= u32::MAX as usize);

		self.prepare(user_data, options, io_priority, personality, IORING_OP_READV, file_descriptor, Self::to_u64_buffers_mut(buffers), length as u32, offset.into());
		self.anonymous_3().read_vectored_flags = read_vectored_flags
	}

	/// Caller must hold onto, and not move, `buffer` until completion.
	#[inline(always)]
	pub(crate) fn prepare_read_fixed(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<impl SeekableFileDescriptor>, buffer: &mut [u8], offset: ReadOrWriteOffset, registered_buffer_index: u16)
	{
		let length = buffer.len();
		debug_assert!(length <= u32::MAX as usize);

		self.prepare(user_data, options, io_priority, personality, IORING_OP_READ_FIXED, file_descriptor, Self::to_u64_buffer_mut(buffer), length as u32, offset.into());
		self.zero_rw_flags();
		self.set_registered_buffer_index(registered_buffer_index)
	}

	/// Caller must hold onto, and not move, `buffer` until completion.
	#[inline(always)]
	pub(crate) fn prepare_read(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<impl SeekableFileDescriptor>, buffer: &mut [u8], offset: ReadOrWriteOffset)
	{
		let length = buffer.len();
		debug_assert!(length <= u32::MAX as usize);

		self.prepare(user_data, options, io_priority, personality, IORING_OP_READ, file_descriptor, Self::to_u64_buffer_mut(buffer), length as u32, offset.into());
		self.zero_rw_flags()
	}

	/// Caller must hold onto, and not move, `buffers` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	///
	/// Equivalent to `pwritev2()`.
	#[inline(always)]
	pub(crate) fn prepare_write_vectored(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<impl SeekableFileDescriptor>, buffers: &[&[u8]], offset: ReadOrWriteOffset, write_vectored_flags: WriteVectoredFlags)
	{
		let length = buffers.len();
		debug_assert!(length <= u32::MAX as usize);

		self.prepare(user_data, options, io_priority, personality, IORING_OP_WRITEV, file_descriptor, Self::to_u64_buffers(buffers), length as u32, offset.into());
		self.zero_rw_flags();
		self.anonymous_3().write_vectored_flags = write_vectored_flags
	}

	/// Caller must hold onto, and not move, `buffer` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	#[inline(always)]
	pub(crate) fn prepare_write_fixed(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<impl SeekableFileDescriptor>, buffer: &[u8], offset: ReadOrWriteOffset, registered_buffer_index: u16)
	{
		let length = buffer.len();
		debug_assert!(length <= u32::MAX as usize);

		self.prepare(user_data, options, io_priority, personality, IORING_OP_WRITE_FIXED, file_descriptor, Self::to_u64_buffer(buffer), length as u32, offset.into());
		self.zero_rw_flags();
		self.set_registered_buffer_index(registered_buffer_index)
	}

	/// Caller must hold onto, and not move, `buffer` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	#[inline(always)]
	pub(crate) fn prepare_write(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<impl SeekableFileDescriptor>, buffer: &[u8], offset: u64)
	{
		let length = buffer.len();
		debug_assert!(length <= u32::MAX as usize);

		self.prepare(user_data, options, io_priority, personality, IORING_OP_WRITE, file_descriptor, Self::to_u64_buffer(buffer), length as u32, offset);
		self.zero_rw_flags();
	}

	#[inline(always)]
	pub(crate) fn prepare_poll_add(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<impl FileDescriptor>, poll_mask: PollFlags)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_POLL_ADD, file_descriptor, Self::Null, 0, 0);
		self.anonymous_3().poll_events = poll_mask
	}

	#[inline(always)]
	pub(crate) fn prepare_poll_remove(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, remove_for_user_data: u64)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_POLL_REMOVE, FileDescriptorOrigin::<File>::Irrelevant, remove_for_user_data, 0, 0);
		self.zero_rw_flags()
	}

	#[inline(always)]
	pub(crate) fn prepare_fsync_synchronize_all(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<File>)
	{
		self.prepare_fsync(user_data, options, io_priority, personality, file_descriptor, FSyncFlags::empty())
	}

	#[inline(always)]
	pub(crate) fn prepare_fsync_synchronize_data_only(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<File>)
	{
		self.prepare_fsync(user_data, options, io_priority, personality, file_descriptor, FSyncFlags::DataSync)
	}

	#[inline(always)]
	pub(crate) fn prepare_fsync(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<File>, fsync_flags: FSyncFlags)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_FSYNC, file_descriptor.into(), Self::Null, 0, 0);
		self.anonymous_3().fsync_flags = fsync_flags
	}

	/// The argument `socket` is a socket that has been created with `socket(2)`, bound to a local address with `bind(2)`, and is listening for connections after a `listen(2)`.
	/// Caller must hold onto, and not move, `pending_accept_connection` until completion.
	///
	/// The `CQE` `res` field will contain either a socket file descriptor or an error code.
	#[inline(always)]
	pub(crate) fn prepare_accept4<SD: SocketData>(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, socket: FileDescriptorOrigin<impl SocketAccept>, pending_accept_connection: &mut PendingAcceptConnection<SD>)
	{
		let flags = SOCK_NONBLOCK | SOCK_CLOEXEC;

		self.prepare(user_data, options, io_priority, personality, IORING_OP_ACCEPT, socket, Self::to_u64_mut(&mut pending_accept_connection.peer_address), 0, Self::to_u64_mut(&mut pending_accept_connection.peer_address_length));
		self.anonymous_3().accept_flags = flags as u32
	}

	/// Caller must hold onto, and not move, `peer_address` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	#[inline(always)]
	pub(crate) fn prepare_connect<SD: SocketData>(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, socket: FileDescriptorOrigin<impl SocketConnect>, peer_address: &SD, peer_address_length: socklen_t)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_CONNECT, socket, Self::to_u64(peer_address), 0, peer_address_length as u64);
		self.zero_rw_flags()
	}
	
	/// Caller must hold onto, and not move, `buffer` until completion.
	#[inline(always)]
	pub(crate) fn prepare_recv(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, socket: FileDescriptorOrigin<impl NonServerSocket>, buffer: &mut [u8], flags: ReceiveFlags)
	{
		let length = buffer.len();
		debug_assert!(length <= u32::MAX as usize);

		self.prepare(user_data, options, io_priority, personality, IORING_OP_RECV, socket, Self::to_u64_buffer_mut(buffer), length as u32, 0);
		self.set_receive_flags(flags)
	}

	/// Caller must hold onto, and not move, `buffer` until completion.
	#[inline(always)]
	pub(crate) fn prepare_send(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, socket: FileDescriptorOrigin<impl NonServerSocket>, buffer: &[u8], flags: SendFlags)
	{
		let length = buffer.len();
		debug_assert!(length <= u32::MAX as usize);

		self.prepare(user_data, options, io_priority, personality, IORING_OP_SEND, socket, Self::to_u64_buffer(buffer), length as u32, 0);
		self.set_send_flags(flags)
	}

	/// Caller must hold onto, and not move, `message` until completion.
	#[inline(always)]
	pub(crate) fn prepare_recvmsg<SD: SocketData>(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, socket: FileDescriptorOrigin<impl NonServerSocket>, message: &mut ReceiveMessage<SD>, flags: ReceiveFlags)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_RECVMSG, socket, Self::to_u64_mut(&mut message.internal), 1, 0);
		self.set_receive_flags(flags)
	}

	/// Caller must hold onto, and not move, `message` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	#[inline(always)]
	pub(crate) fn prepare_sendmsg<SD: SocketData>(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, socket: FileDescriptorOrigin<impl NonServerSocket>, message: &SendMessage<SD>, flags: SendFlags)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_SENDMSG, socket, Self::to_u64(&message.internal), 1, 0);
		self.set_send_flags(flags)
	}

	/// Caller must hold onto, and not move, `Rc<epoll_event>` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	#[inline(always)]
	pub(crate) fn prepare_epoll_ctl_add(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, epoll_file_descriptor: FileDescriptorOrigin<EPollFileDescriptor>, file_descriptor: &impl FileDescriptor, flags: EPollAddFlags, token: u64) -> Rc<epoll_event>
	{
		let event = Rc::new
		(
			epoll_event
			{
				events: flags.bits(),
				data: epoll_data_t
				{
					u64: token,
				},
			}
		);

		self.prepare(user_data, options, io_priority, personality, IORING_OP_EPOLL_CTL, epoll_file_descriptor, Self::to_u64(event.as_ref()), EPOLL_CTL_ADD as u32, file_descriptor.as_raw_fd() as u32 as u64);
		self.zero_rw_flags();
		event
	}

	/// Caller must hold onto, and not move, `Rc<epoll_event>` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	#[inline(always)]
	pub(crate) fn prepare_epoll_ctl_modify(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, epoll_file_descriptor: FileDescriptorOrigin<EPollFileDescriptor>, file_descriptor: &impl FileDescriptor, flags: EPollModifyFlags, token: u64) -> Rc<epoll_event>
	{
		let event = Rc::new
		(
			epoll_event
			{
				events: flags.bits(),
				data: epoll_data_t
				{
					u64: token,
				},
			}
		);

		self.prepare(user_data, options, io_priority, personality, IORING_OP_EPOLL_CTL, epoll_file_descriptor, Self::to_u64(event.as_ref()), EPOLL_CTL_MOD as u32, file_descriptor.as_raw_fd() as u32 as u64);
		self.zero_rw_flags();
		event
	}

	#[inline(always)]
	pub(crate) fn prepare_epoll_ctl_delete(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, epoll_file_descriptor: FileDescriptorOrigin<EPollFileDescriptor>, file_descriptor: &impl FileDescriptor)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_EPOLL_CTL, epoll_file_descriptor, Self::Null, EPOLL_CTL_DEL as u32, file_descriptor.as_raw_fd() as u32 as u64);
		self.zero_rw_flags();
	}

	#[inline(always)]
	pub(crate) fn prepare_fallocate(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<File>, mode: AllocationMode, offset: u64, length: u64)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_FALLOCATE, file_descriptor, length, mode.bits() as u32, offset);
		self.zero_rw_flags()
	}

	#[inline(always)]
	pub(crate) fn prepare_fadvise(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: FileDescriptorOrigin<File>, offset: u64, length: u32, advice: Advice)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_FADVISE, file_descriptor, Self::Null, length, offset);
		self.set_advice(FileOrMemoryAdvice { file: advice })
	}

	/// Caller must hold onto, and not move, `replace_with_files_descriptors` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	#[inline(always)]
	pub(crate) fn prepare_files_update(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, replace_with_files_descriptors: &[SupportedFileDescriptor], starting_from_index_inclusive: u32)
	{
		let length = replace_with_files_descriptors.len();
		debug_assert!(length < u32::MAX as usize);

		self.prepare(user_data, options, io_priority, personality, IORING_OP_FILES_UPDATE, FileDescriptorOrigin::<File>::Irrelevant, replace_with_files_descriptors.as_ptr() as usize as u64, length as u32, starting_from_index_inclusive as u64);
		self.zero_rw_flags()
	}

	#[inline(always)]
	pub(crate) fn prepare_cancel(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, cancel_for_user_data: u64)
	{
		const UnusedFlags: u32 = 0;

		self.prepare(user_data, options, io_priority, personality, IORING_OP_ASYNC_CANCEL, FileDescriptorOrigin::<File>::Irrelevant, cancel_for_user_data, 0, 0);
		self.anonymous_3().cancel_flags = UnusedFlags
	}

	#[inline(always)]
	pub(crate) fn prepare_nop(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_NOP, FileDescriptorOrigin::<File>::Irrelevant, Self::Null, 0, 0);
		self.zero_rw_flags()
	}

	/// Caller must hold onto, and not move, `time` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	#[inline(always)]
	pub(crate) fn prepare_timeout(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, time: &__kernel_timespec, count: u64, relative_or_absolute: RelativeOrAbsoluteTimeout)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_TIMEOUT, FileDescriptorOrigin::<File>::Irrelevant, Self::to_u64(time), 1, count);
		self.set_timeout_flags(relative_or_absolute)
	}

	/// Caller must hold onto, and not move, `time` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	#[inline(always)]
	pub(crate) fn prepare_link_timeout(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, time: &__kernel_timespec, relative_or_absolute: RelativeOrAbsoluteTimeout)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_LINK_TIMEOUT, FileDescriptorOrigin::<File>::Irrelevant, Self::to_u64(time), 1, 0);
		self.set_timeout_flags(relative_or_absolute)
	}

	#[inline(always)]
	pub(crate) fn prepare_timeout_remove(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, remove_for_user_data: u64, relative_or_absolute: RelativeOrAbsoluteTimeout)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_TIMEOUT_REMOVE, FileDescriptorOrigin::<File>::Irrelevant, remove_for_user_data, 0, 0);
		self.set_timeout_flags(relative_or_absolute)
	}

	#[inline(always)]
	pub(crate) fn prepare_madvise(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, address: VirtualAddress, length: u32, advice: MemoryAdvice)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_MADVISE, FileDescriptorOrigin::<File>::Irrelevant, address.into(), length, 0);
		self.set_advice(FileOrMemoryAdvice { memory: advice  })
	}

	#[inline(always)]
	pub(crate) fn prepare_close<FD: FileDescriptor>(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, file_descriptor: &FD)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_CLOSE, FileDescriptorOrigin::<FD>::Absolute(file_descriptor), Self::Null, 0, 0);
		self.zero_rw_flags()
	}

	/// Caller must hold onto, and not move, `path` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	#[inline(always)]
	pub(crate) fn prepare_openat(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, directory_file_descriptor: &DirectoryFileDescriptor, path: &CStr, flags: i32, mode: mode_t)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_OPENAT, FileDescriptorOrigin::<DirectoryFileDescriptor>::Absolute(directory_file_descriptor), Self::to_u64_c_str(path), mode, 0);
		self.anonymous_3().open_flags = flags as u32
	}

	/// Caller must hold onto, and not move, `path` and `how` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed.
	#[inline(always)]
	pub(crate) fn prepare_openat2(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, directory_file_descriptor: &DirectoryFileDescriptor, path: &CStr, how: &mut open_how)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_OPENAT2, FileDescriptorOrigin::<DirectoryFileDescriptor>::Absolute(directory_file_descriptor), Self::to_u64_c_str(path), size_of::<open_how>() as u32, Self::to_u64_mut(how));
		self.zero_rw_flags()
	}

	/// Caller must hold onto, and not move, `path` and `staxbuf` until completion, or, if the parameters features flag `ParametersFeatureFlags::SubmitStable` is set, until the `SubmissionQueueEntry` has been consumed for `path` (but not `statx_buffer`).
	#[inline(always)]
	pub(crate) fn prepare_statx(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, directory_file_descriptor: &DirectoryFileDescriptor, path: &CStr, flags: i32, mask: u32, statx_buffer: &mut statx)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_STATX, FileDescriptorOrigin::<DirectoryFileDescriptor>::Absolute(directory_file_descriptor), Self::to_u64_c_str(path), mask, Self::to_u64_mut(statx_buffer));
		self.anonymous_3().statx_flags = flags as u32
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
	pub(crate) fn prepare_provide_buffers(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, buffers: &mut [u8], number_of_buffers: u32, buffer_group_identifier: u16, buffer_identifier: u16)
	{
		let number_of_bytes = buffers.len();
		debug_assert_eq!(number_of_bytes % (number_of_buffers as usize), 0);
		let every_buffer_length = number_of_bytes / (number_of_buffers as usize);
		debug_assert!(every_buffer_length <= u32::MAX as usize);

		self.prepare(user_data, options, io_priority, personality, IORING_OP_PROVIDE_BUFFERS, FileDescriptorOrigin::<File>::Index(number_of_buffers), Self::to_u64_buffer_mut(buffers), every_buffer_length as u32, buffer_identifier as u64);
		self.zero_rw_flags();
		self.set_buffer_group(buffer_group_identifier)
	}

	/// Take back a contiguous array of buffers given for read-like operations to the Linux kernel.
	#[inline(always)]
	pub(crate) fn prepare_remove_buffers(self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, number_of_buffers: u32, buffer_group_identifier: u16)
	{
		self.prepare(user_data, options, io_priority, personality, IORING_OP_REMOVE_BUFFERS, FileDescriptorOrigin::<File>::Index(number_of_buffers), Self::Null, 0, 0);
		self.zero_rw_flags();
		self.set_buffer_group(buffer_group_identifier)
	}

	#[inline(always)]
	fn prepare(mut self, user_data: u64, options: SubmissionQueueEntryOptions, io_priority: CompressedIoPriority, personality: Option<PersonalityCredentialsIdentifier>, ioring_operation: IORING_OP, file_descriptor: FileDescriptorOrigin<impl FileDescriptor>, address: u64, length: u32, offset: u64)
	{
		(unsafe { self.0.as_mut() }).prepare(ioring_operation, file_descriptor, address, length, offset, options.into_flags(), personality, io_priority, user_data)
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
	fn set_buffer_group(self, buffer_group_identifier: u16)
	{
		self.anonymous_4().anonymous_1.anonymous_1.buf_group = buffer_group_identifier
	}

	#[inline(always)]
	fn set_registered_buffer_index(self, registered_buffer_index: u16)
	{
		self.anonymous_4().anonymous_1.anonymous_1.buf_index = registered_buffer_index
	}

	#[inline(always)]
	fn zero_rw_flags(self)
	{
		self.anonymous_3().rw_flags = 0
	}

	#[inline(always)]
	fn anonymous_2<'a>(self) -> &'a mut io_uring_sqe_anonymous_2
	{
		&mut (unsafe { &mut * self.0.as_ptr() }).anonymous_2
	}

	#[inline(always)]
	fn anonymous_3<'a>(self) -> &'a mut io_uring_sqe_anonymous_3
	{
		&mut (unsafe { &mut * self.0.as_ptr() }).anonymous_3
	}

	#[inline(always)]
	fn anonymous_4<'a>(self) -> &'a mut io_uring_sqe_anonymous_4
	{
		&mut (unsafe { &mut * self.0.as_ptr() }).anonymous_4
	}

	const Null: u64 = unsafe { null::<()>() as usize as u64 };

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
	fn to_u64_c_str(value: &CStr) -> u64
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
}
