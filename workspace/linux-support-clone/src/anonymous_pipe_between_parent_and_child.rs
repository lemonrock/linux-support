#[inline(always)]
fn anonymous_pipe_between_parent_and_child() -> Result<(SendPipeFileDescriptor, ReceivePipeFileDescriptor), CouldNotStartChildProcessError>
{
	let (send_pipe_file_descriptor, receive_pipe_file_descriptor) = SendPipeFileDescriptor::new_anonymous_pipe()?;
	receive_pipe_file_descriptor.make_blocking();
	Ok((send_pipe_file_descriptor, receive_pipe_file_descriptor))
}
