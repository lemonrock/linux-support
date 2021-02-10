// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Clone a child process in a new namespace.
pub fn clone_child_process_in_new_namespace<CPF: ChildProcessFunction, A: Alloc>(proc_path: &ProcPath, child_stack_allocator: &A, child_process_stack_size: usize, child_process_argument: CPF::ChildProcessArgument, new_root: PathBuf) -> Result<(ProcessIdentifier, SendPipeFileDescriptor), CouldNotStartChildProcessError>
{
	let (mut send_pipe_file_descriptor, receive_pipe_file_descriptor) = anonymous_pipe_between_parent_and_child()?;

	let NewNamespacesFlags: CloneFlags = CloneFlags::NewCgroupNamespace | CloneFlags::NewInterProcessCommunicationNamespace | CloneFlags::NewNetworkNamespace | CloneFlags::NewMountNamespace | CloneFlags::NewProcessIdentifierNamespace | CloneFlags::NewUserNamespace | CloneFlags::NewUtsNamespace;
	let SecurityFlags: CloneFlags = CloneFlags::Untraced;

	let (layout, bottom_of_child_stack_pointer, top_of_child_stack_pointer) = allocate_child_process_stack(child_stack_allocator, child_process_stack_size)?;
	let result = clone_wrapper
	(
		CPF::child_process_wrapper,
		top_of_child_stack_pointer,
		ChildTerminationSignal,
		NewNamespacesFlags | SecurityFlags,
		Box::new
		(
			HiddenChildProcessArguments
			{
				send_pipe_file_descriptor: send_pipe_file_descriptor.clone_for_child_process(),
				receive_pipe_file_descriptor: receive_pipe_file_descriptor.clone_for_child_process(),
				new_root,
				child_process_argument,
			}
		),
		null_mut(),
		null_mut(),
		null_mut(),
	);
	deallocate_child_process_stack(child_stack_allocator, layout, bottom_of_child_stack_pointer);
	drop(receive_pipe_file_descriptor);

	let child_process_identifier = result?;

	write_uid_and_gid_maps_and_inform_child_process(proc_path, child_process_identifier, &mut send_pipe_file_descriptor)?;

	Ok((child_process_identifier, send_pipe_file_descriptor))
}
