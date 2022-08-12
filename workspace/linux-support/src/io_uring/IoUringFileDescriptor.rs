// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An io_uring file descriptor.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct IoUringFileDescriptor
{
	raw_file_descriptor: RawFd,
	using_kernel_submission_queue_poll: bool,
	using_io_poll: bool,
}

impl Drop for IoUringFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.raw_file_descriptor.close()
	}
}

impl FromRawFd for IoUringFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(_raw_file_descriptor: RawFd) -> Self
	{
		unimplemented!("Can not be implemented")
	}
}

impl IntoRawFd for IoUringFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.raw_file_descriptor
	}
}

impl AsRawFd for IoUringFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.raw_file_descriptor
	}
}

impl FileDescriptor for IoUringFileDescriptor
{
}

impl MemoryMappableFileDescriptor for IoUringFileDescriptor
{
}

impl IoUringFileDescriptor
{
	/// Using a kernel thread requires the `CAP_SYS_ADMIN` capability.
	/// `number_of_submission_queue_entries` can be thought of as the submission queue depth; it is clamped to a maximum of 32,768.
	/// `number_of_completion_queue_entries` can be thought of as the completion queue depth; if unspecified, it defaults to double the `number_of_submission_queue_entries` up to a maximum of 65,536.
	fn new(number_of_submission_queue_entries: NonZeroU16, number_of_completion_queue_entries: Option<NonZeroU32>, kernel_submission_queue_thread_configuration: Option<&LinuxKernelSubmissionQueuePollingThreadConfiguration>, shared_work_queue: Option<&Self>) -> io::Result<(Self, io_uring_params)>
	{
		const IORING_MAX_ENTRIES: u32 = 32768;
		const IORING_MAX_CQ_ENTRIES: u32 = 2 * IORING_MAX_ENTRIES;

		let number_of_submission_queue_entries = number_of_submission_queue_entries.get() as u32;
		debug_assert!(number_of_submission_queue_entries < IORING_MAX_ENTRIES);
		
		let (sq_thread_cpu, sq_thread_idle, mut flags) = LinuxKernelSubmissionQueuePollingThreadConfiguration::configure(kernel_submission_queue_thread_configuration, SetupFlags::Clamp);

		let mut parameters = io_uring_params
		{
			// Specified if `flags` contains `SetupFlags::CompletionQueueSize`.
			cq_entries: match number_of_completion_queue_entries
			{
				None => unsafe_uninitialized(),
				Some(number_of_completion_queue_entries) =>
				{
					flags |= SetupFlags::CompletionQueueSize;
					let number_of_completion_queue_entries = number_of_completion_queue_entries.get();
					debug_assert!(number_of_completion_queue_entries <= IORING_MAX_CQ_ENTRIES, "number_of_completion_queue_entries {} exceeds IORING_MAX_CQ_ENTRIES {}", number_of_completion_queue_entries, IORING_MAX_CQ_ENTRIES);
					number_of_completion_queue_entries
				}
			},

			sq_thread_cpu,

			sq_thread_idle,
			
			wq_fd: if let Some(shared_work_queue) = shared_work_queue
			{
				flags |= SetupFlags::AttachWorkQueue;
				shared_work_queue.as_raw_fd()
			}
			else
			{
				unsafe_zeroed()
			},

			flags,

			features: unsafe_zeroed(),
			resv: unsafe_zeroed(),
			sq_entries: unsafe_uninitialized(),
			sq_off: unsafe_uninitialized(),
			cq_off: unsafe_uninitialized(),
		};

		let result = SystemCallNumber::system_call_io_uring_setup(number_of_submission_queue_entries, &mut parameters);
		if likely!(result >= 0)
		{
			let features = parameters.features;

			if unlikely!(!features.contains(ParametersFeatureFlags::AllAsOfLinux57))
			{
				panic!("Essential features coded for not supported (instead {:?} is supported", features)
			}

			let this = Self
			{
				raw_file_descriptor: result,
				using_kernel_submission_queue_poll: flags.contains(SetupFlags::SubmissionQueuePoll),
				using_io_poll: flags.contains(SetupFlags::IoPoll),
			};
			
			Ok((this, parameters))
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from io_uring_setup()", result))
		}
	}

