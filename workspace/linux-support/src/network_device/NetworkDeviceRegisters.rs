// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Registers.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkDeviceRegisters
{
	/// Registers type (structure or layout) version.
	pub version: u32,
	
	/// Registers binary data.
	pub binary_data: Box<[u8]>
}
