// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Change capacity error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ChangeCapacityError
{
	/// The process lacks the capability `CAP_SYS_RESOURCE` to increase the capacity above that in `PipeFileDescriptor::maximum_capacity()`
	PermissionDenied,

	/// Would reduce the capacity of the pipe below that needed for the data currently in use.
	WouldReduceCapacityBelowThatInUse,
}

impl Display for ChangeCapacityError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ChangeCapacityError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ChangeCapacityError::*;

		match self
		{
			PermissionDenied => None,

			WouldReduceCapacityBelowThatInUse => None,
		}
	}
}
