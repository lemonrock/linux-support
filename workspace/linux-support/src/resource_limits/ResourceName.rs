// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Name for a resource which has a soft and a hard limit.
#[allow(missing_docs)]
#[repr(i32)]
#[derive(Deserialize, Serialize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResourceName
{
	MaximumSizeOfVirtualMemoryAddressSpaceInBytes = RLIMIT_AS,

	MaximumSizeOfACoreDumpFileInBytes = RLIMIT_CORE,

	CpuTimeLimitInSeconds = RLIMIT_CPU,

	MaximumSizeOfProcessDataSegmentInBytes = RLIMIT_DATA,

	/// Ignored on Linux kernels after 2.4.30.
	MaximumSizeOfProcessResidentSetSizeInBytes = RLIMIT_RSS,

	MaximumSizeOfProcessStackInBytes = RLIMIT_STACK,

	MaximumSizeOfFilesThatProcessCanCreateInBytes = RLIMIT_FSIZE,

	MaximumNumberOfBytesThatProcessCanMemLock = RLIMIT_MEMLOCK,

	MaximumNumberOfBytesForPosixMessageQueues = RLIMIT_MSGQUEUE,

	NiceCeilingLargerIsBetter = RLIMIT_NICE,

	RealTimePriorityCeilingLargerIsBetter = RLIMIT_RTPRIO,

	MaximumNumberOfFileDescriptors = RLIMIT_NOFILE,

	MaximumNumberOfProcessAndThreads = RLIMIT_NPROC,

	RealTimePriorityLimitInMicroseconds = RLIMIT_RTTIME,

	MaximumNumberOfSignalsPending = RLIMIT_SIGPENDING,
}

impl ResourceName
{
	/// Sets a resource to a hard and soft limit.
	///
	/// Panics if it can not be set.
	#[inline(always)]
	pub fn set(&self, soft_and_hard_resource_limit: &SoftAndHardResourceLimit)
	{
		soft_and_hard_resource_limit.set(*self as i32)
	}

	/// Gets a resources hard and soft limits.
	///
	/// Panics if they can not be obtained.
	#[inline(always)]
	pub fn get(&self) -> SoftAndHardResourceLimit
	{
		SoftAndHardResourceLimit::get(*self as i32)
	}
}
