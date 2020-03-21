#[inline(always)]
fn write_uid_and_gid_maps_and_inform_child_process(proc_path: &ProcPath, child_process_identifier: NonZeroU32, send_pipe_file_descriptor: &mut SendPipeFileDescriptor) -> Result<(), CouldNotStartChildProcessError>
{
	write_uid_and_gid_maps(proc_path, child_process_identifier)?;

	if let Err(error) = send_pipe_file_descriptor.write(&[WaitByte])
	{
		kill_wrapper(child_process_identifier);
		Err(CouldNotStartChildProcessError::from(error))
	}
	else
	{
		Ok(())
	}
}