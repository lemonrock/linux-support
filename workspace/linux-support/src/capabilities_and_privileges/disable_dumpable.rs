// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Needs to be called after process change.
#[inline(always)]
pub fn disable_dumpable() -> Result<(), io::Error>
{
	let result = unsafe { prctl(PR_SET_DUMPABLE, 0 as c_ulong) };
	if likely!(result == 0)
	{
		Ok(())
	}
	else if likely!(result == -1)
	{
		Err(io::Error::last_os_error())
	}
	else
	{
		unreachable!("Unexpected result {} from prctl()", result)
	}
}
