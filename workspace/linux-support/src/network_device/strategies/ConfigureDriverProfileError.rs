// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error.
#[derive(Debug)]
pub enum ConfigureDriverProfileError
{
	/// Should not happen.
	CouldNotConfigureReceiveFlowSteeringCount(io::Error),
	
	#[allow(missing_docs)]
	CouldNotDoGlobalNetworkDeviceConfiguration
	{
		network_interface_name: NetworkInterfaceName,
		
		error: GlobalNetworkDeviceConfigurationError,
	},
	
	/// Should not happen.
	CouldNotConfigureInterruptRequestAffinities(io::Error),
}

impl Display for ConfigureDriverProfileError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ConfigureDriverProfileError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		match self
		{
			&CouldNotConfigureReceiveFlowSteeringCount(ref error) => Some(error),
			
			&CouldNotDoGlobalNetworkDeviceConfiguration { ref error, .. } => Some(error),
			
			&CouldNotConfigureInterruptRequestAffinities(ref error) => Some(error),
		}
	}
}
