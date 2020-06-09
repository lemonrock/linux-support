// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn write_uid_and_gid_maps_and_inform_child_process(proc_path: &ProcPath, child_process_identifier: ProcessIdentifier, send_pipe_file_descriptor: &mut SendPipeFileDescriptor) -> Result<(), CouldNotStartChildProcessError>
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
