// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Is this map system wide or per-HyperThread?
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum PerDevice
{
	/// Is per device.
	IsPerDevice(NetworkInterfaceIndex),
	
	/// Is per-cpu or system-wide.
	Cpu(PerCpu),
}

impl Default for PerDevice
{
	#[inline(always)]
	fn default() -> Self
	{
		PerDevice::Cpu(PerCpu::default())
	}
}
