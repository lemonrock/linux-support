// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 2863 operational status.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum IF_OPER
{
	/// `unknown` in sysfs (`/sys/class/net/<network_interface_name>/oper_state`).
	#[serde(rename = "Unknown")] IF_OPER_UNKNOWN = 0,
	
	/// Apparently unused according to a Linux kernel source comment.
	///
	/// `notpresent` in sysfs (`/sys/class/net/<network_interface_name>/oper_state`).
	#[serde(rename = "Not Present")] IF_OPER_NOTPRESENT = 1,
	
	/// `down` in sysfs (`/sys/class/net/<network_interface_name>/oper_state`).
	#[serde(rename = "Down")] IF_OPER_DOWN = 2,
	
	/// `lowerlayerdown` in sysfs (`/sys/class/net/<network_interface_name>/oper_state`).
	#[serde(rename = "Lower Layer Down")] IF_OPER_LOWERLAYERDOWN = 3,
	
	/// Apparently unused according to a Linux kernel source comment.
	///
	/// `testing` in sysfs (`/sys/class/net/<network_interface_name>/oper_state`).
	#[serde(rename = "Testing")] IF_OPER_TESTING = 4,
	
	/// `dormant` in sysfs (`/sys/class/net/<network_interface_name>/oper_state`).
	///
	/// Check that `/sys/class/net/<network_interface_name>/dormant` is true before relying on this state.
	///
	/// Also reported as a boolean in `/sys/class/net/<network_interface_name>/dormant` which:-
	///
	/// * if true, reflects that a driver has been placed in the `dormant` state but the operational status won't yet have been updated to `IF_OPER_DORMANT`.
	/// * if false, reflects that a driver has never been placed in the `dormant` state, or, rarely, is exiting from a `dormant` state but the operational status won't yet have been updated to `IF_OPER_UP`.
	///
	/// Rarely-used state by a very small number of Linux drivers, none of which are common.
	///
	/// See also `NetworkInterfaceName.dormant()`.
	#[serde(rename = "Dormant")] IF_OPER_DORMANT = 5,
	
	/// `up` in sysfs (`/sys/class/net/<network_interface_name>/oper_state`).
	///
	/// Check that `/sys/class/net/<network_interface_name>/dormant` is not true before relying on this state.
	#[serde(rename = "Up")] IF_OPER_UP = 6,
}
