// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Master process error.
#[derive(Debug)]
pub enum MasterProcessError
{
	/// Input-output error.
	InputOutput(io::Error),

	/// Could not start child process.
	CouldNotStartChildProcess(CouldNotStartChildProcessError),
}

impl Display for MasterProcessError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for MasterProcessError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::MasterProcessError::*;

		match self
		{
			&InputOutput(ref source) => Some(source),

			&CouldNotStartChildProcess(ref source) => Some(source),
		}
	}
}

impl From<io::Error> for MasterProcessError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		MasterProcessError::InputOutput(value)
	}
}

impl From<CouldNotStartChildProcessError> for MasterProcessError
{
	#[inline(always)]
	fn from(value: CouldNotStartChildProcessError) -> Self
	{
		MasterProcessError::CouldNotStartChildProcess(value)
	}
}
