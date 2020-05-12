// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Submission queue polling thread configuration.
///
/// Using a kernel thread requires the `CAP_SYS_ADMIN` capability.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct LinuxKernelSubmissionQueuePollingThreadConfiguration
{
	/// Must be online.
	pub thread_runs_on: HyperThread,
	
	/// The default for this is currently 100 milliseconds (`#define HZ`).
	pub put_thread_to_sleep_after_milliseconds: Option<NonZeroU32>,
}

impl LinuxKernelSubmissionQueuePollingThreadConfiguration
{
	#[allow(deprecated)]
	#[inline(always)]
	fn configure(configuration: Option<&LinuxKernelSubmissionQueuePollingThreadConfiguration>, mut flags: SetupFlags) -> (u32, u32, SetupFlags)
	{
		match configuration
		{
			None => (unsafe { uninitialized() }, unsafe { uninitialized() }, flags),
			
			Some(&Self { thread_runs_on, put_thread_to_sleep_after_milliseconds }) =>
			{
				flags |= SetupFlags::SubmissionQueuePoll;
				
				let thread_runs_on: u32 = thread_runs_on.into();
				
				let put_thread_to_sleep_after_milliseconds = match put_thread_to_sleep_after_milliseconds
				{
					None => unsafe { uninitialized() },
					
					Some(put_thread_to_sleep_after_milliseconds) =>
					{
						flags |= SetupFlags::SubmissionQueueAffinity;
						
						put_thread_to_sleep_after_milliseconds.get()
					}
				};
				
				(thread_runs_on, put_thread_to_sleep_after_milliseconds, flags)
			}
		}
	}
}
