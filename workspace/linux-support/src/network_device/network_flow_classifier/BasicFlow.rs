// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Basic flow.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum BasicFlow
{
	TransmissionControlProtocolOverInternetProtocolVersion4(TransmissionControlProtocolFlow<in_addr>, Option<DestinationMediaAccessControlAddressExtendedFlow>),
	
	UserDatagramProtocolOverInternetProtocolVersion4(UserDatagramProtocolFlow<in_addr>, Option<DestinationMediaAccessControlAddressExtendedFlow>),
	
	StreamControlTransmissionProtocolOverInternetProtocolVersion4(StreamControlTransmissionProtocolFlow<in_addr>, Option<DestinationMediaAccessControlAddressExtendedFlow>),
	
	IpsecAuthenticationHeaderOverInternetProtocolVersion4(IpsecAuthenticationHeaderFlow<in_addr>, Option<DestinationMediaAccessControlAddressExtendedFlow>),
	
	IpsecEncapsulatingSecurityPayloadOverInternetProtocolVersion4(IpsecEncapsulatingSecurityPayloadFlow<in_addr>, Option<DestinationMediaAccessControlAddressExtendedFlow>),
	
	UserOverInternetProtocolVersion4(UserLayer4Flow<in_addr>, Option<DestinationMediaAccessControlAddressExtendedFlow>),
	
	TransmissionControlProtocolOverInternetProtocolVersion6(TransmissionControlProtocolFlow<in6_addr>, Option<DestinationMediaAccessControlAddressExtendedFlow>),
	
	UserDatagramProtocolOverInternetProtocolVersion6(UserDatagramProtocolFlow<in6_addr>, Option<DestinationMediaAccessControlAddressExtendedFlow>),
	
	StreamControlTransmissionProtocolOverInternetProtocolVersion6(StreamControlTransmissionProtocolFlow<in6_addr>, Option<DestinationMediaAccessControlAddressExtendedFlow>),
	
	IpsecAuthenticationHeaderOverInternetProtocolVersion6(IpsecAuthenticationHeaderFlow<in6_addr>, Option<DestinationMediaAccessControlAddressExtendedFlow>),
	
	IpsecEncapsulatingSecurityPayloadOverInternetProtocolVersion6(IpsecEncapsulatingSecurityPayloadFlow<in6_addr>, Option<DestinationMediaAccessControlAddressExtendedFlow>),
	
	UserOverInternetProtocolVersion6(UserLayer4Flow<in6_addr>, Option<DestinationMediaAccessControlAddressExtendedFlow>),
	
	Ethernet(EthernetFlow),
}

