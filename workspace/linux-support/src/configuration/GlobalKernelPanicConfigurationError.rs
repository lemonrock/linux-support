// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global kernel panic configuration error kind.
#[derive(Debug)]
pub enum GlobalKernelPanicConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangePanicTimeout(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicOnOops(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicOnRcuStall(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicOnWarn(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicOnIoNonMaskableInterrupt(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicOnUnknownNonMaskableInterrupt(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicOnUnrecoverableNonMaskableInterrupt(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicOnStackOverflow(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicOnHungTask(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicOnMemoryFailure(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicOnSoftwareWatchdogLockup(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeSoftwareWatchdogLockupDebugInformation(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicOnHardwareWatchdogLockup(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeHardwareWatchdogLockupDebugInformation(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicPrint(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeReportPanicDataToHyperV(io::Error),
}

impl Display for GlobalKernelPanicConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalKernelPanicConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalKernelPanicConfigurationError::*;

		match self
		{
			&CouldNotChangePanicTimeout(ref cause) => Some(cause),

			&CouldNotChangePanicOnOops(ref cause) => Some(cause),

			&CouldNotChangePanicOnRcuStall(ref cause) => Some(cause),

			&CouldNotChangePanicOnWarn(ref cause) => Some(cause),

			&CouldNotChangePanicOnIoNonMaskableInterrupt(ref cause) => Some(cause),

			&CouldNotChangePanicOnUnknownNonMaskableInterrupt(ref cause) => Some(cause),

			&CouldNotChangePanicOnUnrecoverableNonMaskableInterrupt(ref cause) => Some(cause),

			&CouldNotChangePanicOnStackOverflow(ref cause) => Some(cause),

			&CouldNotChangePanicOnHungTask(ref cause) => Some(cause),

			&CouldNotChangePanicOnMemoryFailure(ref cause) => Some(cause),

			&CouldNotChangePanicOnSoftwareWatchdogLockup(ref cause) => Some(cause),

			&CouldNotChangeSoftwareWatchdogLockupDebugInformation(ref cause) => Some(cause),

			&CouldNotChangePanicOnHardwareWatchdogLockup(ref cause) => Some(cause),

			&CouldNotChangeHardwareWatchdogLockupDebugInformation(ref cause) => Some(cause),

			&CouldNotChangePanicPrint(ref cause) => Some(cause),

			&CouldNotChangeReportPanicDataToHyperV(ref cause) => Some(cause),
		}
	}
}
