// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error when setting up.
#[derive(Debug)]
pub enum IoUringSetupError
{
	#[allow(missing_docs)]
	IoUringCreation(IoUringCreationError),
	
	#[allow(missing_docs)]
	RegisteredBuffersCreation(RegisteredBuffersCreationError),
	
	#[allow(missing_docs)]
	RegisteringBuffers(io::Error),
}

impl Display for IoUringSetupError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<IoUringSetupError as Debug>::fmt(self, f)
	}
}

impl error::Error for IoUringSetupError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::IoUringSetupError::*;

		match self
		{
			&IoUringCreation(ref error) => Some(error),
			
			&RegisteredBuffersCreation(ref error) => Some(error),
			
			&RegisteringBuffers(ref error) => Some(error),
		}
	}
}

impl From<IoUringCreationError> for IoUringSetupError
{
	#[inline(always)]
	fn from(error: IoUringCreationError) -> Self
	{
		IoUringSetupError::IoUringCreation(error)
	}
}

impl From<RegisteredBuffersCreationError> for IoUringSetupError
{
	#[inline(always)]
	fn from(error: RegisteredBuffersCreationError) -> Self
	{
		IoUringSetupError::RegisteredBuffersCreation(error)
	}
}
