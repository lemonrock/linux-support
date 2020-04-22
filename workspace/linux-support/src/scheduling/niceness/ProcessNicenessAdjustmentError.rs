// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process niceness error.
#[derive(Debug)]
pub enum ProcessNicenessAdjustmentError
{
	/// Could not set current real effective user priority niceness (permission was denied in some way).
	CouldNotSetCurrentUserPriorityNiceness,

	/// Could not set current process group user priority niceness (permission was denied in some way).
	CouldNotSetCurrentProcessGroupPriorityNiceness,

	/// Could not set current process user priority niceness (permission was denied in some way).
	CouldNotSetCurrentProcessPriorityNiceness,

	/// Could not set current process user autogroup priority niceness.
	CouldNotSetCurrentProcessAutogroupPriorityNiceness(io::Error),
}

impl Display for ProcessNicenessAdjustmentError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ProcessNicenessAdjustmentError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ProcessNicenessAdjustmentError::*;

		match self
		{
			&CouldNotSetCurrentUserPriorityNiceness => None,

			&CouldNotSetCurrentProcessGroupPriorityNiceness => None,

			&CouldNotSetCurrentProcessPriorityNiceness => None,

			&CouldNotSetCurrentProcessAutogroupPriorityNiceness(ref error) => Some(error),
		}
	}
}

impl From<io::Error> for ProcessNicenessAdjustmentError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		ProcessNicenessAdjustmentError::CouldNotSetCurrentProcessAutogroupPriorityNiceness(error)
	}
}
