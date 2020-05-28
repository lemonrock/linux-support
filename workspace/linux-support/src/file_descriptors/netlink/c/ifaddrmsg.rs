// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for `RTM_NEWADDR`, `RTM_DELADDR` and `RTM_GETADDR`.
#[repr(C)]
pub(crate) struct ifaddrmsg
{
	/// Address type.
	pub(crate) ifa_family: u8,
	
	/// Prefix length of address.
	pub(crate) ifa_prefixlen: u8,
	
	/// Flags.
	///
	/// Can be extended by a `rtattr` with a type of `IFA_FLAGS`.
	///
	/// If a `rtattr` with a type of `IFA_FLAGS` is present this field should be ignored.
	pub(crate) ifa_flags: InterfaceFlags,
	
	/// Address scope.
	pub(crate) ifa_scope: rt_scope,
	
	/// Interface index.
	pub(crate) ifa_index: Option<NetworkInterfaceIndex>,
}

impl NetlinkRequestMessageBody for ifaddrmsg
{
}