impl BasicFlow
{
	// Explore calls to ixgbe_get_rss_hash_opts or ena_get_rss_hash - reports EINVAL and EOPNOTSUPP for unsupported flows.
	fn parse_actual_flow_type(ethtool_flow_specification: &ethtool_rx_flow_spec, destination_media_access_control_address_extended_flow: Option<DestinationMediaAccessControlAddressExtendedFlow>) -> Result<BasicFlow, BasicFlowParseError>
	{
		use self::BasicFlow::*;
		use self::BasicFlowParseError::*;
		
		let actual_flow_type = ethtool_flow_specification.actual_flow_type();
		let flow = unsafe
		{
			match actual_flow_type
			{
				TCP_V4_FLOW => TransmissionControlProtocolOverInternetProtocolVersion4(TransmissionControlProtocolFlow::from(CommonLayer4Flow::new(&ethtool_flow_specification.h_u.tcp_ip4_spec, &ethtool_flow_specification.m_u.tcp_ip4_spec)), destination_media_access_control_address_extended_flow),
				
				UDP_V4_FLOW => UserDatagramProtocolOverInternetProtocolVersion4(UserDatagramProtocolFlow::from(CommonLayer4Flow::new(&ethtool_flow_specification.h_u.udp_ip4_spec, &ethtool_flow_specification.m_u.udp_ip4_spec)), destination_media_access_control_address_extended_flow),
				
				SCTP_V4_FLOW => StreamControlTransmissionProtocolOverInternetProtocolVersion4(StreamControlTransmissionProtocolFlow::from(CommonLayer4Flow::new(&ethtool_flow_specification.h_u.sctp_ip4_spec, &ethtool_flow_specification.m_u.sctp_ip4_spec)), destination_media_access_control_address_extended_flow),
				
				AH_ESP_V4_FLOW => return Err(UnexpectedActualFlowType { actual_flow_type: AH_ESP_V4_FLOW }),
				
				AH_V4_FLOW => IpsecAuthenticationHeaderOverInternetProtocolVersion4(IpsecAuthenticationHeaderFlow::from(IpsecFlow::new(&ethtool_flow_specification.h_u.ah_ip4_spec, &ethtool_flow_specification.m_u.ah_ip4_spec)), destination_media_access_control_address_extended_flow),
				
				ESP_V4_FLOW => IpsecEncapsulatingSecurityPayloadOverInternetProtocolVersion4(IpsecEncapsulatingSecurityPayloadFlow::from(IpsecFlow::new(&ethtool_flow_specification.h_u.esp_ip4_spec, &ethtool_flow_specification.m_u.esp_ip4_spec)), destination_media_access_control_address_extended_flow),
				
				IPV4_USER_FLOW =>
				{
					let ip_ver = ethtool_flow_specification.h_u.usr_ip4_spec.ip_ver;
					if unlikely!(ip_ver != ETH_RX_NFC_IP4)
					{
						return Err(UserOverInternetProtocolVersion4FlowHasInvalidVersionData { ip_ver })
					}
					
					let ip_ver = ethtool_flow_specification.m_u.usr_ip4_spec.ip_ver;
					if unlikely!(ip_ver != 0)
					{
						return Err(UserOverInternetProtocolVersion4FlowHasInvalidVersionMask { ip_ver })
					}
					
					UserOverInternetProtocolVersion4(UserLayer4Flow::new(&ethtool_flow_specification.h_u.usr_ip4_spec, &ethtool_flow_specification.m_u.usr_ip4_spec)?, destination_media_access_control_address_extended_flow)
				},
				
				TCP_V6_FLOW => TransmissionControlProtocolOverInternetProtocolVersion6(TransmissionControlProtocolFlow::from(CommonLayer4Flow::new(&ethtool_flow_specification.h_u.tcp_ip6_spec, &ethtool_flow_specification.m_u.tcp_ip6_spec)), destination_media_access_control_address_extended_flow),
				
				UDP_V6_FLOW => UserDatagramProtocolOverInternetProtocolVersion6(UserDatagramProtocolFlow::from(CommonLayer4Flow::new(&ethtool_flow_specification.h_u.udp_ip6_spec, &ethtool_flow_specification.m_u.udp_ip6_spec)), destination_media_access_control_address_extended_flow),
				
				SCTP_V6_FLOW => StreamControlTransmissionProtocolOverInternetProtocolVersion6(StreamControlTransmissionProtocolFlow::from(CommonLayer4Flow::new(&ethtool_flow_specification.h_u.sctp_ip6_spec, &ethtool_flow_specification.m_u.sctp_ip6_spec)), destination_media_access_control_address_extended_flow),
				
				AH_ESP_V6_FLOW => return Err(UnexpectedActualFlowType { actual_flow_type: AH_ESP_V6_FLOW }),
				
				AH_V6_FLOW => IpsecAuthenticationHeaderOverInternetProtocolVersion6(IpsecAuthenticationHeaderFlow::from(IpsecFlow::new(&ethtool_flow_specification.h_u.ah_ip6_spec, &ethtool_flow_specification.m_u.ah_ip6_spec)), destination_media_access_control_address_extended_flow),
				
				ESP_V6_FLOW => IpsecEncapsulatingSecurityPayloadOverInternetProtocolVersion6(IpsecEncapsulatingSecurityPayloadFlow::from(IpsecFlow::new(&ethtool_flow_specification.h_u.esp_ip6_spec, &ethtool_flow_specification.m_u.esp_ip6_spec)), destination_media_access_control_address_extended_flow),
				
				IPV6_USER_FLOW => UserOverInternetProtocolVersion6(UserLayer4Flow::new(&ethtool_flow_specification.h_u.usr_ip6_spec, &ethtool_flow_specification.m_u.usr_ip6_spec)?, destination_media_access_control_address_extended_flow),
				
				IPV4_FLOW => return Err(UnusedActualFlowType { actual_flow_type: IPV4_FLOW }),
				
				IPV6_FLOW => return Err(UnusedActualFlowType { actual_flow_type: IPV6_FLOW }),
				
				ETHER_FLOW =>
				{
					if let Some(destination_media_access_control_address_extended_flow) = destination_media_access_control_address_extended_flow
					{
						return Err(EthernetFlowIsNotAllowedAnExtendedDestinationMediaAccessControlAddress { destination_media_access_control_address_extended_flow })
					}
					
					Ethernet(EthernetFlow::new(&ethtool_flow_specification.h_u.ether_spec, &ethtool_flow_specification.m_u.ether_spec))
				}
				
				actual_flow_type @ _ => return Err(UnknownActualFlowType { actual_flow_type }),
			}
		};
		
		Ok(flow)
	}
	
