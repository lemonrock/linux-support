// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A child process' code to execute (ie a function).
pub trait ChildProcessFunction: Sized
{
	/// Type of data passed as arguments to the clone's (child) code in `child_process()`.
	type ChildProcessArgument;

	#[doc(hidden)]
	extern "C" fn child_process_wrapper(argument: Box<HiddenChildProcessArguments<Self::ChildProcessArgument>>)
	{
		#[inline(always)]
		fn inner<CPF: ChildProcessFunction>(argument: Box<HiddenChildProcessArguments<CPF::ChildProcessArgument>>) -> io::Result<()>
		{
			let HiddenChildProcessArguments { send_pipe_file_descriptor, mut receive_pipe_file_descriptor, new_root, child_process_argument } = argument.unbox();
			drop(send_pipe_file_descriptor);

			wait_until_parent_process_has_written_uid_and_gid_maps(&mut receive_pipe_file_descriptor);
			drop(receive_pipe_file_descriptor);
			setup_uts_namespace();
			replace_inheritated_mounts(&new_root)?;

			CPF::child_process(child_process_argument)
		}

		if let Err(_failure) = catch_unwind(AssertUnwindSafe(|| inner::<Self>(argument)))
		{
			exit(1)
		}
	}

	/// Implement this to have code to be run in the clone (child).
	fn child_process(child_process_argument: Self::ChildProcessArgument) -> io::Result<()>;
}
