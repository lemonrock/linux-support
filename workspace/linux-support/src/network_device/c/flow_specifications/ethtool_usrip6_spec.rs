// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Flow specification for IPv6.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub(crate) struct ethtool_usrip6_spec
{
	/// Source host.
	pub(crate) ip6src: [BigEndianU32; 4],
	
	/// Destination host.
	pub(crate) ip6dst: [BigEndianU32; 4],
	
	/// First 4 bytes of transport (layer 4) header.
	pub(crate) l4_4_bytes: BigEndianU32,
	
	/// Traffic Class.
	pub(crate) tclass: u8,
	
	/// Transport protocol number (nexthdr after any Extension Headers).
	pub(crate) l4_proto: u8,
}

impl FlowSpecification for ethtool_usrip6_spec
{
}

impl CommonLayer3FlowSpecification<[BigEndianU32; 4]> for ethtool_usrip6_spec
{
	#[inline(always)]
	fn source_address(&self) -> [BigEndianU32; 4]
	{
		self.ip6src
	}
	
	#[inline(always)]
	fn destination_address(&self) -> [BigEndianU32; 4]
	{
		self.ip6dst
	}
	
	#[inline(always)]
	fn tos_or_tclass(&self) -> u8
	{
		self.tclass
	}
}

impl UserFlowSpecification<[BigEndianU32; 4]> for ethtool_usrip6_spec
{
	#[inline(always)]
	fn first_four_bytes_of_layer4_header(&self) -> [u8; 4]
	{
		self.l4_4_bytes
	}
	
	#[inline(always)]
	fn transport_protocol_number(&self) -> u8
	{
		self.l4_proto
	}
}
