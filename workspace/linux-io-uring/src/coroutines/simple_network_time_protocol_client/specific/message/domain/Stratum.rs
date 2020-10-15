// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Values 17 to 255 inclusive are reserved.
///
/// 8 bits.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Stratum
{
	/// Also known as "kiss-o'-death".
	UnspecifiedOrInvalid = 0,
	
	PrimaryServer = 1,
	
	SecondaryServer0 = 2,
	
	SecondaryServer1 = 3,
	
	SecondaryServer2 = 4,
	
	SecondaryServer3 = 5,
	
	SecondaryServer4 = 6,
	
	SecondaryServer5 = 7,
	
	SecondaryServer6 = 8,
	
	SecondaryServer7 = 9,
	
	SecondaryServer8 = 10,
	
	SecondaryServer9 = 11,
	
	SecondaryServer10 = 12,
	
	SecondaryServer11 = 13,
	
	SecondaryServer12 = 14,
	
	SecondaryServer13 = 15,
	
	Unsynchronized = 16,
}

impl Stratum
{
	#[inline(always)]
	pub(crate) fn parse_server_reply(second_byte: u8) -> Result<Self, NetworkTimeProtocolMessageServerReplyPacketStratumParseError>
	{
		use self::NetworkTimeProtocolMessageServerReplyPacketStratumParseError::*;
		
		match second_byte
		{
			0 ..= 15 => Ok(unsafe { transmute(second_byte) }),
			
			16 => Err(WasUnsynchronized),
			
			_ => Err(UnknownPacketStratum(second_byte)),
		}
	}
}
