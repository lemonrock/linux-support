// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// ***SLOW*** as it uses a syscall whose results than have to be parsed by libc!
#[inline(always)]
pub fn system_information() -> sysinfo
{
	let mut system_information = unsafe_uninitialized();
	let result = unsafe { sysinfo(&mut system_information) };
	if likely!(result == 0)
	{
		system_information
	}
	else if likely!(result == -1)
	{
		match SystemCallErrorNumber::from_errno()
		{
			EFAULT => panic!("info is not a valid address"),
			
			unexpected @ _ => unreachable_code(format_args!("Unexpected error {} from sysinfo()", unexpected)),
		}
	}
	else
	{
		unreachable_code(format_args!("Unexpected result {} from sysinfo()", result));
	}
}
