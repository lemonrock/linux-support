// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Logging configuration error kind.
#[derive(Debug)]
pub enum ProcessLoggingConfigurationError
{
	#[allow(missing_docs)]
	StaticLoggingConfiguration(PrintableAsciiCharacterPushError),
	
	#[allow(missing_docs)]
	CouldNotConnectToLocalSyslog(NewSocketClientError),
}

impl Display for ProcessLoggingConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ProcessLoggingConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ProcessLoggingConfigurationError::*;
		
		match self
		{
			&StaticLoggingConfiguration(ref cause) => Some(cause),
			
			&CouldNotConnectToLocalSyslog(ref cause) => Some(cause),
		}
	}
}

impl From<PrintableAsciiCharacterPushError> for ProcessLoggingConfigurationError
{
	#[inline(always)]
	fn from(cause: PrintableAsciiCharacterPushError) -> Self
	{
		ProcessLoggingConfigurationError::StaticLoggingConfiguration(cause)
	}
}

impl From<NewSocketClientError> for ProcessLoggingConfigurationError
{
	#[inline(always)]
	fn from(cause: NewSocketClientError) -> Self
	{
		ProcessLoggingConfigurationError::CouldNotConnectToLocalSyslog(cause)
	}
}
