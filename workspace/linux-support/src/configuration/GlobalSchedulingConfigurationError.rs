// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global scheduling configuration error kind.
#[derive(Debug)]
pub enum GlobalSchedulingConfigurationError
{
	/// Could not enable or disable autogroup.
	CouldNotChangeAutogroup(io::Error),

	/// Could not change round-robin quantum.
	CouldNotChangeRoundRobinQuantum(io::Error),

	/// Could not reserved CPU time for non-real time scheduler policies.
	CouldNotChangeReservedCpuTimeForNonRealTimeSchedulerPolicies(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeSoftwareWatchdog(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeHardwareWatchdog(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeHardwareWatchdogThreshold(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeSoftwareAndHardwareWatchdogCpus(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeWorkQueueCpus(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeInterruptRequestDefaultAffinity(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeInterruptRequestAffinity(io::Error, InterruptRequest),
}

impl Display for GlobalSchedulingConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalSchedulingConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalSchedulingConfigurationError::*;

		match self
		{
			&CouldNotChangeAutogroup(ref cause) => Some(cause),

			&CouldNotChangeRoundRobinQuantum(ref cause) => Some(cause),

			&CouldNotChangeReservedCpuTimeForNonRealTimeSchedulerPolicies(ref cause) => Some(cause),

			&CouldNotChangeSoftwareWatchdog(ref cause) => Some(cause),

			&CouldNotChangeHardwareWatchdog(ref cause) => Some(cause),

			&CouldNotChangeHardwareWatchdogThreshold(ref cause) => Some(cause),

			&CouldNotChangeSoftwareAndHardwareWatchdogCpus(ref cause) => Some(cause),

			&CouldNotChangeWorkQueueCpus(ref cause) => Some(cause),

			&CouldNotChangeInterruptRequestDefaultAffinity(ref cause) => Some(cause),

			&CouldNotChangeInterruptRequestAffinity(ref cause, ..) => Some(cause),
		}
	}
}
