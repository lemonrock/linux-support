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
