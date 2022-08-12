// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn new_socket(domain: c_int, type_: c_int, protocol: c_int, non_blocking: bool) -> Result<RawFd, CreationError>
{
	let flags = if non_blocking
	{
		type_ | SOCK_CLOEXEC | SOCK_NONBLOCK
	}
	else
	{
		type_ | SOCK_CLOEXEC
	};
	
	let result = unsafe { socket(domain, flags, protocol) };
	if likely!(result >= 0)
	{
		Ok(result)
	}
	else if likely!(result == -1)
	{
		use self::CreationError::*;

		Err
		(
			match SystemCallErrorNumber::from_errno()
			{
				EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
				ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,
				ENOBUFS | ENOMEM => KernelWouldBeOutOfMemory,
				EINVAL => panic!("Invalid arguments"),
				EACCES => panic!("Permission denied"),
				EAFNOSUPPORT => panic!("The implementation does not support the specified address family"),
				EPROTONOSUPPORT => panic!("The protocol type or the specified protocol is not supported within this domain"),
				_ => unreachable_code(format_args!("")),
			}
		)
	}
	else
	{
		unreachable_code(format_args!("Unexpected result {} from socket()", result))
	}
}
