// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Per open file description leasing.
///
/// Leases are relinquished when all open file descriptors for a file are closed.
pub trait Leasing: AsRawFd + Seek + FileExt
{
	/// Acquires a whole of file lease.
	///
	/// An unprivileged process may take out a lease only on a file whose user identifier (owner) matches the filesystem user identifier (fsuid) of the process.
	/// A process with the `CAP_LEASE` capability may take out leases on arbitrary files.
	///
	/// Returns `true` if the lease was acquired or `false` if a signal interrupted whilst waiting to acquire a lease.
	#[inline(always)]
	fn acquire_lease(&self, lease: Lease) -> bool
	{
		let result = unsafe { fcntl(self.as_raw_fd(), F_SETLEASE, lease as i32) };

		if likely!(result == 0)
		{
			true
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				EINTR => false,
				EBADF => panic!("fd is not an open file descriptor"),
				EINVAL => panic!("operation is invalid"),

				unexpected_error @ _ => unexpected_error!(fcntl, unexpected_error),
			}
		}
		else
		{
			unexpected_result!(fcntl, result)
		}
	}

	/// Releases a file lease.
	///
	/// Returns `true` if the lease was released or `false` if a signal interrupted whilst waiting to release a lease.
	#[inline(always)]
	fn release_lease(&self) -> bool
	{
		let result = unsafe { fcntl(self.as_raw_fd(), F_SETLEASE, F_UNLCK) };

		if likely!(result == 0)
		{
			true
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				EINTR => false,
				EBADF => panic!("fd is not an open file descriptor"),
				EINVAL => panic!("operation is invalid"),

				unexpected_error @ _ => unexpected_error!(fcntl, unexpected_error),
			}
		}
		else
		{
			unexpected_result!(fcntl, result)
		}
	}

	/// Releases a file lease.
	///
	/// Returns `Some` if this open file description holds a lease or `None` if it doesn't.
	/// Returns an error if interrupted by a signal.
	#[inline(always)]
	fn get_current_lease(&self) -> Result<Option<Lease>, ()>
	{
		use self::Lease::*;

		match unsafe { fcntl(self.as_raw_fd(), F_GETLEASE) }
		{
			F_RDLCK => Ok(Some(Read)),
			F_WRLCK => Ok(Some(Write)),
			F_UNLCK => Ok(None),

			-1 => match SystemCallErrorNumber::from_errno_panic()
			{
				EINTR => Err(()),
				EBADF => panic!("fd is not an open file descriptor"),
				EINVAL => panic!("operation is invalid"),

				unexpected_error @ _ => unexpected_error!(fcntl, F_GETLEASE, unexpected_error),
			},
			
			unexpected @ _ => unexpected_result!(fcntl, F_GETLEASE, unexpected),
		}
	}
}
