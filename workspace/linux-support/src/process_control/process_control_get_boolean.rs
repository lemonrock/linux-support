// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn process_control_get_boolean(operation: i32) -> io::Result<bool>
{
	process_control_wrapper1
	(
		operation,
		|non_negative_result| match non_negative_result
		{
			0 => Ok(false),
			1 => Ok(true),
			_ => Err(io_error_invalid_data(format!("Non-boolean result `{}` from `prctl()`", non_negative_result))),
		},
		|error_number| Err(io_error_other(format!("Error result `{}` from `prctl()`", error_number))),
	)
}
