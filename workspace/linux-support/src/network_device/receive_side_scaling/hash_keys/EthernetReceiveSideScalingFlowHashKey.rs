// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// * Totally unsupported by Intel ixgbevf.
#[allow(missing_docs)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct EthernetReceiveSideScalingFlowHashKey
{
	/// * Supported by Amazon ENA.
	pub include_ethernet_destination_address: bool,
	
	/// * Not supported by Amazon ENA.
	pub include_virtual_local_area_network_tag: bool,
}

impl From<RXH> for EthernetReceiveSideScalingFlowHashKey
{
	#[inline(always)]
	fn from(rxh: RXH) -> Self
	{
		Self
		{
			include_ethernet_destination_address: rxh.contains(RXH::EthernetDestinationAddress),
			
			include_virtual_local_area_network_tag: rxh.contains(RXH::EthernetVirtualLocalAreaNetworkTag),
		}
	}
}

impl ToDataField for EthernetReceiveSideScalingFlowHashKey
{
	#[inline(always)]
	fn to_data_field(&self) -> RXH
	{
		let mut data_field = RXH::empty();
		
		if self.include_ethernet_destination_address
		{
			data_field |= RXH::EthernetDestinationAddress
		}
		
		if self.include_virtual_local_area_network_tag
		{
			data_field |= RXH::EthernetVirtualLocalAreaNetworkTag
		}
		
		data_field
	}
}
