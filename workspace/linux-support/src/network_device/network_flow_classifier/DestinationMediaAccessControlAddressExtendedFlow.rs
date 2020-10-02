// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// ?Probably used for CommonLayer3Flow to restrict them?
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct DestinationMediaAccessControlAddressExtendedFlow
{
	#[allow(missing_docs)]
	#[serde(flatten)] pub destination_address: MaskedData<MediaAccessControlAddress>,
}

impl DestinationMediaAccessControlAddressExtendedFlow
{
	fn parse_extended_media_access_control_flow_type(ethtool_flow_specification: &ethtool_rx_flow_spec) -> Option<Self>
	{
		if ethtool_flow_specification.has_extended_media_access_control_flow_type()
		{
			Some
			(
				Self
				{
					destination_address: MaskedData::from_underlying_data_and_mask(ethtool_flow_specification.h_ext.h_dest, ethtool_flow_specification.m_ext.h_dest),
				}
			)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn data_and_masks(&self, h_ext: &mut ethtool_flow_ext, m_ext: &mut ethtool_flow_ext)
	{
		h_ext.h_dest = self.destination_address.underlying_data();
		m_ext.h_dest = self.destination_address.underlying_mask();
	}
}
