// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
pub enum ExpressDataPathSocketCreationError
{
	#[allow(missing_docs)]
	CouldNotCreateNetworkDeviceControlSocket(CreationError),
	
	#[allow(missing_docs)]
	CouldNotGetValidNetworkInterfaceName(NetworkDeviceInputOutputControlError<ParseNumberError>),
	
	#[allow(missing_docs)]
	NoSuchNetworkInterfaceName,
	
	#[allow(missing_docs)]
	CouldNotCreateUserMemorySocketFileDescriptor(CreationError),
	
	#[allow(missing_docs)]
	CouldNotCreateUserMemory(CreationError),
	
	#[allow(missing_docs)]
	CouldNotAttachProgram(AttachProgramError),
	
	#[allow(missing_docs)]
	ExpressDataPathSocketBind(SocketBindError),
	
	#[allow(missing_docs)]
	AttachedExpressDataPathProgramNotSuitableForSharing,
	
	#[allow(missing_docs)]
	ChunkSizeDoesNotAccommodateFrameHeadroomAndMaximumTransmissionUnitIncludingFrameCheckSequenceSoLinuxWouldDropPackets
	{
		xdp_packet_headroom: usize,
		
		frame_headroom: FrameHeadroom,
		
		chunk_size: u64,
		
		maximum_transmission_unit_payload_size: MaximumTransmissionUnitPayloadSize,
	},

	#[allow(missing_docs)]
	CouldNotFindAnAcceptableMaximumTransmissionUnit
	{
		xdp_packet_headroom: usize,
		
		frame_headroom: FrameHeadroom,
		
		chunk_size: u64,
		
		reason: ParseNumberError,
	},
	
	#[allow(missing_docs)]
	CouldNotSetAnAcceptableMaximumTransmissionUnit(NetworkDeviceInputOutputControlError<MaximumTransmissionUnitPayloadSizeOutOfRangeError>),
	
	#[allow(missing_docs)]
	CouldNotInsertIntoRedirectMap(InsertError),
}

impl Display for ExpressDataPathSocketCreationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ExpressDataPathSocketCreationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ExpressDataPathSocketCreationError::*;
		
		match self
		{
			&CouldNotCreateNetworkDeviceControlSocket(ref error) => Some(error),
			
			&CouldNotGetValidNetworkInterfaceName(ref error) => Some(error),
			
			&NoSuchNetworkInterfaceName => None,
			
			&CouldNotCreateUserMemorySocketFileDescriptor(ref error) => Some(error),
			
			&CouldNotCreateUserMemory(ref error) => Some(error),
			
			&CouldNotAttachProgram(ref error) => Some(error),
			
			&ExpressDataPathSocketBind(ref error) => Some(error),
			
			&AttachedExpressDataPathProgramNotSuitableForSharing => None,
			
			&ChunkSizeDoesNotAccommodateFrameHeadroomAndMaximumTransmissionUnitIncludingFrameCheckSequenceSoLinuxWouldDropPackets { .. } => None,
			
			&CouldNotFindAnAcceptableMaximumTransmissionUnit { ref reason, .. } => Some(reason),
			
			&CouldNotSetAnAcceptableMaximumTransmissionUnit(ref error) => Some(error),
			
			&CouldNotInsertIntoRedirectMap(ref error) => Some(error),
		}
	}
}

impl From<AttachProgramError> for ExpressDataPathSocketCreationError
{
	#[inline(always)]
	fn from(value: AttachProgramError) -> Self
	{
		ExpressDataPathSocketCreationError::CouldNotAttachProgram(value)
	}
}

impl From<SocketBindError> for ExpressDataPathSocketCreationError
{
	#[inline(always)]
	fn from(value: SocketBindError) -> Self
	{
		ExpressDataPathSocketCreationError::ExpressDataPathSocketBind(value)
	}
}

impl From<NetworkDeviceInputOutputControlError<MaximumTransmissionUnitPayloadSizeOutOfRangeError>> for ExpressDataPathSocketCreationError
{
	#[inline(always)]
	fn from(value: NetworkDeviceInputOutputControlError<MaximumTransmissionUnitPayloadSizeOutOfRangeError>) -> Self
	{
		ExpressDataPathSocketCreationError::CouldNotSetAnAcceptableMaximumTransmissionUnit(value)
	}
}

impl From<InsertError> for ExpressDataPathSocketCreationError
{
	#[inline(always)]
	fn from(value: InsertError) -> Self
	{
		ExpressDataPathSocketCreationError::CouldNotInsertIntoRedirectMap(value)
	}
}

