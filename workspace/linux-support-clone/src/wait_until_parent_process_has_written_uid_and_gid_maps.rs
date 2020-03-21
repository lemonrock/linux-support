#[inline(always)]
fn wait_until_parent_process_has_written_uid_and_gid_maps(receive_pipe_file_descriptor: &mut ReceivePipeFileDescriptor)
{
	let mut buffer = [0; 1];
	receive_pipe_file_descriptor.read_exact(&mut buffer).unwrap();
	if unlikely!(buffer[0] != WaitByte)
	{
		panic!("Did not read WaitByte")
	}
}