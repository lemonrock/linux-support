// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct State<SD: SocketData>
{
	sent_transmit_timestamp: UnsignedTimestampFormat,
	
	sent_poll: UnsignedExponentOfTwo8,
	
	previous_server_reference_timestamp: Unsigned3232FixedPoint
}

impl<SD: SocketData> State<IPA>
{
	#[inline(always)]
	const fn new() -> Self
	{
		Self
		{
			sent_transmit_timestamp: UnsignedTimestampFormat::Zero,
			sent_poll: UnsignedExponentOfTwo8::Zero,
			previous_server_reference_timestamp: Unsigned3232FixedPoint::Zero,
		}
	}
	
	#[inline(always)]
	fn update_on_send(&mut self, sent_transmit_timestamp: UnsignedTimestampFormat)
	{
		self.sent_transmit_timestamp = sent_transmit_timestamp
	}
	
	#[inline(always)]
	fn simple_validate_and_parse_server_reply(&mut self, receive_buffer: &[u8; NetworkTimeProtocolMessage::PacketSize]) -> Result<(LeapIndicator, ServerStratum, Unsigned3232FixedPoint, Signed3232FixedPoint), NetworkTimeProtocolMessageServerReplyParseError>
	{
		let ((leap_indicator, server_stratum, round_trip_delay, system_clock_offset, next_server_reference_timestamp)) = NetworkTimeProtocolMessage::simple_validate_and_parse_server_reply(receive_buffer, self.sent_transmit_timestamp, self.sent_poll, self.previous_server_reference_timestamp)?;
		self.previous_server_reference_timestamp = next_server_reference_timestamp;
		Ok((leap_indicator, server_stratum, round_trip_delay, system_clock_offset))
	}
}
