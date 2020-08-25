// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Network device diagnostic.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkDeviceDiagnostic
{
	/// From netlink.
	#[serde(flatten)] pub link: GetLinkMessageData,

	/// From `ioctl()`.
	#[serde(flatten)] pub input_output_control: DiagnosticUnobtainableResult<NetworkDeviceInputOutputControlDiagnostic>,
}

impl NetworkDeviceDiagnostic
{
	#[inline(always)]
	fn gather(link: GetLinkMessageData) -> Self
	{
		Self
		{
			input_output_control: NetworkDeviceInputOutputControlDiagnostic::gather(&link.network_interface_name),
			link,
		}
	}
}
