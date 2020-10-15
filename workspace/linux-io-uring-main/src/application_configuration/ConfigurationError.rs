// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) enum ConfigurationError
{
	CouldNotOpenConfigurationFile(io::Error),
	
	CouldNotDeserializeJsonConfigurationFile(serde_json::Error),
	
	ProcessConfiguration(ProcessConfigurationError),
	
	DriverProfile(DriverProfileError),
	
	ProcessExecutor(ProcessExecutorError),
}

impl Display for ConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ConfigurationError::*;
		
		match self
		{
			&CouldNotOpenConfigurationFile(ref error) => Some(error),
			
			&CouldNotDeserializeJsonConfigurationFile(ref error) => Some(error),
			
			&ProcessConfiguration(ref error) => Some(error),
			
			&DriverProfile(ref error) => Some(error),
			
			&ProcessExecutor(ref error) => Some(error),
		}
	}
}

impl From<ProcessConfigurationError> for ConfigurationError
{
	#[inline(always)]
	fn from(value: ProcessConfigurationError) -> Self
	{
		ConfigurationError::ProcessConfiguration(value)
	}
}

impl From<DriverProfileError> for ConfigurationError
{
	#[inline(always)]
	fn from(value: DriverProfileError) -> Self
	{
		ConfigurationError::DriverProfile(value)
	}
}

impl From<ProcessExecutorError> for ConfigurationError
{
	#[inline(always)]
	fn from(value: ProcessExecutorError) -> Self
	{
		ConfigurationError::ProcessExecutor(value)
	}
}
