// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub struct ThreadLoop<T: Terminate>
{
	terminate: Arc<T>,
	incoming_messages_queue: PerThreadQueueSubscriber<T, (), DequeuedMessageProcessingError>,
	io_uring: IoUring,
}

impl<T: Terminate> ThreadLoopBodyFunction for ThreadLoop<T>
{
	#[inline(always)]
	fn invoke(&mut self)
	{

		// TODO: every thread probably needs a thread-specific signal handler.
		// If this is signalfd, does it work with io-uring?
		
		// TODO: Why use poll?
		
		// TODO: IDEA:-
		/*
			For a read or write (or send / recv or sendmsg / recvmsg or connect())
			
			- register a read op
			- register a linked timeout op
				- we can then automatically kill most kinds of coroutine
				
			For a close
				- need to wait for close to complete so we can release file descriptor
		 */
		
		/*
			TODO: Registering file descriptors
		 */
		
		Submission polling requires us to always register file descriptors. This can be expensive if not thought about. Inevitably we will have holes in our index necessitating the use of an arena linked list.
		
		Coroutines will need to co-operate.
		
		Closing a file descriptor will require calling a sqe to update the file descriptor entries.
		
		// SQE kernel poll patch: https://patchwork.kernel.org/patch/10803509/
		
		// TODO: Iouring / signalfd: previously bugs when using poll: https://lwn.net/ml/linux-kernel/1a5b156a-fde5-507b-d5cf-f42ba3eacf1a@kernel.dk/
		
		// TODO: Registered buffers allow us to avoid kernel to userspace copies
		
		// TODO: Registered file descriptors allow us to use a kernel SQ thread.

		// TODO: Need to change file descriptor to NOT CLOSE ON drop() if using non-syscall close!!!
		
		// TODO: IDEA:-
		/*
			For coroutines starting several operations in parallel
				- link them
				- use a linked timeout
		 */
		
		// TODO: call .enter() occassionally.

		if self.io_uring.submission_queue_ring_is_full()
		{
			// TODO: drawdown
		}

		let iterator = self.io_uring.current_completion_queue_entries();
		for completion_queue_entry in iterator
		{
			let user_data = UserData(completion_queue_entry.user_data());
			let result = completion_queue_entry.result_or_error();

			xxx(user_data, result)
		}
		drop(iterator);

		let result = self.io_uring.initiate_asynchronous_io();


		// TODO: Use this when the submission queue is rather full?
		// TODO: Use this when we know how many operations all woken coroutines have made?
		self.io_uring.initiate_asynchronous_io_and_wait(minimum_wanted_to_completed);




		use self::DequeuedMessageProcessingError::*;

		let message_handler_arguments = ();
		let result = self.incoming_messages_queue.receive_and_handle_messages(());
		match result
		{
			Err(Fatal(ref cause)) =>
			{
				self.terminate.begin_termination_due_to_irrecoverable_error(cause, None);
				return
			},

			Err(CarryOn(ref cause)) => ProcessLoggingConfiguration::syslog(SyslogPriority::warning, format!("CarryOn:{}", cause)),

			Ok(()) => (),
		}

		// TODO: Make sure we draw-down the completion queue as much as possible before invoking new coroutines or re-entering existing ones.
	}
}

/// This is shared between a thread and a coroutine.
///
/// It probably should be owned by the coroutine and on its stack as a 'coroutine local'.
struct CoroutineParallelOperationSlots
{
	/// Each value is the result of `CQE`.
	///
	/// Up to 16 CQEs can be in-flight at once.
	///
	/// All CQEs must complete before the coroutine is woken up.
	/// This design does not let us:-
	///
	/// * Cancel timeouts.
	/// * Cancel SQEs.
	/// * Remove poll add requests.
	///
	///
	/// Requires a 2-part design:-
	/// - Part (a) creates the SQE
	/// - Part (b) handles the CQE res code put in this slot.
	operation_slots: [Operation; 16],

	operations_submitted: usize,

	operations_completed_bit_set: usize,

	coroutine_index: usize
}

union Operation
{
	completion_queue_entry_result: i32,
	user_data: UserData,
}

#[repr(u8)]
enum OriginalRequestCancelationKind
{
	Poll = 0,

	TimeoutRelative = 1,

	TimeoutAbsolute = 2,

	AnythingElse = 3,
}

// TODO: Revisit design to allow cancel of timeouts and SQEs (and possibly poll adds).
// TODO: We can now get to UserData which we can use to cancel a request.

impl CoroutineParallelOperationSlots
{
	const ExclusiveMaximumOperationSlots: usize = 16;

