// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Errors during dispatch of io-uring operations.
#[derive(Debug)]
pub enum DispatchIoUringError<NonCoroutineHandlerError: error::Error>
{
	#[allow(missing_docs)]
	NonCoroutine(NonCoroutineHandlerError),
}

impl<NonCoroutineHandlerError: error::Error> Display for DispatchIoUringError<NonCoroutineHandlerError>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<IoUringSetupError as Debug>::fmt(self, f)
	}
}

impl<NonCoroutineHandlerError: error::Error> error::Error for DispatchIoUringError<NonCoroutineHandlerError>
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::DispatchIoUringError::*;

		match self
		{
			&NonCoroutine(ref error) => Some(error),
		}
	}
}

impl<NonCoroutineHandlerError: error::Error> From<NonCoroutineHandlerError> for DispatchIoUringError<NonCoroutineHandlerError>
{
	#[inline(always)]
	fn from(error: NonCoroutineHandlerError) -> Self
	{
		DispatchIoUringError::NonCoroutine(error)
	}
}
