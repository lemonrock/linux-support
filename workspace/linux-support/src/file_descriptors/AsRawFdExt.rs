// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Extensions for a file descriptor.
pub trait AsRawFdExt: AsRawFd
{
	/// Make a file descriptor blocking (all file descriptors in this library are ordinarily non-blocking)
	#[inline(always)]
	fn make_blocking(&self)
	{
		let raw_file_descriptor = self.as_raw_fd();

		let result = unsafe { fcntl(raw_file_descriptor, F_GETFL, 0) };
		let flags = if likely!(result >= 0)
		{
			result
		}
		else if likely!(result == -1)
		{
			panic!("Error from fcntl F_GETFL with `{}`", errno())
		}
		else
		{
			panic!("Unexpected result from fcntl F_GETFL of `{}`", result)
		};

		let flags_without_non_block = flags & !O_NONBLOCK;

		let result = unsafe { fcntl(raw_file_descriptor, F_SETFL, flags_without_non_block) };
		if likely!(result == 0)
		{
			()
		}
		else if likely!(result == -1)
		{
			panic!("Error from fcntl F_SETFL with `{}`", errno())
		}
		else
		{
			panic!("Unexpected result from fcntl F_SETFL of `{}`", result)
		}
	}
}
