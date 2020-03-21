// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn anonymous_pipe_between_parent_and_child() -> Result<(SendPipeFileDescriptor, ReceivePipeFileDescriptor), CouldNotStartChildProcessError>
{
	let (send_pipe_file_descriptor, receive_pipe_file_descriptor) = SendPipeFileDescriptor::new_anonymous_pipe()?;
	receive_pipe_file_descriptor.make_blocking();
	Ok((send_pipe_file_descriptor, receive_pipe_file_descriptor))
}
