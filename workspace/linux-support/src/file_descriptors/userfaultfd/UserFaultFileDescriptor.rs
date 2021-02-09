// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.



/// Represents an user fault instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserFaultFileDescriptor(RawFd);

impl Drop for UserFaultFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for UserFaultFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for UserFaultFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for UserFaultFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for UserFaultFileDescriptor
{
}

impl UserFaultFileDescriptor
{
	/// Creates a new blocking instance which is closed-on-exec suitable for monitoring the current process as long as it is single-threaded.
	///
	/// Only suitable if monitoring only page faults.
	///
	/// Internally creates a thread which should *NEVER* be `join()`'ed; it must run until application exit (the easiest way to do this is to `forget()` `.1`).
	/// `user_fault_event_handler_constructor` will be called on the created thread, not the thread executing `new_blocking()`.
	/// For maximum performance, consider using a per-thread memory allocator and arranging for the created thread to execute on a different CPU to the main thread.
	///
	/// `user_mode_only` should normally be `true`.
	/// `requested_features` should normally not contain `Feature::RaiseForkEvents` unless the caller has the `CAP_SYS_PTRACE` capability (will result in `EFAULT` to the api ioctl).
	///
	/// On return:-
	///
	/// * `.0` is an instance of an `UserFaultFileDescriptor`.
	/// * `.1` is a join handler for a blocking instance of an executing `BlockingUserFaultFileDescriptor` running on a separate thread.
	/// * `.2` is a list of supported features for the `UserFaultFileDescriptor`; it is always `Feature::all()` if running on Linux version 5.11.
	/// * `.3` is a list of supported input-outout control requests ('ioctl's); it is always `SupportedInputOutputControlRequests::ApplicationProgrammerInterfaces`.
	#[cold]
	pub fn new_single_threaded_blocking<UFEH: UserFaultEventHandler + 'static>(user_mode_only: bool, requested_features: Features, user_fault_event_handler_constructor: impl FnOnce(FileDescriptorCopy<UserFaultFileDescriptor>) -> UFEH + Send + 'static, thread_builder: Builder) -> Result<(Arc<Self>, JoinHandle<()>, Features, SupportedInputOutputControlRequests), BlockingUserFaultFileDescriptorCreationError>
	{
		debug_assert!(requested_features.does_not_have_requested_features_incompatible_with_single_threaded_blocking_use(), "requested_features {:?} contains features incompatible with single-threaded blocking use {:?}", requested_features, Features::FeaturesIncompatibleWithSingleThreadedUse);
		
		Self::new_blocking(user_mode_only, requested_features, SingleThreadedBlockingEventsReaderAndDispatcher::new, user_fault_event_handler_constructor, thread_builder)
	}
	
	/// Creates a new blocking instance which is closed-on-exec suitable for monitoring the current process, particularly if it is multi-threaded.
	///
	/// Internally creates a thread which should *NEVER* be `join()`'ed; it must run until application exit (the easiest way to do this is to `forget()` `.1`).
	/// `user_fault_event_handler_constructor` will be called on the created thread, not the thread executing `new_blocking()`.
	/// For maximum performance, consider using a per-thread memory allocator and arranging for the created thread to execute on a different CPU to the main thread.
	///
	/// `user_mode_only` should normally be `true`.
	/// `requested_features` should normally not contain `Feature::RaiseForkEvents` unless the caller has the `CAP_SYS_PTRACE` capability (will result in `EFAULT` to the api ioctl).
	///
	/// On return:-
	///
	/// * `.0` is an instance of an `UserFaultFileDescriptor`.
	/// * `.1` is a join handler for a blocking instance of an executing `BlockingUserFaultFileDescriptor` running on a separate thread.
	/// * `.2` is a list of supported features for the `UserFaultFileDescriptor`; it is always `Feature::all()` if running on Linux version 5.11.
	/// * `.3` is a list of supported input-outout control requests ('ioctl's); it is always `SupportedInputOutputControlRequests::ApplicationProgrammerInterfaces`.
	#[cold]
	pub fn new_multi_threaded_blocking<UFEH: UserFaultEventHandler>(user_mode_only: bool, requested_features: Features, user_fault_event_handler_constructor: impl FnOnce(FileDescriptorCopy<UserFaultFileDescriptor>) -> UFEH + Send + 'static, thread_builder: Builder, initial_number_of_events_to_read_at_once: NonZeroUsize) -> Result<(Arc<Self>, JoinHandle<()>, Features, SupportedInputOutputControlRequests), BlockingUserFaultFileDescriptorCreationError>
	{
		Self::new_blocking(user_mode_only, requested_features, move |file_descriptor, user_fault_event_handler| MultiThreadedEventsReaderAndDispatcher::new(file_descriptor, user_fault_event_handler, initial_number_of_events_to_read_at_once), user_fault_event_handler_constructor, thread_builder)
	}
	
	#[inline(always)]
	fn new_blocking<UFEH: UserFaultEventHandler, ERAD: EventsReaderAndDispatcher>(user_mode_only: bool, requested_features: Features, events_reader_and_dispatcher_constructor: impl FnOnce(&Arc<UserFaultFileDescriptor>, UFEH) -> ERAD + Send + 'static, user_fault_event_handler_constructor: impl FnOnce(FileDescriptorCopy<UserFaultFileDescriptor>) -> UFEH + Send + 'static, thread_builder: Builder) -> Result<(Arc<Self>, JoinHandle<()>, Features, SupportedInputOutputControlRequests), BlockingUserFaultFileDescriptorCreationError>
	{
		let (this, features, ioctls) = Self::new(false, user_mode_only, requested_features)?;
		
		let file_descriptor_clone = this.clone();
		
		let join_handle = thread_builder.spawn(move ||
		{
			let file_descriptor_copy = FileDescriptorCopy::new(file_descriptor_clone.0);
			let user_fault_event_handler = user_fault_event_handler_constructor(file_descriptor_copy);
			let event_reader_and_dispatcher = events_reader_and_dispatcher_constructor(&file_descriptor_clone, user_fault_event_handler);
			let mut blocking_user_fault_file_descriptor = BlockingUserFaultFileDescriptor(event_reader_and_dispatcher);
			blocking_user_fault_file_descriptor.read_and_handle_events()
		})?;
		
		Ok((this, join_handle, features, ioctls))
	}
	
	/// Creates a new non-blocking instance which is closed-on-exec suitable for monitoring non-cooperative processes.
	///
	/// `user_mode_only` should normally be `true`.
	/// `requested_features` should normally not contain `Feature::RaiseForkEvents` unless the caller has the `CAP_SYS_PTRACE` capability (will result in `EFAULT` to the api ioctl).
	/// `user_fault_event_handler_constructor` will be called on the thread executing `new_blocking()`.
	///
	/// On return:-
	///
	/// * `.0` is an instance of an `UserFaultFileDescriptor`.
	/// * `.1` is a non-blocking instance of a `NonBlockingUserFaultFileDescriptor` suitable for use on a separate polling thread.
	/// * `.2` is a list of supported features for the `UserFaultFileDescriptor`; it is always `Feature::all()` if running on Linux version 5.11.
	/// * `.3` is a list of supported input-outout control requests ('ioctl's); it is always `SupportedInputOutputControlRequests::ApplicationProgrammerInterfaces`.
	#[cold]
	pub fn new_multi_threaded_non_blocking<UFEH: UserFaultEventHandler, T: Terminate>(user_mode_only: bool, requested_features: Features, initial_number_of_events_to_read_at_once: NonZeroUsize, user_fault_event_handler_constructor: impl FnOnce(FileDescriptorCopy<UserFaultFileDescriptor>) -> UFEH, terminate: &Arc<T>) -> Result<(Arc<Self>, NonBlockingUserFaultFileDescriptor<MultiThreadedEventsReaderAndDispatcher<UFEH>, T>, Features, SupportedInputOutputControlRequests), CreationError>
	{
		let (this, features, ioctls) = Self::new(true, user_mode_only, requested_features)?;
		let file_descriptor_copy = FileDescriptorCopy::new(this.0);
		let user_fault_event_handler = user_fault_event_handler_constructor(file_descriptor_copy);
		let event_reader_and_dispatcher = MultiThreadedEventsReaderAndDispatcher::new(&this, user_fault_event_handler, initial_number_of_events_to_read_at_once);
		let non_blocking_user_fault_file_descriptor = NonBlockingUserFaultFileDescriptor::new(&this, event_reader_and_dispatcher, terminate);
		Ok((this, non_blocking_user_fault_file_descriptor, features, ioctls))
	}
	
	/// Creates a new instance.
	///
	/// `user_mode_only` should normally be `true`.
	/// `requested_features` should normally not contain `Feature::RaiseForkEvents` unless the caller has the `CAP_SYS_PTRACE` capability (will result in `EFAULT` to the api ioctl).
	///
	/// On return:-
	///
	/// * `.0` is an instance of an `UserFaultFileDescriptor`.
	/// * `.1` is a list of supported features for the `UserFaultFileDescriptor`; it is always `Feature::all()` if running on Linux version 5.11.
	/// * `.2` is a list of supported input-outout control requests ('ioctl's); it is always `SupportedInputOutputControlRequests::ApplicationProgrammerInterfaces`.
	#[cold]
	pub fn new(non_blocking: bool, user_mode_only: bool, requested_features: Features) -> Result<(Arc<Self>, Features, SupportedInputOutputControlRequests), CreationError>
	{
		let this = Self::create(non_blocking, user_mode_only)?;
		let (features, ioctls) = this.initialize(requested_features)?;
		Ok((this, features, ioctls))
	}
	
	#[inline(always)]
	fn create(non_blocking: bool, user_mode_only: bool) -> Result<Arc<Self>, CreationError>
	{
		let flags =
		{
			let mut flags = O_CLOEXEC;
			if non_blocking
			{
				flags |= O_NONBLOCK
			}
			if user_mode_only
			{
				flags |= UFFD_USER_MODE_ONLY
			}
			flags
		};
		
		let result = userfaultfd(flags);
		if likely!(result >= 0)
		{
			Ok(Arc::new(Self(result)))
		}
		else if likely!(result == -1)
		{
			use self::CreationError::*;
			
			match errno().0
			{
				EMFILE => Err(PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded),
				
				ENFILE => Err(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded),
				
				ENOMEM => Err(KernelWouldBeOutOfMemory),
				
				EPERM => Err(PermissionDenied),
				
				EINVAL => panic!("Invalid combination of flags"),
				
				unexpected @ _ => panic!("Unexpected error number '{}'", unexpected),
			}
		}
		else
		{
			panic!("Unexpected result {}", result);
		}
	}
	
	/// Obtains API details.
	///
	/// On return:-
	///
	/// * `.0` is a list of supported features; it is always `Feature::all()` if running on Linux version 5.11.
	/// * `.1` is a list of supported input-output control requests ('ioctl's); it is always `SupportedInputOutputControlRequests::ApplicationProgrammerInterfaces`.
	#[inline(always)]
	fn initialize(&self, requested_features: Features) -> Result<(Features, SupportedInputOutputControlRequests), CreationError>
	{
		let mut api = uffdio_api::new(requested_features);
		match self.make_ioctl(UFFDIO_API, &mut api)
		{
			Ok(()) => Ok((api.features, api.ioctls)),
			
			Err(errno) => match errno.0
			{
				EPERM => Err(CreationError::PermissionDenied),
				
				EINVAL => panic!("Already initialized or bad arguments to ioctl"),
				
				EFAULT => panic!("`argp` does not point to a valid memory address"),
				
				_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_API)", errno),
			}
		}
	}
	
	/// Register memory range that has previously been mapped with `mmap()`.
	///
	/// Returns the `Ioctl` operations permitted on the memory, which always fall into one of three sets which can be known without examing the return type:-
	///
	/// * Those suitable for memory backed by huge pages: always `SupportedInputOutputControlRequests::HugePages`.
	/// * Those suitable for memory backed by regular pages: always `SupportedInputOutputControlRequests::RegularPages`.
	/// * Those suitable for memory backed by regular pages and `register_mode` including `PageFaultEventNotificationSetting::RaisePageFaultEventIfWriteProtectedPageAccessed`: always `SupportedInputOutputControlRequests::RegularPagesWithWriteProtectOnCopy`.
	///
	/// Returns errors:-
	///
	/// * `KernelWouldBeOutOfMemory` if `ENOMEM` occurs (this may be because the memory has become unmapped).
	/// * `PermissionDenied` if `EPERM` occurs
	/// 	* the underlying 'vma' flags for `registered_memory_subrange` do not contain `VM_MAYWRITE` (which is related to Linux's COW of memory pages).
	/// 	* `PageFaultEventNotificationSetting::RaisePageFaultEventIfWriteProtectedPageAccessed` was specified and `registered_memory_subrange` is a hugetlbfs mapping or a shmem mapping.
	///
	/// `mapped_memory`:-
	///
	/// * must be page-aligned to memory mapped using `mmap()`; if using huge pages, then they must be aligned to the huge page alignment of that memory.
	/// * can be a sub range of memory mapped using `mmap()`.
	/// * if anonymous, must currently be `MAP_PRIVATE` (ie not have the 'vma' flag `VM_SHARED`).
	///
	/// Note that if the mapped memory is using hugepagetlbfs then only 'basic' `SupportedInputOutputControlRequests` are allowed.
	#[inline(always)]
	pub fn register_memory_subrange(&self, mapped_absolute_memory_subrange: impl AbsoluteMemoryRange, registration_settings: PageFaultEventNotificationSetting) -> Result<SupportedInputOutputControlRequests, CreationError>
	{
		use self::CreationError::*;
		
		let mut register = uffdio_register
		{
			range: Self::to_uffdio_range(mapped_absolute_memory_subrange),
			mode: registration_settings,
			ioctls: SupportedInputOutputControlRequests::empty()
		};
		match self.make_ioctl(UFFDIO_REGISTER, &mut register)
		{
			Ok(()) => Ok(register.ioctls),
			
			Err(errno) => match errno.0
			{
				ENOMEM => Err(KernelWouldBeOutOfMemory),
				
				EPERM => Err(PermissionDenied),
				
				EBUSY => panic!("A mapping in the specified range is registered with another userfaultfd object"),
				
				EFAULT => panic!("`argp` does not point to a valid memory address"),
				
				EINVAL => panic!("The userfaultfd object has not yet been enabled (via the UFFDIO_API operation); or an invalid or unsupported bit was specified in the mode field; or the mode field was zero; or there is no mapping in the specified address range; or range.start or range.len is not a multiple of the system page size (or huge page size); or, range.len is zero; or these fields are otherwise invalid; or there as an incompatible mapping in the specified address range, ie one that does not support userfaults"),
				
				_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_REGISTER)", errno),
			}
		}
	}
	
	/// Unregister memory range that has previously been registered with `self.register_registered_memory_subrange()`.
	///
	/// `mapped_memory`:-
	///
	/// * must be page-aligned to memory mapped using `mmap()`; if using huge pages, then they must be aligned to the huge page alignment of that memory.
	/// * can be a sub range of memory mapped using `mmap()`.
	///
	/// Returns errors:-
	///
	/// * `KernelWouldBeOutOfMemory` if `ENOMEM` occurs (this may be because the memory has become unmapped).
	#[inline(always)]
	pub fn unregister_memory_subrange(&self, mapped_absolute_memory_subrange: impl AbsoluteMemoryRange) -> Result<(), CreationError>
	{
		let mut range = Self::to_uffdio_range(mapped_absolute_memory_subrange);
		match self.make_ioctl(UFFDIO_UNREGISTER, &mut range)
		{
			Ok(()) => Ok(()),
			
			Err(errno) => match errno.0
			{
				EFAULT => panic!("`argp` does not point to a valid memory address"),
				
				EINVAL => panic!("The userfaultfd object has not yet been enabled (via the UFFDIO_API operation); or either the start or the len field of the ufdio_range structure was not a multiple of the system page size; or the len field was zero; or these fields were otherwise invalid; or, there as an incompatible mapping in the specified address range; or, there was no mapping in the specified address range"),
				
				ENOMEM => Err(CreationError::KernelWouldBeOutOfMemory),
				
				_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_UNREGISTER)", errno),
			}
		}
	}
	
	/// Called from polling thread; loops until all bytes copied.
	///
	/// `registered_memory_subrange` and `from` must be:-
	///
	/// * page-aligned.
	/// * encapsulated within a registered memory range (with the page size being of the same huge page size if appropriate).
	///
	/// `registered_memory_subrange`:-
	///
	/// * can be a sub range of memory mapped using `mmap()`.
	///
	/// If `write_protect` is `true`:-
	///
	/// * causes `EINVAL` if the 'vma' flags does not contain the flag `VM_UFFD_WP`, ie the registered memory was not registered to support write protection page fault notification.
	/// * only seems to make an effective difference on x86_64.
	#[inline(always)]
	pub fn copy_registered_memory_subrange(&self, registered_memory_subrange: impl AbsoluteMemoryRange, copy_starting_from: VirtualAddress, wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange: bool, write_protect: bool) -> Result<(), CopyError>
	{
		let (dst, len) = registered_memory_subrange.inclusive_absolute_start_and_length_u64();
		
		let mut copy = uffdio_copy
		{
			dst,
			src: copy_starting_from.into(),
			len,
			mode: CopyMode::new(wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange, write_protect),
			copy: 0,
		};
		
		loop
		{
			use self::CopyError::*;
			
			match self.make_ioctl(UFFDIO_COPY, &mut copy)
			{
				Ok(()) => return Ok(()),
				
				Err(errno) => match errno.0
				{
					EAGAIN =>
					{
						let bytes_copied = copy.copy as u64;
						
						copy.dst += bytes_copied;
						copy.src += bytes_copied;
						copy.len -= bytes_copied;
					}
					
					EINTR => continue,
					
					ENOENT => return Err(FaultingProcessHasChangedItsMemoryLayout),
					
					// `ENOSPC` is historic for Linux versions 4.11 to 4.13 inclusive.
					ESRCH | ENOSPC => return Err(FaultingProcessHasExited),
					
					ENOMEM => return Err(LinuxKernelIsOutOfMemory),
					
					EEXIST => return Err(DestinationMemoryStructuresSuchAsVmaDoNotExist),
					
					EFAULT => panic!("`argp` does not point to a valid memory address"),
					
					EINVAL => panic!("The userfaultfd object has not yet been enabled (via the UFFDIO_API operation); or memory addresses or length not aligned to page size or are outside of permitted range for process or invalid copy mode"),
					
					_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_COPY)", errno),
				}
			}
		}
	}
	
	/// Called from polling thread; loops until all bytes copied.
	///
	/// Writes zeroes to page(s) (logically, copies from pages of zeroes).
	///
	/// __NOTE: This API is not supported by Linux for registered memory using huge pages__.
	///
	/// `registered_memory_subrange` must be:-
	///
	/// * page-aligned.
	/// * encapsulated within a registered memory range.
	///
	/// `registered_memory_subrange`:-
	///
	/// * can be a sub range of memory mapped using `mmap()`.
	#[inline(always)]
	pub fn zero_registered_memory_subrange(&self, registered_memory_subrange: impl AbsoluteMemoryRange, wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange: bool) -> Result<(), CopyError>
	{
		let mut copy = uffdio_zeropage
		{
			range: Self::to_uffdio_range(registered_memory_subrange),
			mode: ZeroPageMode::new(wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange),
			zeropage: 0
		};
		
		loop
		{
			use self::CopyError::*;
			
			match self.make_ioctl(UFFDIO_ZEROPAGE, &mut copy)
			{
				Ok(()) => return Ok(()),
				
				Err(errno) => match errno.0
				{
					EAGAIN =>
					{
						let bytes_copied = copy.zeropage as u64;
						
						copy.range.start += bytes_copied;
						copy.range.len -= bytes_copied;
					}
					
					EINTR => continue,
					
					ENOENT => return Err(FaultingProcessHasChangedItsMemoryLayout),
					
					// `ENOSPC` is historic for Linux versions 4.11 to 4.13 inclusive.
					ESRCH | ENOSPC => return Err(FaultingProcessHasExited),
					
					ENOMEM => return Err(LinuxKernelIsOutOfMemory),
					
					EEXIST => return Err(DestinationMemoryStructuresSuchAsVmaDoNotExist),
					
					EFAULT => panic!("`argp` does not point to a valid memory address"),
					
					EINVAL => panic!("The userfaultfd object has not yet been enabled (via the UFFDIO_API operation); or memory addresses or length not aligned to page size or are outside of permitted range for process or invalid zero page mode"),
					
					_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_ZEROPAGE)", errno),
				}
			}
		}
	}
	
	/// Called from polling thread.
	///
	/// Wake up the thread waiting for page fault resolution.
	///
	/// Used after calling either `copy_registered_memory_subrange()` or `zero_registered_memory_subrange()` or both once or more with a mode of `DoNotWakeUp`.
	///
	/// `registered_memory_subrange` must be:-
	///
	/// * page-aligned.
	/// * encapsulated within a registered memory range.
	///
	/// `registered_memory_subrange`:-
	///
	/// * can be a sub range of memory mapped using `mmap()`.
	#[inline(always)]
	pub fn wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange(&self, registered_memory_subrange: impl AbsoluteMemoryRange)
	{
		let mut wake_up = Self::to_uffdio_range(registered_memory_subrange);
		
		match self.make_ioctl(UFFDIO_WAKE, &mut wake_up)
		{
			Ok(()) => (),
			
			Err(errno) => match errno.0
			{
				EFAULT => panic!("`argp` does not point to a valid memory address"),
				
				EINVAL => panic!("The userfaultfd object has not yet been enabled (via the UFFDIO_API operation); or memory addresses or length not aligned to page size or are outside of permitted range for process"),
				
				_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_WAKE)", errno),
			}
		}
	}
	
	/*
	Write Protect Notifications
	
	This is equivalent to (but faster than) using mprotect and a SIGSEGV signal handler.
	
	Firstly you need to register a range with PageFaultEventNotificationSetting::IfWriteProtectedPageAccess or similar.
	- Instead of using mprotect(2) you use ioctl(UFFDIO_WRITEPROTECT) with mode WriteProtectMode::WakeUpWithWriteProtect.
	- The ioctl(UFFDIO_WRITEPROTECT) range can be the same as or a subrange of the registered memory.
	- You can write protect as many subranges as you like (inside the registered range).
	- Then, in the thread reading from uffd the struct will have the PageFaultEventType::WriteProtectFault flag set.
	
	- Now you send ioctl(UFFDIO_WRITEPROTECT) again with mode NOT SET to WriteProtectMode::WakeUpWithWriteProtect.
	- This wakes up the thread which will continue to run with writes.
	- This allows you to do the bookkeeping about the write in the uffd reading thread before the ioctl.
	
	If you registered with PageFaultEventNotificationSetting::BothIfMissingAndIfWriteProtectedPageAccess then you need to think about the sequence in which you supply a page and undo write protect.
	Note that there is a difference between writes into a WP area and into a !WP area.
	The former will have UFFD_PAGEFAULT_FLAG_WP set, the latter UFFD_PAGEFAULT_FLAG_WRITE.
	The latter did not fail on protection but you still need to supply a page when UFFDIO_REGISTER_MODE_MISSING was used.
	 */
	/// Called from main or polling thread; loops until complete.
	///
	/// `registered_memory_subrange` must be:-
	///
	/// * page-aligned.
	/// * encapsulated within a registered memory range.
	/// * have been __registered__ (`self.register_mapped_memory()`) with `register_mode` containing `RegiserMode::WriteProtect`.
	///
	/// `registered_memory_subrange`:-
	///
	/// * can be a sub range of memory mapped using `mmap()`.
	///
	/// Return an error if the `registered_memory_subrange` no longer exists or is suitable (is shared (`VM_SHARED`), is not anonymous).
	#[inline(always)]
	pub fn enable_write_protection_of_registered_memory_subrange_and_wake_up_registered_memory_subrange(&self, registered_memory_subrange: impl AbsoluteMemoryRange)-> Result<(), ()>
	{
		self.write_protect_registered_memory_subrange_and_wake_up_registered_memory_subrange(registered_memory_subrange, WriteProtectMode::EnableWriteProtectionAndTheRaisingOfWriteProtectionEventsAndThenWakeUp)
	}
	
	/// Called from polling thread, typically in `UserFaultEventHandler::page_fault()`; loops until complete.
	///
	/// `registered_memory_subrange` must be:-
	///
	/// * page-aligned.
	/// * encapsulated within a registered memory range.
	/// * have been __registered__ (`self.register_mapped_memory()`) with `register_mode` containing `RegiserMode::WriteProtect`.
	///
	/// `registered_memory_subrange`:-
	///
	/// * can be a sub range of memory mapped using `mmap()`.
	///
	/// Return an error if the `registered_memory_subrange` no longer exists or is suitable (is shared ('vma' fag `VM_SHARED`), is not anonymous, 'vma' does not have flag `VM_UFFD_WP` (this occurs if the memory was not registered with `PageFaultEventNotificationSetting::RaisePageFaultEventIfWriteProtectedPageAccessed`.).
	///
	/// Disabling write protection *removes* the flag `VM_WRITE` from the 'vma'.
	#[inline(always)]
	pub fn disable_write_protection_of_registered_memory_subrange(&self, registered_memory_subrange: impl AbsoluteMemoryRange, wake_up_after_disabling_write_protection: bool) -> Result<(), ()>
	{
		self.write_protect_registered_memory_subrange_and_wake_up_registered_memory_subrange(registered_memory_subrange, WriteProtectMode::disable(wake_up_after_disabling_write_protection))
	}
	
	#[inline(always)]
	fn write_protect_registered_memory_subrange_and_wake_up_registered_memory_subrange(&self, registered_memory_subrange: impl AbsoluteMemoryRange, write_protect_mode: WriteProtectMode) -> Result<(), ()>
	{
		let mut write_protect = uffdio_writeprotect
		{
			range: Self::to_uffdio_range(registered_memory_subrange),
			
			mode: write_protect_mode,
		};
		
		loop
		{
			match self.make_ioctl(UFFDIO_WRITEPROTECT, &mut write_protect)
			{
				Ok(()) => return Ok(()),
				
				Err(errno) => match errno.0
				{
					EAGAIN => continue,
					
					ENOENT => return Err(()),
					
					EFAULT => panic!("`argp` does not point to a valid memory address"),
					
					EINVAL => panic!("The userfaultfd object has not yet been enabled (via the UFFDIO_API operation); or either the start or the len field of the ufdio_range structure was not a multiple of the system page size; or the len field was zero; or these fields were otherwise invalid; or, there as an incompatible mapping in the specified address range; or, there was no mapping in the specified address range"),
					
					_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_UNREGISTER)", errno),
				}
			}
		}
	}
	
	#[inline(always)]
	fn to_uffdio_range(registered_memory_subrange: impl AbsoluteMemoryRange) -> uffdio_range
	{
		let (start, len) = registered_memory_subrange.inclusive_absolute_start_and_length_u64();
		
		uffdio_range
		{
			start,
			len,
		}
	}
	
	#[inline(always)]
	fn make_ioctl<V>(&self, request: i32, value: &mut V) -> Result<(), Errno>
	{
		let result = unsafe { ioctl(self.0, request, value as *mut V as *mut c_void) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(errno())
		}
		else
		{
			panic!("Unexpected result {}", result);
		}
	}
}
