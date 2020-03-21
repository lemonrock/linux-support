// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Could not start child process.
#[derive(Debug)]
pub enum CouldNotStartChildProcessError
{
	/// Input-Output error.
	InputOutput(io::Error),

	/// Could not allocate an anonymous pipe.
	CouldNotAllocateAnonymousPipe(CreationError),

	/// Could not allocate stack.
	CouldNotAllocateStack(AllocErr),

	/// Could not clone.
	CouldNotClone(CloneError),
}

impl Display for CouldNotStartChildProcessError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for CouldNotStartChildProcessError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::CouldNotStartChildProcessError::*;

		match self
		{
			&InputOutput(ref source) => Some(source),

			&CouldNotAllocateAnonymousPipe(ref source) => Some(source),

			&CouldNotAllocateStack(ref source) => Some(source),

			&CouldNotClone(ref source) => Some(source),
		}
	}
}

impl From<io::Error> for CouldNotStartChildProcessError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		CouldNotStartChildProcessError::InputOutput(error)
	}
}

impl From<CreationError> for CouldNotStartChildProcessError
{
	#[inline(always)]
	fn from(error: CreationError) -> Self
	{
		CouldNotStartChildProcessError::CouldNotAllocateAnonymousPipe(error)
	}
}

impl From<AllocErr> for CouldNotStartChildProcessError
{
	#[inline(always)]
	fn from(error: AllocErr) -> Self
	{
		CouldNotStartChildProcessError::CouldNotAllocateStack(error)
	}
}

impl From<CloneError> for CouldNotStartChildProcessError
{
	#[inline(always)]
	fn from(error: CloneError) -> Self
	{
		CouldNotStartChildProcessError::CouldNotClone(error)
	}
}
