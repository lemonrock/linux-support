// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
pub enum LoadOwnedMemoryProgramError
{
	#[allow(missing_docs)]
	NoSuchNetworkInterfaceName(NetworkInterfaceName),
	
	#[allow(missing_docs)]
	CreateExpressDataPathRedirectSocketMap(MapCreationError),
	
	#[allow(missing_docs)]
	ProgramLoad(ProgramLoadError),
	
	#[allow(missing_docs)]
	NetlinkSocketOpen(SocketCreationOrBindError),
	
	#[allow(missing_docs)]
	GetLinksUsingNetlink(String),
	
	#[allow(missing_docs)]
	CouldNotGetExistingProgramFileDescriptor(Errno),
	
	/// This may be because the program has subsequently been detached; in that sense, not a true error but difficult to handle.
	NoExistingExpressDataPathProgramForAttachedExtendedBpfProgramFileDescriptor,
	
	#[allow(missing_docs)]
	CouldNotGetExistingProgramInformation(Errno),
	
	#[allow(missing_docs)]
	ExistingAttachedProgramHasWrongProgramTypeForExpressDataPath,
	
	#[allow(missing_docs)]
	ValidateAttachMode(ValidateAttachModeError),
	
	#[allow(missing_docs)]
	CouldNotGetRehydrateMap(MapRehydrateError),
}

impl Display for LoadOwnedMemoryProgramError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for LoadOwnedMemoryProgramError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::LoadOwnedMemoryProgramError::*;
		
		match self
		{
			&NoSuchNetworkInterfaceName(_) => None,
			
			&CreateExpressDataPathRedirectSocketMap(ref error) => Some(error),
			
			&ProgramLoad(ref error) => Some(error),
			
			&NetlinkSocketOpen(ref error) => Some(error),
			
			&GetLinksUsingNetlink(_) => None,
			
			&CouldNotGetExistingProgramFileDescriptor(_) => None,
			
			&NoExistingExpressDataPathProgramForAttachedExtendedBpfProgramFileDescriptor => None,
			
			&CouldNotGetExistingProgramInformation(_) => None,
			
			&ExistingAttachedProgramHasWrongProgramTypeForExpressDataPath => None,
			
			&ValidateAttachMode(ref error) => Some(error),
			
			&CouldNotGetRehydrateMap(ref error) => Some(error),
		}
	}
}

impl From<MapCreationError> for LoadOwnedMemoryProgramError
{
	#[inline(always)]
	fn from(value: MapCreationError) -> Self
	{
		LoadOwnedMemoryProgramError::CreateExpressDataPathRedirectSocketMap(value)
	}
}

impl From<ProgramLoadError> for LoadOwnedMemoryProgramError
{
	#[inline(always)]
	fn from(value: ProgramLoadError) -> Self
	{
		LoadOwnedMemoryProgramError::ProgramLoad(value)
	}
}

impl From<SocketCreationOrBindError> for LoadOwnedMemoryProgramError
{
	#[inline(always)]
	fn from(value: SocketCreationOrBindError) -> Self
	{
		LoadOwnedMemoryProgramError::NetlinkSocketOpen(value)
	}
}

impl From<ValidateAttachModeError> for LoadOwnedMemoryProgramError
{
	#[inline(always)]
	fn from(value: ValidateAttachModeError) -> Self
	{
		LoadOwnedMemoryProgramError::ValidateAttachMode(value)
	}
}

impl From<MapRehydrateError> for LoadOwnedMemoryProgramError
{
	#[inline(always)]
	fn from(value: MapRehydrateError) -> Self
	{
		LoadOwnedMemoryProgramError::CouldNotGetRehydrateMap(value)
	}
}
