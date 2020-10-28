// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Remove I/O port privileges.
#[inline(always)]
pub fn remove_ioport_privileges()
{
	let result = unsafe { iopl(0) };
	if likely!(result == 0)
	{
	}
	else if likely!(result == -1)
	{
		match errno().0
		{
			EINVAL => panic!("level is greater than 3"),
			ENOSYS => panic!("This call is unimplemented (it should be)"),
			EPERM => panic!("The calling process has insufficient privilege to call iopl(); the CAP_SYS_RAWIO capability is required to raise the I/O privilege level above its current value."),

			unexpected @ _ => panic!("Unexpected error {} from iopl()", unexpected),
		}
	}
	else
	{
		unreachable_code(format_args!("Unexpected result {} from iopl()", result))
	}
}
