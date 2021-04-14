// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An error that can occur when creating `RegisteredMemory`.
#[derive(Debug)]
pub enum RegisteredMemoryCreationError
{
	MappingMemory(CreationError),
	
	CouldNotAdvise(io::Error, MemoryAdvice),
	
	Registration(CreationError),
}

impl Display for RegisteredMemoryCreationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for RegisteredMemoryCreationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::RegisteredMemoryCreationError::*;
		
		match self
		{
			MappingMemory(cause) => Some(cause),
			
			CouldNotAdvise(cause, _) => Some(cause),
			
			Registration(cause) => Some(cause),
		}
	}
}
