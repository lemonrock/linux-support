// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct InternetProtocolAddressesDiagnostic<IPA: InternetProtocolAddress>(Vec<GetAddressMessageData<IPA>>);

impl<IPA: InternetProtocolAddress> InternetProtocolAddressesDiagnostic<IPA>
{
	#[inline(always)]
	fn gather_internal(get_internet_protocol_addresses: impl FnOnce(&mut NetlinkSocketFileDescriptor<RouteNetlinkProtocol>) -> Result<Vec<GetAddressMessageData<IPA>>, String>) -> DiagnosticUnobtainableResult<Self>
	{
		let mut netlink_socket_file_descriptor = NetlinkSocketFileDescriptor::open().map_err(DiagnosticUnobtainable::from)?;
		match get_internet_protocol_addresses(&mut netlink_socket_file_descriptor)
		{
			Err(error) => Err(DiagnosticUnobtainable(error)),
			Ok(internet_protocol_addresses) => Ok(Self(internet_protocol_addresses))
		}
	}
}

impl InternetProtocolAddressesDiagnostic<in_addr>
{
	#[inline(always)]
	fn gather() -> DiagnosticUnobtainableResult<Self>
	{
		Self::gather_internal(RouteNetlinkProtocol::get_internet_protocol_version_4_addresses)
	}
}

impl InternetProtocolAddressesDiagnostic<in6_addr>
{
	#[inline(always)]
	fn gather() -> DiagnosticUnobtainableResult<Self>
	{
		Self::gather_internal(RouteNetlinkProtocol::get_internet_protocol_version_6_addresses)
	}
}
