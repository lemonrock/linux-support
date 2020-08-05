// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global all network devices configuration error kind.
#[derive(Debug)]
pub enum GlobalAllNetworkDevicesConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeWeight(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeWeightReceiveScalar(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeWeightTransmitScalar(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeMaximumNumberOfPacketsInOneNapiPollingCycle(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeMaximumTimeForOneNapiPollingCycle(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeTimestampPrequeue(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeMaximumFragmentsInASocketBuffer(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeNumberOfSocketBufferListsToFreeInAGenericReceiveOffloadBatch(io::Error),
}

impl Display for GlobalAllNetworkDevicesConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalAllNetworkDevicesConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalAllNetworkDevicesConfigurationError::*;

		match self
		{
			&CouldNotChangeWeight(ref cause) => Some(cause),
			
			&CouldNotChangeWeightReceiveScalar(ref cause) => Some(cause),
			
			&CouldNotChangeWeightTransmitScalar(ref cause) => Some(cause),
			
			&CouldNotChangeMaximumNumberOfPacketsInOneNapiPollingCycle(ref cause) => Some(cause),
			
			&CouldNotChangeMaximumTimeForOneNapiPollingCycle(ref cause) => Some(cause),
			
			&CouldNotChangeTimestampPrequeue(ref cause) => Some(cause),
			
			&CouldNotChangeMaximumFragmentsInASocketBuffer(ref cause) => Some(cause),
			
			&CouldNotChangeNumberOfSocketBufferListsToFreeInAGenericReceiveOffloadBatch(ref cause) => Some(cause),
		}
	}
}
