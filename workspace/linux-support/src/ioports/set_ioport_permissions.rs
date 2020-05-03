// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Remove I/O port permissions.
///
/// Ports above 0x3FF might not be supported.
///
/// If `enable` is `true` then the calling thread must have the `CAP_SYS_RAWIO` capability.
#[cfg(not(target_arch = "powerpc64"))]
#[inline(always)]
fn set_ioport_permissions(range: RangeInclusive<u16>, enable: bool) -> Result<(), &'static str>
{
	let start = *range.start();
	let end = *range.end();
	let result = unsafe { ioperm(start, end - start + 1, enable as i32) };
	if likely!(result == 0)
	{
		Ok(())
	}
	else if likely!(result == -1)
	{
		match errno().0
		{
			EINVAL => Err("Invalid values for from or num"),
			EPERM => Err("The calling thread has insufficient privilege"),
			ENOMEM => panic!("Out of memory"),

			unexpected @ _ => panic!("Unexpected error {} from ioperm()", unexpected),
		}
	}
	else
	{
		unreachable!("Unexpected result {} from ioperm()", result)
	}
}
