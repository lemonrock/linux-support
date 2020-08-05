// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global socket network configuration error kind.
#[derive(Debug)]
pub enum GlobalSocketConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultSocketBusyRead(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultSocketBusySelectAndPoll(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalMaximumBackLog(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultNotSentLowWater(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalMaximumSendBufferSize(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultSendBufferSize(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalMaximumReceiveBufferSize(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultReceiveBufferSize(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalMaximumControlMessageBufferSize(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeAutoCorking(io::Error),
	
}

impl Display for GlobalSocketConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalSocketConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalSocketConfigurationError::*;

		match self
		{
			&CouldNotChangeGlobalDefaultSocketBusyRead(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultSocketBusySelectAndPoll(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalMaximumBackLog(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultNotSentLowWater(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalMaximumSendBufferSize(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultSendBufferSize(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalMaximumReceiveBufferSize(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultReceiveBufferSize(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalMaximumControlMessageBufferSize(ref cause) => Some(cause),
			
			&CouldNotChangeAutoCorking(ref cause) => Some(cause),
		}
	}
}