	#[inline(always)]
	fn new(coroutine_index: usize) -> Self
	{
		debug_assert!(coroutine_index < UserData::ExclusiveMaximumCoroutineIndex);
		Self
		{
			operation_slots: unsafe { zeroed() },
			operations_submitted: 0,
			operations_completed_bit_set: 0,
			coroutine_index,
		}
	}

	#[inline(always)]
	fn coroutine_next_operation_slot_index(&mut self, original_request_cancelation_kind: OriginalRequestCancelationKind) -> Result<usize, ()>
	{
		debug_assert_eq!(self.operations_completed_bit_set, 0, "Not all operations have been completed, ie coroutine_was_woken_up_complete() should have been called another {} time(s)!", self.operations_completed_bit_set.count_ones());

		let next_operation_slot_index = self.operations_submitted;

		debug_assert!(next_operation_slot_index <= Self::MaximumSlots);
		if unlikely!(next_operation_slot_index == Self::MaximumSlots)
		{
			Err(())
		}

		let user_data = UserData::from_coroutine_operation_slot_index(original_request_cancelation_kind, self.coroutine_index, next_operation_slot_index);
		unsafe { *self.operation_slots.get_unchecked_mut(next_operation_slot_index) = Operation { user_data } };
		self.operations_submitted += 1;
		Ok(next_operation_slot_index)
	}

	#[inline(always)]
	fn coroutine_just_before_yield(&mut self)
	{
		debug_assert_eq!(self.operations_completed_bit_set, 0, "Not all operations have been completed, ie coroutine_was_woken_up_complete() should have been called another {} time(s)!", self.operations_completed_bit_set.count_ones());
		debug_assert_ne!(self.operations_submitted, 0, "No operations have been submitted!");
	}

	#[inline(always)]
	fn thread_update_results_return_true_if_coroutine_should_be_woken_up(&mut self, result_now_available_for_operation_slot_index: usize, completion_queue_entry_result: i32) -> bool
	{
		let bit = (1 << result_now_available_for_operation_slot_index);

		debug_assert!(result_now_available_for_operation_slot_index < Self::MaximumSlots);

		debug_assert_eq!(self.operations_completed_bit_set & bit, 0, "Already completed once");
		unsafe { *self.operation_slots.get_unchecked_mut(result_now_available_for_operation_slot_index) = Operation { completion_queue_entry_result } };
		self.operations_completed_bit_set |= bit;

		let wake_up_coroutine = self.operations_completed_bit_set.count_ones() == self.operations_submitted;
		wake_up_coroutine
	}

	#[inline(always)]
	fn coroutine_was_woken_up_complete<R, Error>(&mut self, operation_slot_index: usize, complete: impl FnOnce(i32) -> Result<R, Error>) -> Result<R, Error>
	{
		let bit = (1 << operation_slot_index);

		debug_assert!(operation_slot_index < Self::MaximumSlots);

		debug_assert_ne!(self.operations_completed_bit_set & bit, 0, "Already completed once");
		self.operations_completed_bit_set &= !bit;

		debug_assert_ne!(self.operations_submitted, 0);
		self.operations_submitted -= 1;

		let operation = unsafe { *self.operation_slots.get_unchecked(operation_slot_index) };
		complete(unsafe { operation.completion_queue_entry_result })
	}
}

/// User data.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct UserData(u64);

