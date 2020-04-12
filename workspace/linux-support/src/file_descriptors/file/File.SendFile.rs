// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


impl SendFile for File
{
	#[inline(always)]
	fn write_output_from_file<F: AsRef<File>>(&self, from_file: &F, maximum_number_of_bytes_to_transfer: usize) -> Result<usize, StructWriteError>
	{
		if unlikely!(maximum_number_of_bytes_to_transfer == 0)
		{
			return Ok(0)
		}

		let result = unsafe { sendfile(self.as_raw_fd(), from_file.as_ref().as_raw_fd(), null_mut(), maximum_number_of_bytes_to_transfer) };
		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else if likely!(result == -1)
		{
			use self::StructWriteError::*;

			Err
			(
				match errno().0
				{
					EAGAIN | ENOMEM => WouldBlock,
					EINTR => Interrupted,
					EIO => Cancelled,

					EBADF => panic!("The input file was not opened for reading or the output file was not opened for writing"),
					EFAULT => panic!("Bad address"),
					EINVAL => panic!("Descriptor is not valid or locked, or an mmap(2)-like operation is not available for in_fd"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}

	#[inline(always)]
	fn write_output_from_file_with_offset<F: AsRef<File>>(&self, from_file: &F, mut offset: i64, maximum_number_of_bytes_to_transfer: usize) -> Result<(usize, i64), StructWriteError>
	{
		if unlikely!(maximum_number_of_bytes_to_transfer == 0)
		{
			return Ok((0, offset))
		}

		let result = unsafe { sendfile(self.as_raw_fd(), from_file.as_ref().as_raw_fd(), &mut offset, maximum_number_of_bytes_to_transfer) };
		if likely!(result >= 0)
		{
			Ok((result as usize, offset))
		}
		else if likely!(result == -1)
		{
			use self::StructWriteError::*;

			Err
			(
				match errno().0
				{
					EAGAIN | ENOMEM => WouldBlock,
					EINTR => Interrupted,
					EIO => Cancelled,

					EBADF => panic!("The input file was not opened for reading or the output file was not opened for writing"),
					EFAULT => panic!("Bad address"),
					EINVAL => panic!("Descriptor is not valid or locked, or an mmap(2)-like operation is not available for in_fd"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}
}