	/// Call this if we either want new events or we need to wake up the kernel submission queue poll thread.
	///
	/// `to_submit` specifies the number of I/Os to submit from the submission queue.
	///
	/// A single call can both submit new I/O and wait for completions of I/O initiated by this call or previous calls to io_uring_enter().
	///
	/// If the io_uring instance was configured for interrupt driven I/O (where IORING_SETUP_IOPOLL was not specified in the call to io_uring_setup(2)), an application may check the completion queue for event completions without entering the kernel at all.
	/// `minimum_wanted_to_complete` is a request, but not order, to wait until at least that many I/O event completions have occurred.
	///
	/// If the io_uring instance was configured for polling, by specifying IORING_SETUP_IOPOLL in the call to io_uring_setup(2), then min_complete has a slightly different meaning.
	/// Passing a value of 0 instructs the kernel to return any events which are already complete, without blocking.
	/// If min_complete is a non-zero value, the kernel will still return immediately if any completion events are available.
	/// If no event completions are available, then the call will poll either until one or more completions become available, or until the process has exceeded its scheduler time slice.
	///
	/// When the system call returns that a certain amount of SQEs have been consumed and submitted, it's safe to reuse SQE entries in the ring.
	///
	/// Returns the number of I/Os successfully consumed.
	#[inline(always)]
	fn enter(&self, minimum_wanted_to_complete: Option<NonZeroU32>, submission_queue_ring: &SubmissionQueueRing, temporary_thread_signal_mask: Option<NonNull<sigset_t>>) -> Result<u32, SubmitError>
	{
		let to_submit = submission_queue_ring.length();

		let (mut flags, minimum_wanted_to_complete) = match minimum_wanted_to_complete
		{
			None => (EnterFlags::empty(), 0),
			Some(minimum_wanted_to_complete) => (EnterFlags::GetEvents, minimum_wanted_to_complete.get()),
		};

		if self.using_kernel_submission_queue_poll
		{
			if submission_queue_ring.needs_to_wake_up_kernel_submission_queue_poll_thread()
			{
				flags |= EnterFlags::SubmissionQueueWakeUp
			}
			// Fast Poll feature.
			else if minimum_wanted_to_complete == 0
			{
				return Ok(to_submit)
			}
		}

		let result = SystemCallNumber::system_call_io_uring_enter(self.raw_file_descriptor, to_submit, minimum_wanted_to_complete, flags, unsafe { transmute(temporary_thread_signal_mask) });
		if likely!(result >= 0)
		{
			Ok(result as u32)
		}
		else if likely!(result == -1)
		{
			use self::SubmitError::*;

			match SystemCallErrorNumber::from_errno()
			{
				EAGAIN => Err(TryAgain),
				EBUSY => Err(Busy),

				EBADF => panic!("The fd field in the submission queue entry is invalid, or the IOSQE_FIXED_FILE flag was set in the submission queue entry, but no files were registered with the io_uring instance"),

				EFAULT => panic!("IORING_OP_READ_FIXED or IORING_OP_WRITE_FIXED was specified in the opcode field of the submission queue entry, but either buffers were not registered for this io_uring instance, or the address range described by addr and len does not fit within the buffer registered at buf_index. Or, buffer is outside of the process' accessible address space."),

				EINVAL => panic!("The index member of the submission queue entry is invalid. Or, The flags field or opcode in a submission queue entry is invalid. Or, IORING_OP_NOP was specified in the submission queue entry, but the io_uring context was setup for polling (IORING_SETUP_IOPOLL was specified in the call to io_uring_setup). Or, IORING_OP_READV or IORING_OP_WRITEV was specified in the submission queue entry, but the io_uring instance has fixed buffers registered. Or, IORING_OP_READ_FIXED or IORING_OP_WRITE_FIXED was specified in the submission queue entry, and the buf_index is invalid. Or, IORING_OP_READV, IORING_OP_WRITEV, IORING_OP_READ_FIXED, IORING_OP_WRITE_FIXED or IORING_OP_FSYNC was specified in the submission queue entry, but the io_uring instance was configured for IOPOLLing, or any of addr, ioprio, off, len, or buf_index was set in the submission queue entry. Or, IORING_OP_POLL_ADD or IORING_OP_POLL_REMOVE was specified in the opcode field of the submission queue entry, but the io_uring instance was configured for busy-wait polling (IORING_SETUP_IOPOLL), or any of ioprio, off, len, or buf_index was non-zero in the submission queue entry. Or, IORING_OP_POLL_ADD was specified in the opcode field of the submission queue entry, and the addr field was non-zero."),

				ENXIO => panic!("The io_uring instance is in the process of being torn down"),
				EOPNOTSUPP => panic!("fd does not refer to an io_uring instance. Or, opcode is valid, but not supported by this kernel."),

				unexpected @ _ => panic!("Unexpected error {} from io_uring_enter()", unexpected),
			}
		}
		else
		{
			unreachable_code(format_args!("io_uring_enter() returned unexpected result {}", result))
		}
	}
	
