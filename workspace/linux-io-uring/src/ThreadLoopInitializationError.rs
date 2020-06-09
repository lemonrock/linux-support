// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error when initializing.
#[derive(Debug)]
pub enum ThreadLoopInitializationError
{
	#[allow(missing_docs)]
	AcceptConnectionsCoroutineManager(LargeRingQueueCreationError),
	
	#[allow(missing_docs)]
	IoUringSetup(IoUringSetupError),
	
	#[allow(missing_docs)]
	SignalFileDescriptor(CreationError),
	
	#[allow(missing_docs)]
	NewSocketServerListener(NewSocketServerListenerError),
	
	#[allow(missing_docs)]
	CouldNotAllocateAcceptCoroutine(AllocErr),
}

impl Display for ThreadLoopInitializationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<ThreadLoopInitializationError as Debug>::fmt(self, f)
	}
}

impl error::Error for ThreadLoopInitializationError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::ThreadLoopInitializationError::*;

		match self
		{
			&AcceptConnectionsCoroutineManager(ref error) => Some(error),
			
			&IoUringSetup(ref error) => Some(error),
			
			&SignalFileDescriptor(ref error) => Some(error),
			
			&NewSocketServerListener(ref error) => Some(error),
			
			&CouldNotAllocateAcceptCoroutine(ref error) => Some(error),
		}
	}
}

impl From<IoUringSetupError> for ThreadLoopInitializationError
{
	#[inline(always)]
	fn from(error: IoUringSetupError) -> Self
	{
		ThreadLoopInitializationError::IoUringSetup(error)
	}
}

impl From<NewSocketServerListenerError> for ThreadLoopInitializationError
{
	#[inline(always)]
	fn from(error: NewSocketServerListenerError) -> Self
	{
		ThreadLoopInitializationError::NewSocketServerListener(error)
	}
}
