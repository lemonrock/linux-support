// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Network device diagnostics.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkDeviceDiagnostics(Vec<NetworkDeviceDiagnostic>);

impl NetworkDeviceDiagnostics
{
	fn gather() -> DiagnosticUnobtainableResult<Self>
	{
		let mut netlink_socket_file_descriptor = NetlinkSocketFileDescriptor::open().map_err(DiagnosticUnobtainable::from)?;
		let links = RouteNetlinkProtocol::get_links(&mut netlink_socket_file_descriptor).map_err(DiagnosticUnobtainable)?;
		
		let mut diagnostics = Vec::with_capacity(links.len());
		for link in links
		{
			diagnostics.push(NetworkDeviceDiagnostic::gather(link))
		}
		
		Ok(Self(diagnostics))
	}
}
