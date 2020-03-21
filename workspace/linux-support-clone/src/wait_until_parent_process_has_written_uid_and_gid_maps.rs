// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


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