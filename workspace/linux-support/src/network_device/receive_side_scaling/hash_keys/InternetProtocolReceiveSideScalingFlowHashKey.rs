// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// * Totally unsupported by Intel ixgbevf.
#[allow(missing_docs)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct InternetProtocolReceiveSideScalingFlowHashKey
{
	/// * Not supported by Amazon ENA.
	#[serde(flatten)] pub ethernet: EthernetReceiveSideScalingFlowHashKey,
	
	/// * Not supported by Amazon ENA.
	pub include_layer_3_protocol_number: bool,
	
	/// * Supported by Amazon ENA.
	pub include_internet_protocol_version_6_source_address: bool,
	
	/// * Supported by Amazon ENA.
	pub include_internet_protocol_version_6_destination_address: bool,
}

impl From<RXH> for InternetProtocolReceiveSideScalingFlowHashKey
{
	#[inline(always)]
	fn from(rxh: RXH) -> Self
	{
		Self
		{
			ethernet: EthernetReceiveSideScalingFlowHashKey::from(rxh),
			
			include_layer_3_protocol_number: rxh.contains(RXH::Layer3ProtocolNumber),
			
			include_internet_protocol_version_6_source_address: rxh.contains(RXH::InternetProtocolVersion4OrInternetProtocolVersion6SourceAddress),
			
			include_internet_protocol_version_6_destination_address: rxh.contains(RXH::InternetProtocolVersion4OrInternetProtocolVersion6DestinationAddress),
		}
	}
}

impl ToDataField for InternetProtocolReceiveSideScalingFlowHashKey
{
	#[inline(always)]
	fn to_data_field(&self) -> RXH
	{
		let mut data_field = self.ethernet.to_data_field();
		
		if self.include_layer_3_protocol_number
		{
			data_field |= RXH::Layer3ProtocolNumber
		}
		
		if self.include_internet_protocol_version_6_source_address
		{
			data_field |= RXH::InternetProtocolVersion4OrInternetProtocolVersion6SourceAddress
		}
		
		if self.include_internet_protocol_version_6_destination_address
		{
			data_field |= RXH::InternetProtocolVersion4OrInternetProtocolVersion6DestinationAddress
		}
		
		data_field
	}
}
