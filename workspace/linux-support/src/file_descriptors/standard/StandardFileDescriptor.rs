// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A file descriptor marker for standard in, standard out and standard error.
pub trait StandardFileDescriptor: Sized + Debug + AsRawFd
{
	#[doc(hidden)]
	const ReadOrWriteFlag: i32;

	/// Redirect to `/dev/null`.
	#[inline(always)]
	fn redirect_to_dev_null(&self, dev_path: &DevPath)
	{
		let dev_null = dev_path.null().as_os_str().os_str_to_c_string();

		let file_descriptor = self.as_raw_fd();
		let result = unsafe { open(dev_null.as_ptr(), Self::ReadOrWriteFlag) };
		if likely!(result >= 0)
		{
			let null_file_descriptor = result;
			let result = unsafe { dup2(null_file_descriptor, file_descriptor) };
			if likely!(result == 0)
			{
				// NOTE: Errors from close are ignored.
				unsafe { close(null_file_descriptor) };
			}
			else if likely!(result == -1)
			{
				panic!("Could not open dup2() because {}", io::Error::last_os_error())
			}
			else
			{
				unexpected_result!(dup2, result)
			}

		}
		else if likely!(result == -1)
		{
			panic!("Could not open /dev/null because {}", io::Error::last_os_error())
		}
		else
		{
			unexpected_result!(open, result)
		}
	}
}
