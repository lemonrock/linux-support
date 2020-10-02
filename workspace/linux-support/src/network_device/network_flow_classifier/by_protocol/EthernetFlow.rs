// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `ethtool` calls this `ether`.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct EthernetFlow
{
	/// For Intel devices, only an all 1s mask is supported (ie exact match).
	pub source_address: MaskedData<MediaAccessControlAddress>,
	
	/// For Intel devices, only an all 1s mask is supported (ie exact match).
	pub destination_address: MaskedData<MediaAccessControlAddress>,
	
	/// For Intel devices, only an all 1s mask is supported (ie exact match).
	pub ethertype_or_length: MaskedData<EtherTypeOrLength>,
}

impl EthernetFlow
{
	#[inline(always)]
	pub(super) fn new(data: &ethhdr, mask: &ethhdr) -> Self
	{
		Self
		{
			source_address: MaskedData::from_underlying_data_and_mask(data.h_source, mask.h_source),
			
			destination_address: MaskedData::from_underlying_data_and_mask(data.h_dest, mask.h_dest),
			
			ethertype_or_length: MaskedData::from_underlying_data_and_mask(data.h_proto, mask.h_proto),
		}
	}
}

impl DataAndMasks for EthernetFlow
{
	#[inline(always)]
	fn data_and_masks(&self) -> (ethtool_flow_union, ethtool_flow_union)
	{
		(
			ethtool_flow_union
			{
				ether_spec: ethhdr
				{
					h_dest: self.destination_address.underlying_data(),
					
					h_source: self.source_address.underlying_data(),
					
					h_proto: self.ethertype_or_length.underlying_data(),
				}
			},
			
			ethtool_flow_union
			{
				ether_spec: ethhdr
				{
					h_dest: self.destination_address.underlying_mask(),
					
					h_source: self.source_address.underlying_mask(),
					
					h_proto: self.ethertype_or_length.underlying_mask(),
				}
			},
		)
	}
}
