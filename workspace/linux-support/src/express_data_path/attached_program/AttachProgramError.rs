// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
pub enum AttachProgramError
{
	#[allow(missing_docs)]
	NoSuchNetworkInterfaceIndex(NetworkInterfaceIndex),
	
	#[allow(missing_docs)]
	NoSuchNetworkInterfaceNameForNetworkInterfaceIndex(NetworkInterfaceIndexToNetworkInterfaceNameError),
	
	#[allow(missing_docs)]
	CouldNotCreateNetworkDeviceInputOutputControl(CreationError),
	
	#[allow(missing_docs)]
	CouldNotGetNumberOfChannels(NetworkDeviceInputOutputControlError<Infallible>),
	
	#[allow(missing_docs)]
	CreateExpressDataPathRedirectSocketMap(MapCreationError),
	
	#[allow(missing_docs)]
	ProgramLoad(ProgramLoadError),
	
	#[allow(missing_docs)]
	NetlinkSocketOpen(SocketCreationOrBindError),
	
	#[allow(missing_docs)]
	GetLinksUsingNetlink(String),
	
	#[allow(missing_docs)]
	CouldNotGetExistingProgramFileDescriptor(SystemCallErrorNumber),
	
	/// This may be because the program has subsequently been detached; in that sense, not a true error but difficult to handle.
	NoExistingExpressDataPathProgramForAttachedExtendedBpfProgramFileDescriptor,
	
	#[allow(missing_docs)]
	CouldNotGetExistingProgramInformation(SystemCallErrorNumber),
	
	#[allow(missing_docs)]
	ExistingAttachedProgramHasWrongProgramTypeForExpressDataPath,
	
	#[allow(missing_docs)]
	ValidateAttachMode(ValidateAttachModeError),
	
	#[allow(missing_docs)]
	CouldNotGetRehydrateRedirectMap(MapRehydrateError),
	
	#[allow(missing_docs)]
	SocketCreation(CreationError),
	
	#[allow(missing_docs)]
	CouldNotAttachXdpProgram(SystemCallErrorNumber),
}

impl Display for AttachProgramError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for AttachProgramError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::AttachProgramError::*;
		
		match self
		{
			&NoSuchNetworkInterfaceIndex(_) => None,
			
			&NoSuchNetworkInterfaceNameForNetworkInterfaceIndex(ref error) => Some(error),
			
			&CouldNotCreateNetworkDeviceInputOutputControl(ref error) => Some(error),
			
			&CouldNotGetNumberOfChannels(ref error) => Some(error),
			
			&CreateExpressDataPathRedirectSocketMap(ref error) => Some(error),
			
			&ProgramLoad(ref error) => Some(error),
			
			&NetlinkSocketOpen(ref error) => Some(error),
			
			&GetLinksUsingNetlink(_) => None,
			
			&CouldNotGetExistingProgramFileDescriptor(_) => None,
			
			&NoExistingExpressDataPathProgramForAttachedExtendedBpfProgramFileDescriptor => None,
			
			&CouldNotGetExistingProgramInformation(_) => None,
			
			&ExistingAttachedProgramHasWrongProgramTypeForExpressDataPath => None,
			
			&ValidateAttachMode(ref error) => Some(error),
			
			&CouldNotGetRehydrateRedirectMap(ref error) => Some(error),
			
			&SocketCreation(ref error) => Some(error),
			
			&CouldNotAttachXdpProgram(_) => None,
		}
	}
}

impl From<NetworkInterfaceIndexToNetworkInterfaceNameError> for AttachProgramError
{
	#[inline(always)]
	fn from(value: NetworkInterfaceIndexToNetworkInterfaceNameError) -> Self
	{
		AttachProgramError::NoSuchNetworkInterfaceNameForNetworkInterfaceIndex(value)
	}
}

impl From<MapCreationError> for AttachProgramError
{
	#[inline(always)]
	fn from(value: MapCreationError) -> Self
	{
		AttachProgramError::CreateExpressDataPathRedirectSocketMap(value)
	}
}

impl From<ProgramLoadError> for AttachProgramError
{
	#[inline(always)]
	fn from(value: ProgramLoadError) -> Self
	{
		AttachProgramError::ProgramLoad(value)
	}
}

impl From<SocketCreationOrBindError> for AttachProgramError
{
	#[inline(always)]
	fn from(value: SocketCreationOrBindError) -> Self
	{
		AttachProgramError::NetlinkSocketOpen(value)
	}
}

impl From<ValidateAttachModeError> for AttachProgramError
{
	#[inline(always)]
	fn from(value: ValidateAttachModeError) -> Self
	{
		AttachProgramError::ValidateAttachMode(value)
	}
}

impl From<MapRehydrateError> for AttachProgramError
{
	#[inline(always)]
	fn from(value: MapRehydrateError) -> Self
	{
		AttachProgramError::CouldNotGetRehydrateRedirectMap(value)
	}
}

impl From<CreationError> for AttachProgramError
{
	#[inline(always)]
	fn from(value: CreationError) -> Self
	{
		AttachProgramError::SocketCreation(value)
	}
}
