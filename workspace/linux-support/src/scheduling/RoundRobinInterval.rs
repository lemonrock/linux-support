// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Round robin interval for a process.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct RoundRobinInterval(Duration);

impl Into<Duration> for RoundRobinInterval
{
	#[inline(always)]
	fn into(self) -> Duration
	{
		self.0
	}
}

impl RoundRobinInterval
{
	/// Returns `None` if the process identifier wasn't for an extant process with permissions.
	#[inline(always)]
	pub fn for_process(process_identifier: ProcessIdentifierChoice) -> Option<Self>
	{
		let mut time = unsafe_uninitialized();
		let result = unsafe { sched_rr_get_interval(process_identifier.into(), &mut time) };
		if likely!(result == 0)
		{
			let duration = Duration::new(time.tv_sec as u64, time.tv_nsec as u32);
			Some(Self(duration))
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				ESRCH => None,

				EFAULT => panic!("Problem with copying information to user space"),
				EINVAL => panic!("Invalid pid"),
				ENOSYS => panic!("The system call is not yet implemented (only on rather old kernels)"),

				unexpected @ _ => unexpected_error!(sched_rr_get_interval, unexpected),
			}
		}
		else
		{
			unexpected_result!(sched_rr_get_interval, result)
		}
	}
}
