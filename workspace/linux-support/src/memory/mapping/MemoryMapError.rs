// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Memory map error.
#[derive(Debug)]
pub enum MemoryMapError
{
	#[allow(missing_docs)]
	CouldNotMapMemory(CreationError),

	#[allow(missing_docs)]
	CouldNotSetNumaMemoryPolicy,

	#[allow(missing_docs)]
	CouldNotLockMemory(io::Error, MemoryLockSettings),

	#[allow(missing_docs)]
	CouldNotLockAllMappedMemory,

	#[allow(missing_docs)]
	CouldNotApplyMemoryAdvice(io::Error, MemoryAdvice),
}

impl Display for MemoryMapError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for MemoryMapError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::MemoryMapError::*;

		match self
		{
			&CouldNotMapMemory(ref cause) => Some(cause),

			&CouldNotSetNumaMemoryPolicy => None,

			&CouldNotLockMemory(ref cause, ..) => Some(cause),

			&CouldNotLockAllMappedMemory => None,

			&CouldNotApplyMemoryAdvice(ref cause, ..) => Some(cause),
		}
	}
}

impl From<CreationError> for MemoryMapError
{
	#[inline(always)]
	fn from(cause: CreationError) -> Self
	{
		MemoryMapError::CouldNotMapMemory(cause)
	}
}
