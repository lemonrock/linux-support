// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Vectored write common implementation.
macro_rules! vectored_write
{
	() =>
	{
		#[inline(always)]
		fn write_vectored(&self, buffers: &[&[u8]]) -> std::io::Result<usize>
		{
			let length = buffers.len();
			debug_assert!(length < crate::vectors::MaximumNumberOfBuffers, "buffer length '{}' equals or exceeds MaximumNumberOfBuffers {}", length, crate::vectors::MaximumNumberOfBuffers);
			if buffers.len() == 0
			{
				return Ok(0)
			}

			// NOTE: Relies on iovec having the same layout as a Rust slice.
			let result = unsafe { libc::writev(self.as_raw_fd(), buffers.as_ptr() as *const libc::iovec, buffers.len() as libc::c_int) };
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
							WriteZero
						}
						else if likely!(result == -1)
						{
							match errno().0
							{
								libc::EAGAIN => WouldBlock,
								libc::EINTR => Interrupted,
								libc::ENOMEM | ENOBUFS => Other,
								libc::EPIPE => BrokenPipe,
								libc::EACCES => PermissionDenied,
								libc::ECONNRESET => ConnectionReset,
								libc::EBADF => panic!("The argument `sockfd` is an invalid descriptor"),
								libc::EFAULT => panic!("The receive buffer pointer(s) point outside the process's address space"),
								libc::EINVAL => panic!("Invalid argument passed"),
								libc::ENOTCONN => panic!("The socket is associated with a connection-oriented protocol and has not been connected"),
								libc::ENOTSOCK => panic!("The argument `sockfd` does not refer to a socket"),
								libc::EOPNOTSUPP => panic!("Some flags in the `flags` argument are inappropriate for the socket type"),
								libc::EMSGSIZE => panic!("The socket type requires that message be sent atomically, and the size of the message to be sent made this impossible"),
								libc::EISCONN => panic!("The connection-mode socket was connected already but a recipient was specified"),
								libc::EDESTADDRREQ => panic!("The socket is not connection-mode, and no peer address is set"),
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
}
