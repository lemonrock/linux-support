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
	CouldNotChangePanicOnNonMaskableInterrupt(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicOnRcuStall(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicOnWarn(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePanicPrint(io::Error),
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
			&CouldNotChangePanicTimeout(ref cause) => Some(error),

			&CouldNotChangePanicOnOops(ref cause) => Some(error),

			&CouldNotChangePanicOnNonMaskableInterrupt(ref cause) => Some(error),

			&CouldNotChangePanicOnRcuStall(ref cause) => Some(error),

			&CouldNotChangePanicOnWarn(ref cause) => Some(error),

			&CouldNotChangePanicPrint(ref cause) => Some(error),
		}
	}
}
