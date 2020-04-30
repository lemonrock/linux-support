// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error getting or setting a resource limit.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResourceLimitError
{
	/// `MaximumNumberOfFileDescriptors` is set via `/proc/sys/fs/nr_open`.
	PermissionDeniedOrTriedToIncreaseAboveMaximumNumberOfFileDescriptors,

	/// Limit was too large (or bad resource id).
	LimitWasTooLarge,
}

impl Display for ResourceLimitError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ResourceLimitError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ResourceLimitError::*;

		match self
		{
			&PermissionDeniedOrTriedToIncreaseAboveMaximumNumberOfFileDescriptors => None,

			&LimitWasTooLarge => None,
		}
	}
}
