// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global network device configuration error kind.
#[derive(Debug)]
pub enum GlobalNetworkDeviceConfigurationError
{
	#[allow(missing_docs)]
	NetworkDeviceDoesNotExist(NetworkInterfaceName),
	
	#[allow(missing_docs)]
	NetworkDeviceSocketFileDescriptorCreation(CreationError),
	
	#[allow(missing_docs)]
	CouldNotSetDriverMessageLevel(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotSetLinkFlags(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotSetTransmissionQueueLength(NetworkDeviceInputOutputControlError<TransmissionQueueLengthOutOfRangeError>),
	
	#[allow(missing_docs)]
	CouldNotSetMaximumTransmissionUnit(NetworkDeviceInputOutputControlError<MaximumTransmissionUnitPayloadSizeOutOfRangeError>),
	
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
	CouldNotGetAllStringSets(NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>),
	
	#[allow(missing_docs)]
	CouldNotChangeDriverSpecificFlags(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotChangeTunable(NetworkDeviceInputOutputControlError<TunableOutOfRangeError>),
	
	#[allow(missing_docs)]
	CouldNotChangeCoalesceConfiguration(NetworkDeviceInputOutputControlError<UndocumentedError>),
	
	#[allow(missing_docs)]
	CouldNotChangePerQueueCoalesceConfiguration(NetworkDeviceInputOutputControlError<UndocumentedError>),
	
	#[allow(missing_docs)]
	CouldNotChangeChannels(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotChangePendingQueueDepths(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotConfigureReceiveSideScalingHashConfiguration(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CouldNotConfigureHashFunctionFieldsConfiguration(NetworkDeviceInputOutputControlError<UndocumentedError>),
	
	#[allow(missing_docs)]
	CouldNotSetGenericReceiveOffloadTimeout(io::Error),
	
	#[allow(missing_docs)]
	CouldNotSetNapHardInterruptRequestsCount(io::Error),
	
	#[allow(missing_docs)]
	CouldNotSetPerReceiveQueueReceivePacketSteeringAffinity(io::Error),
	
	#[allow(missing_docs)]
	CouldNotSetPerReceiveQueueReceivePacketSteeringFlowTableCount(io::Error),
	
	#[allow(missing_docs)]
	CouldNotSetPerTransmitQueueMaximumRate(io::Error),
	
	#[allow(missing_docs)]
	CouldNotSetPerTransmitQueueTransmitPacketSteeringHyperThreadAffinity(io::Error),
	
	#[allow(missing_docs)]
	CouldNotSetPerTransmitQueueTransmitPacketSteeringReceiveQueueAffinity(io::Error),
	
	#[allow(missing_docs)]
	CouldNotSetPerTransmitQueueByteLimitsHoldTime(io::Error),
	
	#[allow(missing_docs)]
	CouldNotSetPerTransmitQueueMinimumCurrentByteLimit(io::Error),
	
	#[allow(missing_docs)]
	CouldNotSetPerTransmitQueueMaximumCurrentByteLimit(io::Error),
	
	#[allow(missing_docs)]
	CouldNotSetPerTransmitQueueCurrentByteLimit(io::Error),
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
			&NetworkDeviceDoesNotExist(..) => None,
			
			&NetworkDeviceSocketFileDescriptorCreation(ref error) => Some(error),
			
			&CouldNotSetDriverMessageLevel(ref error) => Some(error),
			
			&CouldNotSetLinkFlags(ref error) => Some(error),
			
			&CouldNotSetTransmissionQueueLength(ref error) => Some(error),
			
			&CouldNotSetMaximumTransmissionUnit(ref error) => Some(error),
			
			&CouldNotSetForwardErrorConnection(ref error) => Some(error),
			
			&CouldNotSetPause(ref error) => Some(error),
			
			&CouldNotSetEnergyEfficientEthernet(ref error) => Some(error),
			
			&CouldNotDisableWakeOnLan(ref error) => Some(error),
			
			&CouldNotChangeFeatures(ref error) => Some(error),
			
			&CouldNotGetAllStringSets(ref error) => Some(error),
			
			&CouldNotChangeDriverSpecificFlags(ref error) => Some(error),
			
			&CouldNotChangeTunable(ref error) => Some(error),
			
			&CouldNotChangeCoalesceConfiguration(ref error) => Some(error),
			
			&CouldNotChangePerQueueCoalesceConfiguration(ref error) => Some(error),
			
			&CouldNotChangeChannels(ref error) => Some(error),
			
			&CouldNotChangePendingQueueDepths(ref error) => Some(error),
			
			&CouldNotConfigureReceiveSideScalingHashConfiguration(ref error) => Some(error),
			
			&CouldNotConfigureHashFunctionFieldsConfiguration(ref error) => Some(error),
			
			&CouldNotSetGenericReceiveOffloadTimeout(ref error) => Some(error),
			
			&CouldNotSetNapHardInterruptRequestsCount(ref error) => Some(error),
			
			&CouldNotSetPerReceiveQueueReceivePacketSteeringAffinity(ref error) => Some(error),
			
			&CouldNotSetPerReceiveQueueReceivePacketSteeringFlowTableCount(ref error) => Some(error),
			
			&CouldNotSetPerTransmitQueueMaximumRate(ref error) => Some(error),
			
			&CouldNotSetPerTransmitQueueTransmitPacketSteeringHyperThreadAffinity(ref error) => Some(error),
			
			&CouldNotSetPerTransmitQueueTransmitPacketSteeringReceiveQueueAffinity(ref error) => Some(error),
			
			&CouldNotSetPerTransmitQueueByteLimitsHoldTime(ref error) => Some(error),
			
			&CouldNotSetPerTransmitQueueMinimumCurrentByteLimit(ref error) => Some(error),
			
			&CouldNotSetPerTransmitQueueMaximumCurrentByteLimit(ref error) => Some(error),
			
			&CouldNotSetPerTransmitQueueCurrentByteLimit(ref error) => Some(error),
		}
	}
}

impl From<CreationError> for GlobalNetworkDeviceConfigurationError
{
	#[inline(always)]
	fn from(value: CreationError) -> Self
	{
		GlobalNetworkDeviceConfigurationError::NetworkDeviceSocketFileDescriptorCreation(value)
	}
}
