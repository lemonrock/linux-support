// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn process_control_wrapper<V, E>(operation: i32, arg2: usize, arg3: usize, arg4: usize, arg5: usize, ok_handler: impl FnOnce(i32) -> Result<V, E>, err_handler: impl FnOnce(Errno) -> Result<V, E>) -> Result<V, E>
{
	let result = unsafe { prctl(operation, arg2, arg3, arg4, arg5) };
	if likely!(result >= 0)
	{
		ok_handler(result)
	}
	else if likely!(result == -1)
	{
		err_handler(errno())
	}
	else
	{
		unreachable!("Unexpected result {} from prctl()", result)
	}
}
