// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// ?Might not only be for VLANs?
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct VirtualLocalAreaNetworkExtendedFlow
{
	pub virtual_local_area_network_ethertype: MaskedData<EtherTypeOrLength>,

	pub virtual_local_area_network_tag_control_information: MaskedData<VirtualLocalAreaNetworkTagControlInformation>,

	pub user_defined_data: MaskedData<UserDefinedData>,
}

impl VirtualLocalAreaNetworkExtendedFlow
{
	fn parse_extended_flow_type(ethtool_flow_specification: &ethtool_rx_flow_spec) -> Option<Self>
	{
		if ethtool_flow_specification.has_extended_flow_type()
		{
			Some
			(
				Self
				{
					virtual_local_area_network_ethertype: MaskedData::from_underlying_data_and_mask(ethtool_flow_specification.h_ext.vlan_etype, ethtool_flow_specification.m_ext.vlan_etype),
				
					virtual_local_area_network_tag_control_information: MaskedData::from_underlying_data_and_mask(ethtool_flow_specification.h_ext.vlan_tci, ethtool_flow_specification.m_ext.vlan_tci),
					
					user_defined_data: MaskedData::from_underlying_data_and_mask(ethtool_flow_specification.h_ext.data, ethtool_flow_specification.m_ext.data),
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
		h_ext.vlan_etype = self.virtual_local_area_network_ethertype.underlying_data();
		m_ext.vlan_etype = self.virtual_local_area_network_ethertype.underlying_mask();
		
		h_ext.vlan_tci = self.virtual_local_area_network_tag_control_information.underlying_data();
		m_ext.vlan_tci = self.virtual_local_area_network_tag_control_information.underlying_mask();
		
		h_ext.data = self.user_defined_data.underlying_data();
		m_ext.data = self.user_defined_data.underlying_mask();
	}
}
