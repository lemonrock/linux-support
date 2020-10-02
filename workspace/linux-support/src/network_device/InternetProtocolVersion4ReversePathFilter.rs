// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `rp_filter`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum InternetProtocolVersion4ReversePathFilter
{
	#[allow(missing_docs)]
	NoSourceValidation = 0,
	
	/// Strict mode as defined in RFC3704 Strict Reverse Path Each incoming packet is tested against the FIB and if the interface is not the best reverse path the packet check will fail. By default failed packets are discarded.
	StrictMode = 1,
	
	/// Loose mode as defined in RFC3704 Loose Reverse Path Each incoming packet’s source address is also tested against the FIB and if the source address is not reachable via any interface the packet check will fail.
	LooseMode = 2,
}

impl Default for InternetProtocolVersion4ReversePathFilter
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion4ReversePathFilter::StrictMode
	}
}

impl InternetProtocolVersion4ReversePathFilter
{
	#[inline(always)]
	pub(crate) fn parse(value: u32) -> Result<Self, String>
	{
		use self::InternetProtocolVersion4ReversePathFilter::*;
		
		match value
		{
			0 => Ok(NoSourceValidation),
			
			1 => Ok(StrictMode),
			
			2 => Ok(LooseMode),
			
			_ => Err(format!("Unexpected value for rp_filter of {}", value))
		}
	}
}
