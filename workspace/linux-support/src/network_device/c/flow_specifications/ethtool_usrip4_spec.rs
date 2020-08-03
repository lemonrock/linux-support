// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Flow specification for IPv4.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub(crate) struct ethtool_usrip4_spec
{
	/// Source host.
	pub(crate) ip4src: BigEndianU32,
	
	/// Destination host.
	pub(crate) ip4dst: BigEndianU32,
	
	/// First 4 bytes of transport (layer 4) header.
	pub(crate) l4_4_bytes: BigEndianU32,
	
	/// Type-of-Service (TOS).
	pub(crate) tos: u8,
	
	/// Value must be `ETH_RX_NFC_IP4`; when masked, must be `0`.
	pub(crate) ip_ver: u8,
	
	/// Transport protocol number; when masked, must be `0`.
	pub(crate) proto: u8,
}

impl FlowSpecification for ethtool_usrip4_spec
{
}
