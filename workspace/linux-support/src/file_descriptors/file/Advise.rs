// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Advise (using `posix_fadvise()`).
pub trait Advise: AsRawFd + Seek + FileExt
{
	/// Programs can use `advise()` to announce an intention to access file data in a specific pattern in the future, thus allowing the kernel to perform appropriate optimizations.
	///
	/// The advice applies to a (not necessarily existent) region starting at `offset` and extending for `length` bytes (or until the end of the file if `length` is `None`) within the file.
	///
	/// The advice is not binding; it merely constitutes an expectation on behalf of the application.
	#[inline(always)]
	fn advise(&self, offset: u64, length: Option<NonZeroU64>, advice: Advice)
	{
		let result = unsafe { posix_fadvise(self.as_raw_fd(), offset as i64, length.map(|value| value.get()).unwrap_or(0) as i64, advice as i32) };
		if likely!(result == 0)
		{
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				EINVAL => panic!("An invalid value was specified for advice"),
				EBADF => panic!("fd is not a valid file descriptor"),
				ESPIPE => panic!("The specified file descriptor refers to a pipe or FIFO (before Linux 2.6.16, this was EINVAL)"),
				unexpected_error @ _ => unexpected_error!(posix_fadvise, unexpected_error),
			}
		}
		else
		{
			unexpected_result!(posix_fadvise, result)
		}
	}
}
