// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct InternetProtocolVersion4AddressesDiagnostic
{
	pub addresses: DiagnosticUnobtainableResult<Vec<GetInternetProtocolVersion4AddressMessageData>>,
}

impl InternetProtocolVersion4AddressesDiagnostic
{
	#[inline(always)]
	fn gather() -> DiagnosticUnobtainableResult<Self>
	{
		let mut netlink_socket_file_descriptor = NetlinkSocketFileDescriptor::open().map_err(DiagnosticUnobtainable::from)?;
		
		Ok
		(
			Self
			{
				addresses: RouteNetlinkProtocol::get_internet_protocol_version_4_addresses(&mut netlink_socket_file_descriptor, None).map_err(DiagnosticUnobtainable::from),
			}
		)
	}
}
