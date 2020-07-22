// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A map creation error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MapCreationError
{
	#[allow(missing_docs)]
	KeyIsEmpty,
	
	#[allow(missing_docs)]
	KeyIsTooLarge,
	
	#[allow(missing_docs)]
	ValueIsEmpty,
	
	#[allow(missing_docs)]
	ValueIsTooLarge,
	
	#[allow(missing_docs)]
	HashByDeviceArrayByDeviceAndStructOpsMandatesThatThereAreNotKeyOrValueTypeIdentifiers,
	
	#[allow(missing_docs)]
	SocketStorageMandatesBpfTypeFormatTypeIdentifiersForKeyAndValue,

	#[allow(missing_docs)]
	MissingMapFileDescriptor(FileDescriptorLabelsMapError),

	#[allow(missing_docs)]
	CreateFailed(Errno),
}

impl Display for MapCreationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for MapCreationError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::MapCreationError::*;
		
		match self
		{
			&KeyIsTooLarge => None,
			
			&KeyIsEmpty => None,
			
			&ValueIsTooLarge => None,
			
			&ValueIsEmpty => None,
			
			&HashByDeviceArrayByDeviceAndStructOpsMandatesThatThereAreNotKeyOrValueTypeIdentifiers => None,
			
			&SocketStorageMandatesBpfTypeFormatTypeIdentifiersForKeyAndValue => None,
			
			&MissingMapFileDescriptor(ref cause) => Some(cause),
			
			&CreateFailed(_) => None,
		}
	}
}

impl From<FileDescriptorLabelsMapError> for MapCreationError
{
	#[inline(always)]
	fn from(value: FileDescriptorLabelsMapError) -> Self
	{
		MapCreationError::MissingMapFileDescriptor(value)
	}
}