impl Into<u64> for UserData
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl UserData
{
	const IsCoroutineBit: u64 = 0x8000_0000_0000_0000;

	const OriginalRequestCancelationKindMask: u64 = 0x6000_0000_0000_0000;

	const OriginalRequestCancelationKindMaskShift: u64 = 61;

	const CoroutineIndexMask: u64 = 0x0000_0000_FFFF_FFF0;

	const CoroutineIndexShift: u64 = 4;

	const ExclusiveMaximumCoroutineIndex: usize = ((Self::CoroutineIndexMask >> Self::CoroutineIndexShift) + 1) as usize;

	#[inline(always)]
	pub fn from_coroutine_operation_slot_index(original_request_cancelation_kind: OriginalRequestCancelationKind, coroutine_index: usize, operation_slot_index: usize) -> Self
	{
		debug_assert!(coroutine_index < Self::ExclusiveMaximumCoroutineIndex);
		debug_assert!(operation_slot_index < CoroutineParallelOperationSlots::ExclusiveMaximumOperationSlots);

		Self(Self::IsCoroutineBit | ((original_request_cancelation_kind as u8 as u64) << Self::OriginalRequestCancelationKindMaskShift) | ((operation_slot_index as u64) << Self::CoroutineIndexShift) | (operation_slot_index as u64))
	}

	#[inline(always)]
	pub const fn is_for_coroutine(self) -> bool
	{
		(self.0 & Self::IsCoroutineBit) != 0
	}

	/// `0 .. 3`.
	#[inline(always)]
	pub const fn original_request_cancelation_kind(self) -> OriginalRequestCancelationKind
	{
		unsafe { transmute(((self.0 & Self::OriginalRequestCancelationKindMask) >> Self::OriginalRequestCancelationKindMaskShift) as u8) }
	}

	/// Check `is_for_coroutine()` first.
	///
	/// `0 .. (2 ^ 28)`.
	#[inline(always)]
	pub const fn coroutine_index(self) -> usize
	{
		((self.0 & Self::CoroutineIndexMask) >> Self::CoroutineIndexShift) as usize
	}

	/// Check `is_for_coroutine()` first.
	///
	/// `0 .. CoroutineParallelOperationSlots::ExclusiveMaximumOperationSlots` (where `CoroutineParallelOperationSlots::ExclusiveMaximumOperationSlots` is currently 16).
	#[inline(always)]
	pub const fn coroutine_operation_slot_index(self) -> usize
	{
		(self.0 & 0x0000_0000_0000_000F) as usize
	}
}

/*
	user_data =>
		- last operation performed so we can interpret cqe.res
			- and file descriptor type so we can 'rehydrate' eg a particular SocketEnum
				- eg 2 types of file (memory, regular file)
			- this type code needs to combine file-descriptor-type + IORING_OP
				- not all ops for all file descriptor types
		- file descriptor or file descriptor index
			- non-atomic count of file descriptor so we know when to close.
		- coroutine to invoke to continue operation
			- not all operations are per-coroutine; some are per thread
				- eg if trying to invoke a new coroutine for a new socket file descriptor, we're out of coroutine memory; we'll need to close(new_socket_fd) using io-uring
		- any data structures, such as buffers, open_how, &mut statx, etc that can now be freed (recycled)
			- these should reside in an Arena of the particular type.
	closing a file descriptor
		- count reaches 0
			- add a close() event, but still retain entry as close() can fail
			- if using reg'd file descriptors, chain-link close SQE to a file update SQE

	Do we need to reference count file descriptors
		- No - Use dup2() et al so different coroutines and threads don't need to open the same file path

	Challenges
		- There is no ordering to completions unless 'linked'
		- We can avoid having ANY file descriptor typing information by relying solely on the coroutine to 'know'
			- If we want to enable coroutines that can start multiple operations in parallel, we need to provide a way so they know which of their operations has completed
			- we could do this by using, say, 4 bits of the user_data to give 16 parallel operations per coroutine.

	Operation and File Descriptor permutations

			splice 9 in file descriptors, 9 out file descriptor types
				81

			read_vectored File, MemoryFileDescriptor
			read_fixed File, MemoryFileDescriptor
			read File, MemoryFileDescriptor
			write_vectored File, MemoryFileDescriptor
			write_fixed File, MemoryFileDescriptor
			write File, MemoryFileDescriptor
				12

			fsync_synchronize_all & fsync_synchronize_data_only File
				1

			accept4 3 kinds of socket
				3

			connect 3 kinds of socket
				3

			recv 6 kinds of socket
				6

			send 6 kinds of socket
				6

			recvmsg 1 if restricted to Unix domain socket.
				1

			sendmsg 1 if restricted to Unix domain socket.
				1

			epoll_ctl_add / epoll_ctl_modify / epoll_ctl_delete 1 epoll type + ANY being acted upon (for which we need to reference count)
				1 + ?

			fallocate File
				1

			fadvise File
				1

			synchronize_file_range File
				1

			openat Directory
				- Directories have a special value for CWD, so there are 2 PERMUTATIONS!
				2

			openat2 Directory
				- Directories have a special value for CWD, so there are 2 PERMUTATIONS!
				2

			statx Directory
				- Directories have a special value for CWD, so there are 2 PERMUTATIONS!
				2

			= 124

			poll_add ANY (including those not otherwise supported by io-uring) [could be modelled as if only one choice]
			close ANY (including those not otherwise supported by io-uring) [could be modelled as if only one choice]

			poll_remove (None)
			files_update (None)
			cancel (None but original user_data)
			timeout (None)
			link_timeout (None)
			timeout_remove (None but original user_data)
			madvise (None)
			provide_buffers (None)
			remove_buffers (None)


			nop (exclude as useless)
*/
