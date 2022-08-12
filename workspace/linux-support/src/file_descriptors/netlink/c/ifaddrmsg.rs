// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for `RTM_NEWADDR`, `RTM_DELADDR` and `RTM_GETADDR`.
///
/// In addition, the attribute `IFA_TARGET_NETNSID` can be specified after this header in `RTM_GETADDR` to filter by `NetNamespaceIdentifier`.
#[repr(C)]
pub(crate) struct ifaddrmsg
{
	/// Address type.
	///
	/// Must be either `AF_INET` or `AF_INET6` for `RTM_GETADDR` ordinarily.
	pub(crate) ifa_family: u8,
	
	/// Prefix length of address.
	///
	/// Must be `0` for `RTM_GETADDR`.
	pub(crate) ifa_prefixlen: u8,
	
	/// Flags.
	///
	/// Can be extended by a `rtattr` with a type of `IFA_FLAGS`.
	///
	/// If a `rtattr` with a type of `IFA_FLAGS` is present this field should be ignored.
	///
	/// Must be `0` for `RTM_GETADDR`.
	pub(crate) ifa_flags: InterfaceFlags,
	
	/// Address scope.
	///
	/// Must be `0` for `RTM_GETADDR`.
	pub(crate) ifa_scope: rt_scope,
	
	/// Interface index.
	///
	/// Can be specified in `RTM_GETADDR` to filter by network interface index.
	pub(crate) ifa_index: Option<NetworkInterfaceIndex>,
}

impl NetlinkRequestMessageBody for ifaddrmsg
{
	#[inline(always)]
	fn family(&self) -> c_uchar
	{
		self.ifa_family
	}
}
