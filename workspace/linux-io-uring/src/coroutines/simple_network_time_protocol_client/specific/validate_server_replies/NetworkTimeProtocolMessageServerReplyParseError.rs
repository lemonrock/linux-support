// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) enum NetworkTimeProtocolMessageServerReplyParseError
{
	IoErrorOnReceive(io::Error),
	
	InvalidLength(usize),
	
	NetworkTimeProtocolMessageServerReplyFirstByteParse(NetworkTimeProtocolMessageServerReplyFirstByteParseError),
	
	NetworkTimeProtocolMessageServerReplyPacketStratumParse(NetworkTimeProtocolMessageServerReplyPacketStratumParseError),
	
	/// If another server is available, use that from now on.
	///
	/// If not, use an exponential back-off algorithm (doubling of timeout; start with an initial timeout randomized between 1 to 5 minutes).
	///
	/// A client should not poll a server more frequently than every 15 seconds, in any event.
	KissOfDeath(KissOfDeathCode),
	
	PrimaryServersMustHaveARootDelayOfZero
	{
		root_delay_received: UnsignedShortFormat,
	},
	
	PrimaryServersMustHaveARootDispersionOfZero
	{
		root_dispersion_received: SignedShortFormat,
	},
	
	SecondaryServersMustHaveARootDelayOfLessThanOne
	{
		root_delay_received: Unsigned1616FixedPoint,
	},
	
	SecondaryServersMustHaveARootDispersionOfBetweenZeroAndLessThanOne
	{
		root_dispersion_received: Signed1616FixedPoint,
	},
	
	PrimaryServersMustHavePollEqualToThatSent
	{
		poll_sent: UnsignedExponentOfTwo,
	
		poll_received: UnsignedExponentOfTwo,
	},
	
	TransmitTimestampWasZero,
	
	ReferenceTimestampHasGoneBackwards,
	
	OriginateTimestampDoesNotMatchSentTransmitTimestamp
	{
		orignate_timestamp: UnsignedTimestampFormat,
		
		sent_transmit_timestamp: UnsignedTimestampFormat,
	},
	
	ARoundTripDelayOfLessThanZeroIsOnlyPossibleForSymmetricModesWhichAreNotPermittedBySntp { roundtrip_delay: Signed3232FixedPoint },
	
	InvalidSecondaryServerReferenceIdentifier
	{
		reference_identifier_unspecified: [u8; 4],
	},
	
	TooLargeATimestampDifference
	{
		left: UnsignedTimestampFormat,
		
		right: UnsignedTimestampFormat,
	},
	
	TooLargeASigned3232FixedPointDifference,
}

impl Display for NetworkTimeProtocolMessageServerReplyParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formmater) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for NetworkTimeProtocolMessageServerReplyParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::NetworkTimeProtocolMessageServerReplyParseError::*;
		
		match self
		{
			&IoErrorOnReceive(ref error) => Some(error),
			
			&InvalidLength(..) => None,
			
			&NetworkTimeProtocolMessageServerReplyFirstByteParse(ref error) => Some(error),
			
			&NetworkTimeProtocolMessageServerReplyPacketStratumParse(ref error) => Some(error),
			
			&KissOfDeath(..) => None,
			
			&PrimaryServersMustHaveARootDelayOfZero { .. } => None,
			
			&PrimaryServersMustHaveARootDispersionOfZero { .. } => None,
			
			&PrimaryServersMustHavePollEqualToThatSent { .. } => None,
			
			&SecondaryServersMustHaveARootDelayOfLessThanOne { .. } => None,
			
			&SecondaryServersMustHaveARootDispersionOfBetweenZeroAndLessThanOne { .. } => None,
			
			&TransmitTimestampWasZero => None,
			
			&ReferenceTimestampHasGoneBackwards => None,
			
			&OriginateTimestampDoesNotMatchSentTransmitTimestamp { .. } => None,
			
			&ARoundTripDelayOfLessThanZeroIsOnlyPossibleForSymmetricModesWhichAreNotPermittedBySntp { .. } => None,
			
			&InvalidSecondaryServerReferenceIdentifier { .. } => None,
			
			&TooLargeATimestampDifference { .. } => None,
			
			&TooLargeASigned3232FixedPointDifference => None,
		}
	}
}

impl From<NetworkTimeProtocolMessageServerReplyFirstByteParseError> for NetworkTimeProtocolMessageServerReplyParseError
{
	#[inline(always)]
	fn from(value: NetworkTimeProtocolMessageServerReplyFirstByteParseError) -> Self
	{
		NetworkTimeProtocolMessageServerReplyParseError::NetworkTimeProtocolMessageServerReplyFirstByteParse(value)
	}
}

impl From<NetworkTimeProtocolMessageServerReplyPacketStratumParseError> for NetworkTimeProtocolMessageServerReplyParseError
{
	#[inline(always)]
	fn from(value: NetworkTimeProtocolMessageServerReplyPacketStratumParseError) -> Self
	{
		NetworkTimeProtocolMessageServerReplyParseError::NetworkTimeProtocolMessageServerReplyPacketStratumParse(value)
	}
}
