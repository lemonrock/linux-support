// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global scheduling configuration error kind.
#[derive(Debug)]
pub enum GlobalComputedSchedulingConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeSoftwareAndHardwareWatchdogCpus(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeWorkQueueCpus(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeInterruptRequestDefaultAffinity(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeInterruptRequestAffinity(io::Error, InterruptRequest),
	
	#[allow(missing_docs)]
	CouldNotChangeWhichHyperThreadsHaveReceivePacketSteeringFlowLimitTablesEnabled(io::Error),
}

impl Display for GlobalComputedSchedulingConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalComputedSchedulingConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalComputedSchedulingConfigurationError::*;

		match self
		{
			&CouldNotChangeSoftwareAndHardwareWatchdogCpus(ref cause) => Some(cause),

			&CouldNotChangeWorkQueueCpus(ref cause) => Some(cause),

			&CouldNotChangeInterruptRequestDefaultAffinity(ref cause) => Some(cause),

			&CouldNotChangeInterruptRequestAffinity(ref cause, ..) => Some(cause),
			
			&CouldNotChangeWhichHyperThreadsHaveReceivePacketSteeringFlowLimitTablesEnabled(ref cause) => Some(cause),
		}
	}
}
