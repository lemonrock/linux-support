// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global network device configuration error kind.
#[derive(Debug)]
pub enum GlobalNetworkDeviceConfigurationError
{
	#[allow(missing_docs)]
	NetworkDeviceDoesNotExist(NetworkInterfaceName),
	
	#[allow(missing_docs)]
	CouldNotSetDriverMessageLevel(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotSetForwardErrorConnection(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotSetPause(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotSetEnergyEfficientEthernet(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotDisableWakeOnLan(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotChangeFeatures(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotChangeTunable(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotChangeCoalesceConfiguration(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotMaximizeChannels(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotMaximizePendingQueueDepths(NetworkDeviceInputOutputControlError<Infallible>),
}

impl Display for GlobalNetworkDeviceConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalNetworkDeviceConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalNetworkDeviceConfigurationError::*;
		
		match self
		{
			&CouldNotSetDriverMessageLevel(ref error) => Some(error),
			
			&CouldNotSetDriverMessageLevel(ref error) => Some(error),
			
			&CouldNotSetForwardErrorConnection(ref error) => Some(error),
			
			&CouldNotSetPause(ref error) => Some(error),
			
			&CouldNotSetEnergyEfficientEthernet(ref error) => Some(error),
			
			&CouldNotDisableWakeOnLan(ref error) => Some(error),
			
			&CouldNotChangeFeatures(ref error) => Some(error),
			
			&CouldNotChangeTunable(ref error) => Some(error),
			
			&CouldNotChangeCoalesceConfiguration(ref error) => Some(error),
			
			&CouldNotMaximizeChannels(ref error) => Some(error),
			
			&CouldNotMaximizePendingQueueDepths(ref error) => Some(error),
		}
	}
}
