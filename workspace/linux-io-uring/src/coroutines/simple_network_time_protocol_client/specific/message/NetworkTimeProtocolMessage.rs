// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
pub(crate) struct NetworkTimeProtocolMessage
{
	first_byte: AssociationModeAndVersionNumberAndLeapIndicator,
	
	packet_stratum: Stratum,
	
	/// An exponent of 2.
	///
	/// Poll in seconds is 2^N where `N` is this value.
	///
	/// Zero for client messages.
	/// Values in server messages range from 4 to 17 inclusive which translate to 16 seconds to 131,072 seconds (circa 36 hours).
	poll: UnsignedExponentOfTwo8,
	
	/// An exponent of 2.
	///
	/// Precision in seconds is 2^N where `N` is this value.
	///
	/// Zero for client messages.
	/// Values in server messages range from -6 (mains frequency clocks) to -20 inclusive (microsecond clocks).
	precision: SignedExponentOfTwo8,
	
	/// This is a 32-bit signed fixed-point number indicating the total roundtrip delay to the primary reference source, in seconds with the fraction point between bits 15 and 16.
	///
	/// Layout is known as 'Short Format'.
	///
	/// Zero for client messages.
	/// Values in server messages range from negative values of a few milliseconds to positive values of several hundred milliseconds.
	/// Note that this variable can take on both positive and negative values, depending on the relative time and frequency offsets.
	root_delay: UnsignedShortFormat,
	
	/// This is a 32-bit unsigned fixed-point number indicating the maximum error due to the clock frequency tolerance, in seconds with the fraction point between bits 15 and 16.
	///
	/// Layout is known as 'Short Format'.
	///
	/// Zero for client messages.
	/// Values in server messages range from zero to several hundred milliseconds.
	root_dispersion: SignedShortFormat,
	
	/// This is a 32-bit bitstring identifying the particular reference source.
	///
	/// Zero for client messages.
	/// This field is significant only in server messages, where for stratum 0 (kiss-o'-death message) and 1 (primary server), the value is a four-character ASCII string, left justified and zero padded to 32 bits.
	/// For IPv4 secondary servers, the value is the 32-bit IPv4 address of the synchronization source.
	/// For IPv6 and OSI secondary servers, the value is the first 32 bits of the MD5 hash of the IPv6 or NSAP address of the synchronization source.
	reference_identifier: KissOfDeathCodeOrReferenceIdentifier,
	
	/// This field is the time the system clock was last set or corrected, in 64-bit timestamp format.
	///
	/// Layout is known as 'Timestamp Format'.
	///
	/// For a server reply, this means it is the time the last update was received from the reference source.
	/// If the server has not yet been synchronized itself, then it is zero.
	reference_timestamp: UnsignedTimestampFormat,
	
	/// This is the time at which the request departed the client for the server, in 64-bit timestamp format.
	///
	/// Layout is known as 'Timestamp Format'.
	///
	/// ie when a client receives a server message, this is the sent client message's `transmit_timestamp`.
	originate_timestamp: UnsignedTimestampFormat,
	
	/// In a server's reply, this is the time the client's request was received.
	///
	/// Layout is known as 'Timestamp Format'.
	receive_timestamp: UnsignedTimestampFormat,
	
	/// This is the time at which the request departed the client or the reply departed the server, in 64-bit timestamp format.
	///
	/// Layout is known as 'Timestamp Format'.
	///
	/// Although setting the Transmit Timestamp field in the request to the time of day according to the client clock in NTP timestamp format is not necessary in a conforming client implementation, it is highly recommended.
	/// This allows a simple calculation to determine the propagation delay between the server and client and to align the system clock generally within a few tens of milliseconds relative to the server.
	/// In addition, this provides a simple method for verifying that the server reply is in fact a legitimate response to the specific client request and thereby for avoiding replays.
	transmit_timestamp: UnsignedTimestampFormat,
}

impl Message for NetworkTimeProtocolMessage
{
	const PacketSize: usize = size_of::<Self>();
	
	#[inline(always)]
	fn serialize_request_from_client_to_server(mut self) -> ([u8; Self::PacketSize], UnsignedTimestampFormat)
	{
		let sent_transmit_timestamp = UnsignedTimestampFormat::now();
		self.transmit_timestamp = sent_transmit_timestamp;
		
		(unsafe { transmute(self) }, sent_transmit_timestamp)
	}
}

