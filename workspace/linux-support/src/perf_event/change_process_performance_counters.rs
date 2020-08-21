// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Will fail with `EINVAL` if the Linux kernel was not compiled with `CONFIG_PERF_EVENTS`.
#[inline(always)]
pub fn change_process_performance_counters(enable_or_disable_process_performance_counters: bool) -> Result<(), Errno>
{
	let command = if enable_or_disable_process_performance_counters
	{
		PR_TASK_PERF_EVENTS_ENABLE
	}
	else
	{
		PR_TASK_PERF_EVENTS_DISABLE
	};
	
	process_control_wrapper1
	(
		command,
		|non_negative_result| if likely!(non_negative_result == 0)
		{
			Ok(())
		}
		else
		{
			unreachable!("Positive result")
		},
		|error_number| Err(error_number),
	)
}
