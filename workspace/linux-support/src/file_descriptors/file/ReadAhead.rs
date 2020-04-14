// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Read ahead optimization.
pub trait ReadAhead: AsRawFd + Seek + FileExt
{
	/// This function populates the page cache with data from a file so that subsequent reads from that file will not block on disk I/O.
	///
	/// The `offset` argument specifies the starting point from which data is to be read and count specifies the number of bytes to be read.
	/// I/O is performed in whole pages, so that `offset` is effectively rounded down to a page boundary and bytes are read up to the next page boundary greater than or equal to `offset + count`).
	///
	/// Reading does not read beyond the end of the file.
	/// This function blocks until the specified data has been read.
	///
	/// The current file offset (position) of the open file is left unchanged.
	///
	/// Returns an error if the file does not support reading ahead.
	/// Panics if the file is not open for reading.
	#[inline(always)]
	fn read_ahead(&self, offset: i64, count: usize) -> Result<(), ()>
	{
		let result = unsafe { readahead(self.as_raw_fd(), offset, count) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => Err(()),
				EBADF => panic!("fd is not a valid file descriptor or is not open for reading"),
				unexpected @ _ => panic!("Unexpected error {} for readahead()", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result {} for readahead()", result)
		}
	}
}
