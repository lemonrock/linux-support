// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive flow rule.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rule
{
	/// Rule location.
	#[serde(default)] pub location: RuleLocation,
	
	/// Rule action.
	#[serde(default)] pub action: RuleAction,

	/// Basic receive flow specification.
	pub basic_flow: BasicFlow,
	
	/// Additional receive flow specification.
	#[serde(default)] pub virtual_local_area_network_extended_flow: Option<VirtualLocalAreaNetworkExtendedFlow>,
}

impl Rule
{
	#[allow(dead_code)]
	pub(crate) fn parse(ethtool_flow_specification: ethtool_rx_flow_spec) -> Result<(Self, bool), FlowSpecificationParseError>
	{
		use self::FlowSpecificationParseError::*;
		
		let location = ethtool_flow_specification.location.try_into().map_err(SpecialRuleLocationInvalid)?;
		
		let destination_media_access_control_address_extended_flow = DestinationMediaAccessControlAddressExtendedFlow::parse_extended_media_access_control_flow_type(&ethtool_flow_specification);
		
		let basic_flow = BasicFlow::parse_actual_flow_type(&ethtool_flow_specification, destination_media_access_control_address_extended_flow)?;
		let virtual_local_area_network_extended_flow = VirtualLocalAreaNetworkExtendedFlow::parse_extended_flow_type(&ethtool_flow_specification);
		
		let has_receive_side_scaling_context = ethtool_flow_specification.has_receive_side_scaling_flow_type();
		
		let action = ethtool_flow_specification.ring_cookie.try_into().map_err(RingCookieQueueIdentifierOutOfRange)?;
		
		Ok
		(
			(
				Self
				{
					action,
				
					location,
					
					basic_flow,
				
					virtual_local_area_network_extended_flow,
				},
				
				has_receive_side_scaling_context
			)
		)
	}
	
	#[allow(dead_code)]
	pub(crate) fn to_ethtool_flow_specification(&self, has_receive_side_scaling_context: bool) -> ethtool_rx_flow_spec
	{
		let mut ethtool_flow_specification: ethtool_rx_flow_spec = unsafe { zeroed() };
		
		let (mut flow_type, (data, mask), destination_media_access_control_address_extended_flow) = self.basic_flow.actual_flow_type_and_data_and_masks();
		
		if let Some(ref virtual_local_area_network_extended_flow) = self.virtual_local_area_network_extended_flow
		{
			flow_type |= FLOW_EXT;
			virtual_local_area_network_extended_flow.data_and_masks(&mut ethtool_flow_specification.h_ext, &mut ethtool_flow_specification.m_ext)
		}
		
		if let Some(ref destination_media_access_control_address_extended_flow) = destination_media_access_control_address_extended_flow
		{
			flow_type |= FLOW_MAC_EXT;
			destination_media_access_control_address_extended_flow.data_and_masks(&mut ethtool_flow_specification.h_ext, &mut ethtool_flow_specification.m_ext)
		}
		
		if has_receive_side_scaling_context
		{
			flow_type |= FLOW_RSS
		}
		
		ethtool_flow_specification.flow_type = flow_type;
		
		ethtool_flow_specification.h_u = data;
		
		ethtool_flow_specification.m_u = mask;
		
		ethtool_flow_specification.ring_cookie = self.action.into();
		
		ethtool_flow_specification.location = self.location.into();
		
		ethtool_flow_specification
	}
	
	// `do_srxclass` "flow-type"
	pub(crate) fn to_x(&self) -> ethtool_rxnfc
	{
	}
}
