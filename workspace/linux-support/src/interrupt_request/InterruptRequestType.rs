// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum InterruptRequestType
{
	/// Edge.
	Edge,
	
	/// Level.
	Level,
}

impl FromBytes for InterruptRequestType
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		use self::InterruptRequestType::*;
		
		match bytes
		{
			b"edge" => Ok(Edge),
			
			b"level" => Ok(Level),
			
			_ => Err(ParseNumberError::OutOfRange)
		}
	}
}
