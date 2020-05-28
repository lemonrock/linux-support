// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for `RTM_NEWLINK`, `RTM_DELLINK` and `RTM_GETLINK`.
#[repr(C)]
pub struct ifinfomsg
{
	/// Often `AF_UNSPEC`.
	pub ifi_family: u8,
	
	__ifi_pad: u8,
	
	/// Device type.
	pub ifi_type: u16,
	
	/// `0` for unspecified.
	pub ifi_index: Option<NetworkInterfaceIndex>,
	
	/// Device flags.
	///
	/// Flags start `IFF_*`.
	pub ifi_flags: u32,
	
	/// Change bit mask; currently always `0xFFFF_FFFF` (officially).
	///
	/// However, seems to change on interface state change: "If there is a state change it gives a finite value else it is zero".
	///
	/// See <https://stackoverflow.com/questions/27700208/how-to-suppress-multiple-netlink-events>.
	pub ifi_change: u32,
}

impl NetlinkRequestMessageBody for ifinfomsg
{
}