	#[inline(always)]
	fn register_buffers(&self, buffers: &[&mut [u8]]) -> io::Result<()>
	{
		let result = self.register_array(RegisterOperation::RegisterBuffers, buffers);

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from io_uring_register()", result))
		}
	}

	#[inline(always)]
	fn unregister_all_buffers(&self) -> io::Result<()>
	{
		let result = self.unregister(RegisterOperation::UnregisterBuffers);

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from io_uring_register()", result))
		}
	}

	#[inline(always)]
	fn register_file_descriptors(&self, file_descriptors: &[SupportedFileDescriptor]) -> io::Result<()>
	{
		let result = self.register_array(RegisterOperation::RegisterFiles, file_descriptors);

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from io_uring_register()", result))
		}
	}

	#[inline(always)]
	fn unregister_all_file_descriptors(&self) -> io::Result<()>
	{
		let result = self.unregister(RegisterOperation::UnregisterFiles);

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from io_uring_register()", result))
		}
	}

	/// `replace_with_files_descriptors.len()` must be less than or equal to `i32::MAX as usize`.
	///
	/// Returns the number of file descriptors replaced.
	#[deprecated]
	#[inline(always)]
	fn replace_some_registered_file_descriptors(&self, replace_with_files_descriptors: &[SupportedFileDescriptor], starting_from_index_inclusive: RegisteredFileDescriptorIndex) -> Result<u32, ()>
	{
		let length = replace_with_files_descriptors.len();
		debug_assert!(length <= i32::MAX as usize);

		let mut argument = io_uring_files_update
		{
			offset: starting_from_index_inclusive.0,
			resv: 0,
			fds: replace_with_files_descriptors.as_ptr() as *const RawFd,
		};

		let result = self.register(RegisterOperation::RegisterFilesUpdate, &mut argument, length as u32);

		if likely!(result >= 0)
		{
			debug_assert!(result <= length as i32);
			Ok(result as u32)
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				ENOMEM => Err(()),
				ENXIO => panic!("io_uring is being torn down"),
				EINVAL => panic!("bad arguments"),

				unexpected @ _ => panic!("Unexpected error {} from io_uring_register()", unexpected)
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from io_uring_register()", result))
		}
	}

	#[inline(always)]
	fn register_event_file_descriptor_for_all_notifications(&self, event_file_descriptor: &EventFileDescriptor) -> Result<(), bool>
	{
		let result = self.register_array(RegisterOperation::RegisterEventFileDescriptor, &[event_file_descriptor.as_raw_fd()]);

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				ENOMEM => Err(false),
				EINTR => Err(true),
				ENXIO => panic!("io_uring is being torn down"),
				EINVAL => panic!("bad arguments"),

				unexpected @ _ => panic!("Unexpected error {} from io_uring_register()", unexpected)
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from io_uring_register()", result))
		}
	}

	#[inline(always)]
	fn register_event_file_descriptor_for_only_asynchronous_notifications(&self, event_file_descriptor: &EventFileDescriptor) -> Result<(), bool>
	{
		let result = self.register_array(RegisterOperation::RegisterEventFileDescriptorAsynchronous, &[event_file_descriptor.as_raw_fd()]);

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				ENOMEM => Err(false),
				EINTR => Err(true),
				ENXIO => panic!("io_uring is being torn down"),
				EINVAL => panic!("bad arguments"),

				unexpected @ _ => panic!("Unexpected error {} from io_uring_register()", unexpected)
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from io_uring_register()", result))
		}
	}

	#[inline(always)]
	fn unregister_event_file_descriptor_for_notifications(&self) -> Result<(), bool>
	{
		let result = self.unregister(RegisterOperation::UnregisterEventFileDescriptor);

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				ENOMEM => Err(false),
				EINTR => Err(true),
				ENXIO => panic!("io_uring is being torn down"),
				EINVAL => panic!("bad arguments"),

				unexpected @ _ => panic!("Unexpected error {} from io_uring_register()", unexpected)
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from io_uring_register()", result))
		}
	}

	#[inline(always)]
	fn probe_for_supported_operations(&self) -> Result<BTreeSet<IORING_OP>, ()>
	{
		let mut arguments = io_uring_probe::new();
		let result = self.register(RegisterOperation::RegisterProbe, &mut arguments, arguments.ops_len as u32);

		if likely!(result == 0)
		{
			let mut supported_operations = BTreeSet::new();
			for op in arguments.ops()
			{
				if likely!(op.flags.contains(ProbeFlags::OperationSupported) && op.op < IORING_OP::IORING_OP_LAST as u8)
				{
					supported_operations.insert(unsafe { transmute(op.op) });
				}
			}
			Ok(supported_operations)
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				ENOMEM => Err(()),

				ENXIO => panic!("io_uring is being torn down"),
				EOVERFLOW => panic!("arguments ops array field was too large"),
				EFAULT => panic!("Bad pointers to user space data"),
				EINVAL => panic!("arguments ops array field was empty"),

				unexpected @ _ => panic!("Unexpected error {} from io_uring_register()", unexpected)
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from io_uring_register()", result))
		}
	}

	#[inline(always)]
	fn register_personality(&self) -> Result<PersonalityCredentialsIdentifier, ()>
	{
		let result = self.register::<()>(RegisterOperation::RegisterPersonality, null_mut(), 0);
		if likely!(result > 0 && result <= (u16::MAX as i32))
		{
			Ok(PersonalityCredentialsIdentifier(new_non_zero_u16(result as u16)))
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				ENOMEM => Err(()),

				ENXIO => panic!("io_uring is being torn down"),
				EINVAL => panic!("Bad arg or nr_arg"),

				unexpected @ _ => panic!("Unexpected error {} from io_uring_register()", unexpected)
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from io_uring_register()", result))
		}
	}
	
	#[inline(always)]
	fn unregister_personality(&self, personality_credentials_identifier: PersonalityCredentialsIdentifier)
	{
		let result = self.register::<()>(RegisterOperation::UnregisterPersonality, null_mut(), personality_credentials_identifier.0.get() as u32);
		if likely!(result == 0)
		{
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				ENXIO => panic!("io_uring is being torn down"),
				EINVAL => panic!("Invalid personality credentials identifier"),

				unexpected @ _ => panic!("Unexpected error {} from io_uring_register()", unexpected)
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from io_uring_register()", result))
		}
	}

	#[inline(always)]
	fn unregister(&self, unregister_operation: RegisterOperation) -> i32
	{
		self.register::<()>(unregister_operation, null_mut(), 0)
	}

	#[inline(always)]
	fn register_array<Argument>(&self, register_operation: RegisterOperation, arguments: &[Argument]) -> i32
	{
		let length = arguments.len();
		debug_assert!(length <= u32::MAX as usize);
		self.register(register_operation, arguments.as_ptr() as *mut Argument, length as u32)
	}

	#[inline(always)]
	fn register<Argument>(&self, register_operation: RegisterOperation, arguments: *mut Argument, length: u32) -> i32
	{
		SystemCallNumber::system_call_io_uring_register(self.raw_file_descriptor, register_operation, arguments as *mut c_void, length)
	}
}
