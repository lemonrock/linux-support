// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Vectored read common implementation.
macro_rules! vectored_read
{
	() =>
	{
		#[inline(always)]
		fn read_vectored(&self, buffers: &[&mut [u8]]) -> std::io::Result<usize>
		{
			let length = buffers.len();
			debug_assert!(length < crate::vectors::MaximumNumberOfBuffers, "buffer length '{}' equals or exceeds MaximumNumberOfBuffers {}", length, crate::vectors::MaximumNumberOfBuffers);
			if unlikely!(length == 0)
			{
				return Ok(0)
			}

			// NOTE: Relies on iovec having the same layout as a Rust slice.
			let result = unsafe { libc::readv(self.as_raw_fd(), buffers.as_ptr() as *const libc::iovec, buffers.len() as libc::c_int) };

			if likely!(result > 0)
			{
				Ok(result as usize)
			}
			else
			{
				use std::io::ErrorKind::*;

				Err
				(
					std::io::Error::from
					(
						if likely!(result == 0)
						{
							UnexpectedEof
						}
						else if likely!(result == -1)
						{
							match SystemCallErrorNumber::from_errno()
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
								unexpected @ _ => unreachable_code(format_args!("Unexpected error code {}", unexpected)),
							}
						}
						else
						{
							unreachable_code(format_args!("Unexpected result {}", result))
						}
					)
				)
			}
		}
	}
}
