// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use linux_support::syscall::SystemCallErrorNumber;
use swiss_army_knife::unexpected_error;

/// `top_of_child_stack_pointer` points to the top (highest) address of a stack; it must be 16-byte aligned.
///
/// Interpretation of errors assumes this logic is running on at least Linux 4.9.
///
/// Returns the process identifier (PID) of the child process.
#[inline(always)]
pub(crate) fn clone_wrapper<T>(child_process: extern "C" fn(Box<T>) -> (), top_of_child_stack_pointer: NonNull<u8>, child_termination_signal_number: Signal, flags: CloneFlags, mut argument_to_child_process: Box<T>, ptid: *mut pid_t, newtls: *mut c_void, ctid: *mut pid_t) -> Result<ProcessIdentifier, CloneError>
{
	let child_stack_pointer = top_of_child_stack_pointer.as_ptr() as *mut c_void;

	debug_assert_eq!((child_stack_pointer as usize) % ChildStackAlignment, 0, "top_of_child_stack_pointer is not a multiple of 16");

	macro_rules! debug_assert_both_flags_are_not_specified
	{
		($flags: ident, $flag1: ident, $flag2: ident) =>
		{
			debug_assert!(!$flags.contains(CloneFlags::$flag1 | CloneFlags::$flag1), "CloneFlags::{} and CloneFlags::{} conflict", stringify!($flag1), stringify!($flag2));
		}
	}

	macro_rules! debug_assert_second_flag_required_if_first_specified
	{
		($flags: ident, $first_flag: ident, $second_flag: ident) =>
		{
			if cfg!(debug_assertions)
			{
				if $flags.contains(CloneFlags::$first_flag)
				{
					debug_assert!($flags.contains(CloneFlags::$second_flag), "If CloneFlags::{} is specified then CloneFlags::{} must also be specified", stringify!($first_flag), stringify!($second_flag))
				}
			}
		}
	}

	macro_rules! debug_assert_argument_can_not_be_null_if_flag_specified
	{
		($flags: ident, $flag: ident, $argument: ident) =>
		{
			if cfg!(debug_assertions)
			{
				if $flags.contains(CloneFlags::$flag)
				{
					debug_assert!(!$argument.is_null(), "`{}` can not be null if CloneFlags::{} is specified", stringify!($argument), stringify!($flag))
				}
				else
				{
					debug_assert!($argument.is_null(), "`{}` must be null if CloneFlags::{} is not specified", stringify!($argument), stringify!($flag))
				}
			}
		}
	}

	debug_assert_both_flags_are_not_specified!(flags, NewInterProcessCommunicationNamespace, ShareSysVSemaphoreAdjustmentList);
	debug_assert_both_flags_are_not_specified!(flags, NewMountNamespace, ShareFilesystemContext);
	debug_assert_both_flags_are_not_specified!(flags, NewProcessIdentifierNamespace, Thread);
	debug_assert_both_flags_are_not_specified!(flags, NewProcessIdentifierNamespace, ParentOfChildIsParentOfParent);
	debug_assert_both_flags_are_not_specified!(flags, NewUserNamespace, Thread);
	debug_assert_both_flags_are_not_specified!(flags, NewUserNamespace, ParentOfChildIsParentOfParent);
	debug_assert_both_flags_are_not_specified!(flags, NewUserNamespace, ShareFilesystemContext);

	debug_assert_second_flag_required_if_first_specified!(flags, ShareSignalHandlersTable, VirtualMachine);
	debug_assert_second_flag_required_if_first_specified!(flags, Thread, ShareSignalHandlersTable);

	debug_assert_argument_can_not_be_null_if_flag_specified!(flags, StoreChildThreadIdentifierInParentMemory, ptid);
	debug_assert_argument_can_not_be_null_if_flag_specified!(flags, SetThreadLocalStorageDescriptor, newtls);
	debug_assert_argument_can_not_be_null_if_flag_specified!(flags, ZeroChildThreadIdentifierInChildMemoryWhenChildExits, ctid);
	debug_assert_argument_can_not_be_null_if_flag_specified!(flags, StoreChildThreadIdentifierInChildMemory, ctid);

	let x: i32 = child_termination_signal_number.into();
	let flags = flags.bits | x;

	let result_code = unsafe { clone(transmute(child_process), child_stack_pointer, flags, argument_to_child_process.as_mut() as *mut T as *mut c_void, ptid, newtls, ctid) };
	drop(argument_to_child_process);

	if likely!(result_code > 0)
	{
		Ok(ProcessIdentifier::from(new_non_zero_i32(result_code)))
	}
	else if likely!(result_code == -1)
	{
		const ERESTARTNOINTR: i32 = 513;

		use self::CloneError::*;
		match SystemCallErrorNumber::from_errno()
		{
			EAGAIN => Err(MaximumNumberOfProcessesExceeded),
			ENOMEM => Err(InsufficientMemoryToAllocateChildTask),
			ENOSPC => Err(NestingOfUserOrProcessIdentifierNamespacesWouldBeExceeded),

			EPERM => panic!("Permission denied."),

			ERESTARTNOINTR => panic!("System call was interrupted during ptrace; this should never be seen by user processes (according to Linux source) but is documented in man page for clone."),
			EINVAL => panic!("Invalid combination of arguments, see `man 2 clone` as there are over 10 causes of this fault."),
			EUSERS => panic!("Should not occur as of Linux 4.9."),

			unexpected_error @ _ => unexpected_error!(clone, unexpected_error),
		}
	}
	else if result_code == 0
	{
		panic!("clone should never return a process identifier (PID) of 0 as this is reserved for Linux's scheduler.")
	}
	else
	{
		panic!("clone returned a result_code code of `{}`.", result_code)
	}
}
