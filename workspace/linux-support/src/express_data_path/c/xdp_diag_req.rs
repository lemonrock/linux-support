// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub(crate) struct xdp_diag_req
{
	sdiag_family: u8,
	
	sdiag_protocol: u8,
	
	pad: u16,
	
	xdiag_ino: u32,
	
	xdiag_show: XDP_SHOW_flags,
	
	xdiag_cookie: [u32; 2],
}

impl NetlinkRequestMessageBody for xdp_diag_req
{
	#[inline(always)]
	fn family(&self) -> c_uchar
	{
		self.sdiag_family
	}
}

impl xdp_diag_req
{
	#[inline(always)]
	pub(crate) fn for_get() -> Self
	{
		Self
		{
			sdiag_family: AF_XDP as u8,
			sdiag_protocol: 0,
			pad: 0,
			xdiag_ino: 0,
			xdiag_show: XDP_SHOW_flags::BasicInformation | XDP_SHOW_flags::RingConfiguration | XDP_SHOW_flags::UserMemory | XDP_SHOW_flags::SocketMemoryInformation | XDP_SHOW_flags::Statistics,
			xdiag_cookie: [0, 0],
		}
	}
}
