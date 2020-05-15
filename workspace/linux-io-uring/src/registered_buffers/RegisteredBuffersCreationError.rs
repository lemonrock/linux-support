// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error when constructing.
#[derive(Debug)]
pub enum RegisteredBuffersCreationError
{
	#[allow(missing_docs)]
	CouldNotCreateLargeRingQueue(LargeRingQueueCreationError),
	
	#[allow(missing_docs)]
	TooManyBuffersNeedToBeCreated,
	
	#[allow(missing_docs)]
	BufferSizeExceeded1GbMaximumSize,
}

impl Display for RegisteredBuffersCreationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<RegisteredBuffersCreationError as Debug>::fmt(self, f)
	}
}

impl error::Error for RegisteredBuffersCreationError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::RegisteredBuffersCreationError::*;

		match self
		{
			&CouldNotCreateLargeRingQueue(ref error) => Some(error),
			
			&TooManyBuffersNeedToBeCreated => None,
			
			&BufferSizeExceeded1GbMaximumSize => None,
		}
	}
}

impl From<LargeRingQueueCreationError> for RegisteredBuffersCreationError
{
	#[inline(always)]
	fn from(error: LargeRingQueueCreationError) -> Self
	{
		RegisteredBuffersCreationError::CouldNotCreateLargeRingQueue(error)
	}
}
