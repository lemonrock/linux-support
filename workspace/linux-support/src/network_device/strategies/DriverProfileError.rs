// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error.
#[derive(Debug)]
pub enum DriverProfileError
{
	/// Should not happen.
	FindDriverProfile(FindDriverProfileError),

	/// Should not happen.
	FailedToRetrieveAllPciBuses(io::Error),
	
	/// At least 2 valid (online, etc) HyperThreads are required.
	///
	/// One is used for IRQs for the device (eg Amazon ENA's administration queue).
	/// The remaining are used for receive-transmit queue pairs; there is at least one such pair, hence a minimum of 2.
	AtLeastTwoHyperThreadsAreRequired
	{
		/// Name.
		network_interface_name: NetworkInterfaceName,
		
		/// Zero or One.
		actual_number: usize,
	},
	
	/// Error retrieving the number of channels.
	CouldNotGetNumberOfChannels
	{
		/// Name.
		network_interface_name: NetworkInterfaceName,
		
		/// Error.
		error: NetworkDeviceInputOutputControlError<Infallible>,
	},
	
	/// Does not support retrieving the number of channels; this shouldn't occur if the driver profile is correct.
	DoesNotSupportNumberOfChannels
	{
		/// Name.
		network_interface_name: NetworkInterfaceName,
	},
	
	/// Does not support combined channels; this shouldn't occur if the driver profile is correct.
	DoesNotSupportCombinedChannels
	{
		/// Name.
		network_interface_name: NetworkInterfaceName,
	},
	
	/// This shouldn't occur if the driver profile is correct.
	CouldNotGetNumberOfReceiveRingQueues
	{
		/// Name.
		network_interface_name: NetworkInterfaceName,
		
		/// Error.
		error: NetworkDeviceInputOutputControlError<ParseNumberError>,
	},
	
	/// Error retrieving the RSS hash configuration of channels; this shouldn't occur if the driver profile is correct.
	CouldNotGetReceiveSideScalingHashFunctionConfiguration
	{
		/// Name.
		network_interface_name: NetworkInterfaceName,
		
		/// Error.
		error: NetworkDeviceInputOutputControlError<HashFunctionNameUnsupportedError>,
	},
	
	/// Should not happen.
	IndirectionTableLength(IndirectionTableLengthError),
	
	/// Should not happen.
	DoesNotSupportAHashFunctionIndirectionTable,
	
	/// Should not happen.
	DoesNotSupportAHashFunctionSeed,
	
	/// Should not happen.
	ConfigureDriverProfile(ConfigureDriverProfileError),
}

impl Display for DriverProfileError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for DriverProfileError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		match self
		{
			&FindDriverProfile(ref error) => Some(error),
			
			&FailedToRetrieveAllPciBuses(ref error) => Some(error),
			
			&AtLeastTwoHyperThreadsAreRequired { .. } => None,
			
			&CouldNotGetNumberOfChannels { ref error, ..} => Some(error),
			
			&DoesNotSupportNumberOfChannels { .. } => None,
			
			&DoesNotSupportCombinedChannels { .. } => None,
			
			&CouldNotGetNumberOfReceiveRingQueues { .. } => None,
			
			&CouldNotGetReceiveSideScalingHashFunctionConfiguration { ref error, .. } => Some(error),
			
			&IndirectionTableLength(ref error) => Some(error),
			
			&DoesNotSupportAHashFunctionIndirectionTable => None,
			
			&DoesNotSupportAHashFunctionSeed => None,
			
			&ConfigureDriverProfile(ref error) => Some(error),
		}
	}
}

impl From<FindDriverProfileError> for DriverProfileError
{
	#[inline(always)]
	fn from(value: FindDriverProfileError) -> Self
	{
		FindDriverProfile(value)
	}
}

impl From<IndirectionTableLengthError> for DriverProfileError
{
	#[inline(always)]
	fn from(value: IndirectionTableLengthError) -> Self
	{
		IndirectionTableLength(value)
	}
}

impl From<ConfigureDriverProfileError> for DriverProfileError
{
	#[inline(always)]
	fn from(value: ConfigureDriverProfileError) -> Self
	{
		ConfigureDriverProfile(value)
	}
}
