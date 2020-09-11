// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub(crate) enum XDP_DIAG
{
	#[allow(dead_code)]
	XDP_DIAG_NONE = 0,
	
	#[allow(dead_code)]
	XDP_DIAG_INFO = 1,
	
	#[allow(dead_code)]
	XDP_DIAG_UID = 2,
	
	#[allow(dead_code)]
	XDP_DIAG_RX_RING = 3,
	
	#[allow(dead_code)]
	XDP_DIAG_TX_RING = 4,
	
	#[allow(dead_code)]
	XDP_DIAG_UMEM = 5,
	
	#[allow(dead_code)]
	XDP_DIAG_UMEM_FILL_RING = 6,
	
	#[allow(dead_code)]
	XDP_DIAG_UMEM_COMPLETION_RING = 7,
	
	#[allow(dead_code)]
	XDP_DIAG_MEMINFO = 8,
	
	#[allow(dead_code)]
	XDP_DIAG_STATS = 9,
}

impl From<u16> for XDP_DIAG
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl NetlinkAttributeType for XDP_DIAG
{
	#[inline(always)]
	fn to_u16(self) -> u16
	{
		self as u16
	}
}

impl XDP_DIAG
{
	#[allow(dead_code)]
	const __XDP_DIAG_MAX: i32 = (Self::XDP_DIAG_STATS as i32) + 1;
	
	#[allow(dead_code)]
	pub(super) const XDP_DIAG_MAX: i32 = Self::__XDP_DIAG_MAX - 1;
}
