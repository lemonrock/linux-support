// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Creation error.
#[derive(Debug)]
pub enum NonBlockingUserFaultFileDescriptorCreationError<ValidationError: error::Error + 'static>
{
	/// Creation.
	Creation(CreationError),

	/// Validation.
	Validation(ValidationError),
}

impl<ValidationError: error::Error + 'static> Display for NonBlockingUserFaultFileDescriptorCreationError<ValidationError>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<ValidationError: error::Error + 'static> error::Error for NonBlockingUserFaultFileDescriptorCreationError<ValidationError>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::NonBlockingUserFaultFileDescriptorCreationError::*;
		
		match self
		{
			Creation(ref cause) => Some(cause),
			
			Validation(ref cause) => Some(cause),
		}
	}
}

impl<ValidationError: error::Error + 'static> From<CreationError> for NonBlockingUserFaultFileDescriptorCreationError<ValidationError>
{
	#[inline(always)]
	fn from(from: CreationError) -> Self
	{
		NonBlockingUserFaultFileDescriptorCreationError::Creation(from)
	}
}
