// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process capabilities configuration error kind.
#[derive(Debug)]
pub enum ThreadCapabilitiesConfigurationError
{
	#[allow(missing_docs)]
	CouldNotConfigureBoundingSet,

	#[allow(missing_docs)]
	CouldNotConfigurePermittedEffectiveAndInheritableSets(io::Error),

	#[allow(missing_docs)]
	CouldNotConfigureAmbient(AmbientCapabilityError),

	#[allow(missing_docs)]
	CouldNotLockSecureBits(Errno),
}

impl Display for ThreadCapabilitiesConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ThreadCapabilitiesConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ThreadCapabilitiesConfigurationError::*;

		match self
		{
			&CouldNotConfigureBoundingSet => None,

			&CouldNotConfigurePermittedEffectiveAndInheritableSets(ref cause) => Some(cause),

			&CouldNotConfigureAmbient(ref cause) => Some(cause),

			&CouldNotLockSecureBits(..) => None,
		}
	}
}
