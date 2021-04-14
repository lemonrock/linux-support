// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Creation error.
#[derive(Debug)]
pub enum BlockingUserFaultFileDescriptorCreationError<UFFDE: error::Error>
{
	/// Creation.
	Creation(CreationError),
	
	/// Wrapper.
	Wrapper(UFFDE),
	
	/// Thread spawn.
	ThreadSpawn(io::Error),
}

impl<UFFDE: error::Error> Display for BlockingUserFaultFileDescriptorCreationError<UFFDE>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<UFFDE: error::Error> error::Error for BlockingUserFaultFileDescriptorCreationError<UFFDE>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::BlockingUserFaultFileDescriptorCreationError::*;
		
		match self
		{
			Creation(ref cause) => Some(cause),
			
			Wrapper(ref cause) => Some(cause),
			
			ThreadSpawn(ref cause) => Some(cause),
		}
	}
}

impl<UFFDE: error::Error> From<CreationError> for BlockingUserFaultFileDescriptorCreationError<UFFDE>
{
	#[inline(always)]
	fn from(from: CreationError) -> Self
	{
		BlockingUserFaultFileDescriptorCreationError::Creation(from)
	}
}

impl<UFFDE: error::Error> From<io::Error> for BlockingUserFaultFileDescriptorCreationError<UFFDE>
{
	#[inline(always)]
	fn from(from: io::Error) -> Self
	{
		BlockingUserFaultFileDescriptorCreationError::ThreadSpawn(from)
	}
}
