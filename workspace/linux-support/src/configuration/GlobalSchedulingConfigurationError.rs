// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global scheduling configuration error kind.
#[derive(Debug)]
pub enum GlobalSchedulingConfigurationError
{
	/// Could not enable or disable autogroup.
	CouldNotChangeAutogroup(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeLatency(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeLatencyScaling(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeMinimumGranularity(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeWakeUpGranularity(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeMigrationCost(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeMigrationCompletelyFairSchedulerBandwidthSlice(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeNumberOfNormalTasksToMigrate(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeStatisticsEnablement(io::Error),

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
	CouldNotChangeRcuGracePeriod(io::Error),
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

			&CouldNotChangeLatency(ref cause) => Some(cause),

			&CouldNotChangeLatencyScaling(ref cause) => Some(cause),

			&CouldNotChangeMinimumGranularity(ref cause) => Some(cause),

			&CouldNotChangeWakeUpGranularity(ref cause) => Some(cause),

			&CouldNotChangeMigrationCost(ref cause) => Some(cause),

			&CouldNotChangeMigrationCompletelyFairSchedulerBandwidthSlice(ref cause) => Some(cause),

			&CouldNotChangeNumberOfNormalTasksToMigrate(ref cause) => Some(cause),

			&CouldNotChangeStatisticsEnablement(ref cause) => Some(cause),

			&CouldNotChangeRoundRobinQuantum(ref cause) => Some(cause),

			&CouldNotChangeReservedCpuTimeForNonRealTimeSchedulerPolicies(ref cause) => Some(cause),

			&CouldNotChangeSoftwareWatchdog(ref cause) => Some(cause),

			&CouldNotChangeHardwareWatchdog(ref cause) => Some(cause),

			&CouldNotChangeHardwareWatchdogThreshold(ref cause) => Some(cause),

			&CouldNotChangeRcuGracePeriod(ref cause) => Some(cause),
		}
	}
}
