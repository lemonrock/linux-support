// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateExpressDataPathRedirectSocketMapError
{
	#[allow(missing_docs)]
	NetworkDeviceInputOutputControl(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	MapCreation(MapCreationError),
}

impl Display for CreateExpressDataPathRedirectSocketMapError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for CreateExpressDataPathRedirectSocketMapError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::CreateExpressDataPathRedirectSocketMapError::*;
		
		match self
		{
			&NetworkDeviceInputOutputControl(ref error) => Some(error),
			&MapCreation(ref error) => Some(error),
		}
	}
}

impl From<NetworkDeviceInputOutputControlError<Infallible>> for CreateExpressDataPathRedirectSocketMapError
{
	#[inline(always)]
	fn from(value: NetworkDeviceInputOutputControlError<Infallible>) -> Self
	{
		CreateExpressDataPathRedirectSocketMapError::NetworkDeviceInputOutputControl(value)
	}
}

impl From<MapCreationError> for CreateExpressDataPathRedirectSocketMapError
{
	#[inline(always)]
	fn from(value: MapCreationError) -> Self
	{
		CreateExpressDataPathRedirectSocketMapError::MapCreation(value)
	}
}
