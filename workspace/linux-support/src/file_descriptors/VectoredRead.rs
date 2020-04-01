// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Implementors support vectored reads.
pub trait VectoredRead: AsRawFd
{
	/// Performs a vectored read.
	///
	/// All buffers should be non-zero sized otherwise the results are ambiguous (ie was nothing read or is this end-of-file).
	#[inline(always)]
	fn read_vectored(&self, buffers: &[&mut [u8]]) -> io::Result<usize>
	{
		let length = buffers.len();
		if unlikely!(length == 0)
		{
			return Ok(0)
		}

		// NOTE: Relies on iovec having the same layout as a Rust slice.
		let result = unsafe { readv(self.as_raw_fd(), buffers.as_ptr() as *const iovec, buffers.len() as c_int) };

		if likely!(result > 0)
		{
			Ok(result as usize)
		}
		else
		{
			use self::ErrorKind::*;

			Err
			(
				io::Error::from
				(
					if likely!(result == 0)
					{
						UnexpectedEof
					}
					else if likely!(result == -1)
					{
						match errno().0
						{
							EAGAIN => WouldBlock,
							EINTR => Interrupted,
							ENOMEM => Other,
							ECONNRESET => ConnectionReset,
							ECONNREFUSED => ConnectionRefused,
							EBADF => panic!("The argument `sockfd` is an invalid descriptor"),
							EFAULT => panic!("The receive buffer pointer(s) point outside the process's address space"),
							EINVAL => panic!("Invalid argument passed"),
							ENOTCONN => panic!("The socket is associated with a connection-oriented protocol and has not been connected"),
							ENOTSOCK => panic!("The argument `sockfd` does not refer to a socket"),
							EOPNOTSUPP => panic!("Some flags in the `flags` argument are inappropriate for the socket type"),
							_ => unreachable!(),
						}
					}
					else
					{
						unreachable!()
					}
				)
			)
		}
	}
}
