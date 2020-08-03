// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Additional receive flow fields.
/// Note:-
///
/// * `vlan_etype`, `vlan_tci`, and `data` are only valid if `FLOW_EXT` is set in struct `ethtool_rx_flow_spec flow_type`.
/// * `h_dest` is valid if `FLOW_MAC_EXT is set`.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct ethtool_flow_ext
{
	padding: [u8; 2],
	
	// Destination MAC address.
	pub(crate) h_dest: [c_uchar; ETH_ALEN],
	
	/// VLAN EtherType.
	pub(crate) vlan_etype: BigEndianU16,
	
	/// VLAN tag control information (TCI).
	pub(crate) vlan_tci: BigEndianU16,
	
	/// User defined data.
	pub(crate) data: [BigEndianU32; 2],
}