impl NetworkTimeProtocolMessage
{
	pub(crate) const SimpleNetworkTimeProtocolClientRequest: Self = Self
	{
		first_byte: AssociationModeAndVersionNumberAndLeapIndicator::SimpleNetworkTimeProtocolClientRequest,
		
		packet_stratum: Stratum::UnspecifiedOrInvalid,
		
		poll: ExponentOfTwo::Zero,
		
		precision: ExponentOfTwo::Zero,
		
		root_delay: ShortFormat::Zero,
		
		root_dispersion: ShortFormat::Zero,
		
		reference_identifier: KissOfDeathCodeOrReferenceIdentifier
		{
			unspecified: [0; 4],
		},
		
		reference_timestamp: UnsignedTimestampFormat::Zero,
		
		originate_timestamp: UnsignedTimestampFormat::Zero,
		
		receive_timestamp: UnsignedTimestampFormat::Zero,
		
		transmit_timestamp: UnsignedTimestampFormat::Zero,
	};
	
	#[inline(always)]
	fn receive_packet_buffer() -> [u8; Self::PacketSize]
	{
		let packet_buffer: [u8; Self::PacketSize] = unsafe_zeroed();
		packet_buffer
	}
	
	/// As defined by RFC 4330.
	pub(crate) fn simple_validate_and_parse_server_reply(packet: &[u8; Self::PacketSize], sent_transmit_timestamp: UnsignedTimestampFormat, sent_poll: UnsignedExponentOfTwo8, previous_server_reference_timestamp: Unsigned3232FixedPoint) -> Result<(LeapIndicator, ServerStratum, Unsigned3232FixedPoint, Signed3232FixedPoint, Unsigned3232FixedPoint), NetworkTimeProtocolMessageServerReplyParseError>
	{
		use self::NetworkTimeProtocolMessageServerReplyParseError::*;
		
		let leap_indicator = AssociationModeAndVersionNumberAndLeapIndicator::parse_into_constituent_bitfields(packet.get_unchecked_value_safe(0))?;
		
		let server_reply = unsafe { & * (packet.as_ptr() as *const Self) };
		
		let server_stratum = ServerStratum::parse_server_reply(Stratum::parse_server_reply(packet.get_unchecked_value_safe(1))?, server_reply, sent_poll)?;
		
		let (round_trip_delay, system_clock_offset, next_server_reference_timestamp) = server_reply.simple_parse_server_reply_after_basic_validation(sent_transmit_timestamp)?;
		
		Ok((leap_indicator, server_stratum, round_trip_delay, system_clock_offset, next_server_reference_timestamp))
	}
	
	#[inline(always)]
	fn simple_parse_server_reply_after_basic_validation(&self, sent_transmit_timestamp: UnsignedTimestampFormat, previous_server_reference_timestamp: Unsigned3232FixedPoint) -> Result<(Unsigned3232FixedPoint, Signed3232FixedPoint, Unsigned3232FixedPoint), NetworkTimeProtocolMessageServerReplyParseError>
	{
		use self::NetworkTimeProtocolMessageServerReplyParseError::*;
		
		let originate_timestamp =
		{
			let originate_timestamp = self.originate_timestamp;
			if unlikely!(originate_timestamp != sent_transmit_timestamp)
			{
				return Err(OriginateTimestampDoesNotMatchSentTransmitTimestamp { orignate_timestamp, sent_transmit_timestamp })
			}
			originate_timestamp
		};
		
		let transmit_timestamp =
		{
			let transmit_timestamp = self.transmit_timestamp;
			if unlikely!(transmit_timestamp.is_zero())
			{
				return Err(TransmitTimestampWasZero)
			}
			transmit_timestamp
		};
		
		let reference_timestamp = self.reference_timestamp.into();
		if unlikely!(reference_timestamp < previous_server_reference_timestamp)
		{
			return Err(ReferenceTimestampHasGoneBackwards)
		}
		
		let destination_timestamp = UnsignedTimestampFormat::now();
		
		let T1 = originate_timestamp;
		let T2 = self.receive_timestamp;
		let T3 = transmit_timestamp;
		let T4 = destination_timestamp;
		
		let roundtrip_delay_d: Unsigned3232FixedPoint =
		{
			// `d = (T4 - T1) - (T3 - T2)`.
			// Also known as delta.
			let left = T4.subtract(T1)?;
			let right = T3.subtract(T2)?;
			let round_trip_delay = left.checked_sub(right).ok_or(TooLargeASigned3232FixedPointDifference)?;
			if unlikely!(roundtrip_delay_d.is_negative())
			{
				return Err(ARoundTripDelayOfLessThanZeroIsOnlyPossibleForSymmetricModesWhichAreNotPermittedBySntp { roundtrip_delay })
			}
			roundtrip_delay_d.into()
		};
		
		let system_clock_offset =
		{
			// `t = ((T2 - T1) + (T3 - T4)) / 2`.
			// Also known as theta.
			let left = T2.subtract(T1)?;
			let right = T3.checked_sub(T4)?;
			let top = left.checked_add(right).ok_or(TooLargeASigned3232FixedPointDifference)?;
			top.checked_div_by_scalar(2).ok_or((TooLargeASigned3232FixedPointDifference))?
		};
		
		Ok((round_trip_delay, system_clock_offset, reference_timestamp))
	}
}
