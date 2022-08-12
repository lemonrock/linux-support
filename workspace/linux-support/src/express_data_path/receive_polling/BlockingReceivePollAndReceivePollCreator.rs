// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A simple, blocking receive poll.
#[derive(Debug)]
pub struct BlockingReceivePollAndReceivePollCreator([pollfd; Self::NumberOfPolledFileDescriptors]);

impl ReceivePollCreator for BlockingReceivePollAndReceivePollCreator
{
	type RP = Self;
	
	#[inline(always)]
	fn create(self, express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor) -> Self::RP
	{
		Self
		(
			[
				pollfd
				{
					fd: express_data_path_socket_file_descriptor.as_raw_fd(),
					events: POLLIN,
					revents: 0
				}
			]
		)
	}
}

impl ReceivePoll for BlockingReceivePollAndReceivePollCreator
{
	#[inline(always)]
	fn poll(&mut self)
	{
		loop
		{
			let result = unsafe { poll(self.0.as_mut_ptr(), Self::NumberOfPolledFileDescriptors as u64, Self::DefaultTimeoutInMilliseconds) };
			if likely!(result > 0)
			{
				return
			}
			else if likely!(result == 0)
			{
				continue
			}
			else if likely!(result == -1)
			{
				match SystemCallErrorNumber::from_errno()
				{
					// On Linux, `EAGAIN` is not supposed to be possible as the result is `0`.
					EINTR | ENOMEM | EAGAIN => continue,
					EFAULT => panic!("fds points outside the process's accessible address space. The array given as argument was not contained in the calling program's address space."),
					EINVAL => panic!("The nfds value exceeds the RLIMIT_NOFILE value"),
					
					unexpected @ _ => panic!("Unexpected error_number `{}`", unexpected)
				}
			}
			else
			{
				unreachable_code(format_args!("poll() returned unexpected result {}", result))
			}
		}
	}
}

impl BlockingReceivePollAndReceivePollCreator
{
	const NumberOfPolledFileDescriptors: usize = 1;
	
	const DefaultTimeoutInMilliseconds: i32 = 1000;
}
