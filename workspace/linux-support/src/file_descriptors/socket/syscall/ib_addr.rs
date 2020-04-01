// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Infiniband address.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ib_addr
{
	/// Must be a 64-bit integer in Network Endian form, not Native Endian form.
	pub sib_subnet_prefix: u64,

	/// Must be a 64-bit integer in Network Endian form, not Native Endian form.
	pub sib_interface_id: u64,
}

impl ib_addr
{
	/// The 'any' address.
	pub const Any: Self = Self
	{
		sib_subnet_prefix: 0,
		sib_interface_id: 0,
	};

	/// The 'loopback' address.
	#[cfg(target_endian = "little")]
	pub const Loopback: Self = Self
	{
		sib_subnet_prefix: 0,
		sib_interface_id: 0x100000000000000,
	};

	/// The 'loopback' address.
	#[cfg(target_endian = "big")]
	pub const Loopback: Self = Self
	{
		sib_subnet_prefix: 0,
		sib_interface_id: 0x000000100000000,
	};

	/// Is 'any' address.
	#[inline(always)]
	pub fn is_any(&self) -> bool
	{
		self == &Self::Any
	}

	/// Is 'loopback' address.
	#[inline(always)]
	pub fn is_loopback(&self) -> bool
	{
		self == &Self::Loopback
	}
}
