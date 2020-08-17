// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global network configuration error kind.
#[derive(Debug)]
pub enum GlobalNetworkConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultQueuingDisciplineAlgorithm(io::Error),
	
	#[allow(missing_docs)]
	GlobalReceivePacketSteeringAndReceiveFlowSteeringConfiguration(GlobalReceivePacketSteeringAndReceiveFlowSteeringConfigurationError),
	
	#[allow(missing_docs)]
	GlobalAllNetworkDevicesConfiguration(GlobalAllNetworkDevicesConfigurationError),
	
	#[allow(missing_docs)]
	GlobalNetworkDeviceConfiguration(GlobalNetworkDeviceConfigurationError),
	
	#[allow(missing_docs)]
	GlobalSocketConfiguration(GlobalSocketConfigurationError),
	
	#[allow(missing_docs)]
	GlobalTransmissionControlProtocolConfiguration(GlobalTransmissionControlProtocolConfigurationError),
	
	#[allow(missing_docs)]
	CouldNotChangeMaximumUnixDomainSocketDatagramQueueLength(io::Error),
}

impl Display for GlobalNetworkConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalNetworkConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalNetworkConfigurationError::*;

		match self
		{
			&CouldNotChangeGlobalDefaultQueuingDisciplineAlgorithm(ref cause) => Some(cause),
			
			&GlobalReceivePacketSteeringAndReceiveFlowSteeringConfiguration(ref cause) => Some(cause),
			
			&GlobalAllNetworkDevicesConfiguration(ref cause) => Some(cause),
			
			&GlobalNetworkDeviceConfiguration(ref cause) => Some(cause),
			
			&GlobalSocketConfiguration(ref cause) => Some(cause),
			
			&GlobalTransmissionControlProtocolConfiguration(ref cause) => Some(cause),
			
			&CouldNotChangeMaximumUnixDomainSocketDatagramQueueLength(ref cause) => Some(cause),
		}
	}
}

impl From<GlobalReceivePacketSteeringAndReceiveFlowSteeringConfigurationError> for GlobalNetworkConfigurationError
{
	#[inline(always)]
	fn from(value: GlobalReceivePacketSteeringAndReceiveFlowSteeringConfigurationError) -> Self
	{
		GlobalNetworkConfigurationError::GlobalReceivePacketSteeringAndReceiveFlowSteeringConfiguration(value)
	}
}

impl From<GlobalAllNetworkDevicesConfigurationError> for GlobalNetworkConfigurationError
{
	#[inline(always)]
	fn from(value: GlobalAllNetworkDevicesConfigurationError) -> Self
	{
		GlobalNetworkConfigurationError::GlobalAllNetworkDevicesConfiguration(value)
	}
}

impl From<GlobalNetworkDeviceConfigurationError> for GlobalNetworkConfigurationError
{
	#[inline(always)]
	fn from(value: GlobalNetworkDeviceConfigurationError) -> Self
	{
		GlobalNetworkConfigurationError::GlobalNetworkDeviceConfiguration(value)
	}
}

impl From<GlobalSocketConfigurationError> for GlobalNetworkConfigurationError
{
	#[inline(always)]
	fn from(value: GlobalSocketConfigurationError) -> Self
	{
		GlobalNetworkConfigurationError::GlobalSocketConfiguration(value)
	}
}

impl From<GlobalTransmissionControlProtocolConfigurationError> for GlobalNetworkConfigurationError
{
	#[inline(always)]
	fn from(value: GlobalTransmissionControlProtocolConfigurationError) -> Self
	{
		GlobalNetworkConfigurationError::GlobalTransmissionControlProtocolConfiguration(value)
	}
}
