// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// * Totally unsupported by Intel ixgbevf.
#[allow(missing_docs)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Layer4HashFunctionFields
{
	#[serde(flatten)] internet_protocol: InternetProtocolHashFunctionFields,
	
	/// * Supported by Amazon ENA.
	/// * Unsupported by Intel ixgbevf.
	pub include_source_port: bool,
	
	/// * Supported by Amazon ENA.
	/// * Unsupported by Intel ixgbevf.
	pub include_destination_port: bool,
}

impl From<RXH> for Layer4HashFunctionFields
{
	#[inline(always)]
	fn from(rxh: RXH) -> Self
	{
		Self
		{
			internet_protocol: InternetProtocolHashFunctionFields::from(rxh),
			
			include_source_port: rxh.contains(RXH::FirstTwoBytesOfLayer4Header),
			
			include_destination_port: rxh.contains(RXH::NextTwoBytesOfLayer4Header),
		}
	}
}

impl ToDataField for Layer4HashFunctionFields
{
	#[inline(always)]
	fn to_data_field(&self) -> RXH
	{
		let mut data_field = self.internet_protocol.to_data_field();
		
		if self.include_source_port
		{
			data_field |= RXH::FirstTwoBytesOfLayer4Header
		}
		
		if self.include_destination_port
		{
			data_field |= RXH::NextTwoBytesOfLayer4Header
		}
		
		data_field
	}
}
