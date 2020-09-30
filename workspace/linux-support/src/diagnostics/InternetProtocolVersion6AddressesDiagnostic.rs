// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct InternetProtocolVersion6AddressesDiagnostic
{
	pub unicast: DiagnosticUnobtainableResult<Vec<GetInternetProtocolVersion6AddressMessageData>>,
	
	pub multicast: DiagnosticUnobtainableResult<Vec<GetInternetProtocolVersion6OtherCastAddressMessageData>>,
	
	pub anycast: DiagnosticUnobtainableResult<Vec<GetInternetProtocolVersion6OtherCastAddressMessageData>>,
}

impl InternetProtocolVersion6AddressesDiagnostic
{
	#[inline(always)]
	fn gather() -> DiagnosticUnobtainableResult<Self>
	{
		let netlink_socket_file_descriptor = NetlinkSocketFileDescriptor::open().map_err(DiagnosticUnobtainable::from)?;
		
		Ok
		(
			Self
			{
				unicast: RouteNetlinkProtocol::get_internet_protocol_version_6_addresses(&mut netlink_socket_file_descriptor, None).map_err(DiagnosticUnobtainable::from),
				
				multicast: RouteNetlinkProtocol::get_internet_protocol_version_6_multicast_addresses(&mut netlink_socket_file_descriptor, None).map_err(DiagnosticUnobtainable::from),
				
				anycast: RouteNetlinkProtocol::get_internet_protocol_version_6_anycast_addresses(&mut netlink_socket_file_descriptor, None).map_err(DiagnosticUnobtainable::from),
			}
		)
	}
}
