// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Infiniband socket address structure.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct sockaddr_ib
{
	/// Socket address family.
	sib_family: sa_family_t,

	/// Must a 16-bit integer in Network Endian form, not Native Endian form.
	pub sib_pkey: u16,

	/// Must a 32-bit integer in Network Endian form, not Native Endian form.
	pub sib_flowinfo: u32,

	/// Address.
	pub sib_addr: ib_addr,

	/// Must a 64-bit integer in Network Endian form, not Native Endian form.
	pub sib_sid: u64,

	/// Must a 64-bit integer in Network Endian form, not Native Endian form.
	pub sib_sid_mask: u64,

	/// Must a 64-bit integer in *Native Endian form*.
	pub sib_scope_id: u64,
}

impl Default for sockaddr_ib
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			sib_family: AF_IB as sa_family_t,
			sib_pkey: 0,
			sib_flowinfo: 0,
			sib_addr: ib_addr::default(),
			sib_sid: 0,
			sib_sid_mask: 0,
			sib_scope_id: 0,
		}
	}
}

impl SocketData for sockaddr_ib
{
	#[inline(always)]
	fn family(&self) -> sa_family_t
	{
		self.sib_family
	}
}