	fn actual_flow_type_and_data_and_masks(&self) -> (u32, (ethtool_flow_union, ethtool_flow_union), Option<&DestinationMediaAccessControlAddressExtendedFlow>)
	{
		use self::BasicFlow::*;
		
		match self
		{
			TransmissionControlProtocolOverInternetProtocolVersion4(flow, destination_media_access_control_address_extended_flow) => (TCP_V4_FLOW, flow.data_and_masks(), destination_media_access_control_address_extended_flow.as_ref()),
			
			UserDatagramProtocolOverInternetProtocolVersion4(flow, destination_media_access_control_address_extended_flow) => (UDP_V4_FLOW, flow.data_and_masks(), destination_media_access_control_address_extended_flow.as_ref()),
			
			StreamControlTransmissionProtocolOverInternetProtocolVersion4(flow, destination_media_access_control_address_extended_flow) => (SCTP_V4_FLOW, flow.data_and_masks(), destination_media_access_control_address_extended_flow.as_ref()),
			
			IpsecAuthenticationHeaderOverInternetProtocolVersion4(flow, destination_media_access_control_address_extended_flow) => (AH_V4_FLOW, flow.data_and_masks(), destination_media_access_control_address_extended_flow.as_ref()),
			
			IpsecEncapsulatingSecurityPayloadOverInternetProtocolVersion4(flow, destination_media_access_control_address_extended_flow) => (ESP_V4_FLOW, flow.data_and_masks(), destination_media_access_control_address_extended_flow.as_ref()),
			
			UserOverInternetProtocolVersion4(flow, destination_media_access_control_address_extended_flow) => (IPV4_USER_FLOW, flow.data_and_masks(), destination_media_access_control_address_extended_flow.as_ref()),
			
			TransmissionControlProtocolOverInternetProtocolVersion6(flow, destination_media_access_control_address_extended_flow) => (TCP_V6_FLOW, flow.data_and_masks(), destination_media_access_control_address_extended_flow.as_ref()),
			
			UserDatagramProtocolOverInternetProtocolVersion6(flow, destination_media_access_control_address_extended_flow) => (UDP_V6_FLOW, flow.data_and_masks(), destination_media_access_control_address_extended_flow.as_ref()),
			
			StreamControlTransmissionProtocolOverInternetProtocolVersion6(flow, destination_media_access_control_address_extended_flow) => (SCTP_V6_FLOW, flow.data_and_masks(), destination_media_access_control_address_extended_flow.as_ref()),
			
			IpsecAuthenticationHeaderOverInternetProtocolVersion6(flow, destination_media_access_control_address_extended_flow) => (AH_V6_FLOW, flow.data_and_masks(), destination_media_access_control_address_extended_flow.as_ref()),
			
			IpsecEncapsulatingSecurityPayloadOverInternetProtocolVersion6(flow, destination_media_access_control_address_extended_flow) => (ESP_V6_FLOW, flow.data_and_masks(), destination_media_access_control_address_extended_flow.as_ref()),
			
			UserOverInternetProtocolVersion6(flow, destination_media_access_control_address_extended_flow) => (IPV6_USER_FLOW, flow.data_and_masks(), destination_media_access_control_address_extended_flow.as_ref()),
			
			Ethernet(flow) => (ETHER_FLOW, flow.data_and_masks(), None),
		}
	}
}
