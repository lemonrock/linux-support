// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// * Totally unsupported by Amazon ENA.
/// * Totally unsupported by Intel ixgbevf.
#[allow(missing_docs)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct IpsecReceiveSideScalingFlowHashKey
{
	#[serde(flatten)] internet_protocol: InternetProtocolReceiveSideScalingFlowHashKey,
	
	pub include_security_parameter_index: bool,
}

impl From<RXH> for IpsecReceiveSideScalingFlowHashKey
{
	#[inline(always)]
	fn from(rxh: RXH) -> Self
	{
		Self
		{
			internet_protocol: InternetProtocolReceiveSideScalingFlowHashKey::from(rxh),
			
			include_security_parameter_index: rxh.contains(RXH::FirstTwoBytesOfLayer4Header) && rxh.contains(RXH::NextTwoBytesOfLayer4Header),
		}
	}
}

impl ToDataField for IpsecReceiveSideScalingFlowHashKey
{
	#[inline(always)]
	fn to_data_field(&self) -> RXH
	{
		let data_field = self.internet_protocol.to_data_field();
		
		if self.include_security_parameter_index
		{
			data_field | (RXH::FirstTwoBytesOfLayer4Header | RXH::NextTwoBytesOfLayer4Header)
		}
		else
		{
			data_field
		}
	}
}
