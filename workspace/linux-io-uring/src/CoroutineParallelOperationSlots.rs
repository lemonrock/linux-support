// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


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

impl CoroutineParallelOperationSlots
{
	const ExclusiveMaximumOperationSlots: usize = 16;

	#[inline(always)]
	fn new(coroutine_index: usize) -> Self
	{
		debug_assert!(coroutine_index < CoroutineUserData::ExclusiveMaximumCoroutineIndex);
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

		let user_data = CoroutineUserData::from_coroutine_operation_slot_index(original_request_cancelation_kind, self.coroutine_index, next_operation_slot_index);
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
