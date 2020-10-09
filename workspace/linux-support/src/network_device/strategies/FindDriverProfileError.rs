// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error.
#[derive(Debug)]
pub enum FindDriverProfileError
{
	/// Should not happen.
	FailedToRetrieveLinuxKernelVersion(io::Error),
	
	/// Should not happen.
	FailedToObtainAllNetworkInterfaceNames(String),
	
	/// This should not be possible.
	CouldNotCreateNetworkDeviceInputOutputControl
	{
		/// Error.
		error: CreationError,
		
		/// Name.
		network_interface_name: NetworkInterfaceName,
	},
	
	/// This should not be possible.
	CouldNotGetDriverAndDeviceInformation
	{
		error: NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>,
		
		/// Name.
		network_interface_name: NetworkInterfaceName,
	},
	
	/// This should not be possible.
	CouldNotGetPciDevice
	{
		/// Error.
		error: NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>,
		
		/// Name.
		network_interface_name: NetworkInterfaceName,
	},
	
	/// The provided NetworkInterfaceName has no matching NetworkInterfaceIndex implying it does not exist.
	NoSuchNetworkInterface
	{
		/// Name.
		network_interface_name: NetworkInterfaceName,
	},
}

impl Display for FindDriverProfileError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for FindDriverProfileError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		match self
		{
			&FailedToRetrieveLinuxKernelVersion(ref error) => Some(error),
			
			&FailedToObtainAllNetworkInterfaceNames(..) => None,
			
			&CouldNotCreateNetworkDeviceInputOutputControl { ref error, .. } => Some(error),
			
			&CouldNotGetDriverAndDeviceInformation { ref error, .. } => Some(error),
			
			&CouldNotGetPciDevice { ref error, .. } => Some(error),
			
			&NoSuchNetworkInterface { .. } => None,
		}
	}
}
