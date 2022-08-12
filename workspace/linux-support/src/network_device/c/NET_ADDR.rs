// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Hardware address assignment types.
///
/// Also available at `/sys/class/net/<network_interface_name>/addr_assign_type`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[derive(EnumCount)]
#[repr(u8)]
pub enum NET_ADDR
{
	/// Address is permanent (default).
	#[serde(rename = "permanent")] NET_ADDR_PERM = 0,

	/// Address is generated randomly.
	#[serde(rename = "random")] NET_ADDR_RANDOM = 1,

	/// Address is stolen from another device.
	#[serde(rename = "stolen")] NET_ADDR_STOLEN = 2,
	
	/// Address has been set using `dev_set_mac_address()`.
	#[serde(rename = "set")] NET_ADDR_SET = 3,
}
