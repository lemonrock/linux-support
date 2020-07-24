// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(i32)]
pub(super) enum XDP_DIAG
{
	XDP_DIAG_NONE = 0,
	XDP_DIAG_INFO = 1,
	XDP_DIAG_UID = 2,
	XDP_DIAG_RX_RING = 3,
	XDP_DIAG_TX_RING = 4,
	XDP_DIAG_UMEM = 5,
	XDP_DIAG_UMEM_FILL_RING = 6,
	XDP_DIAG_UMEM_COMPLETION_RING = 7,
	XDP_DIAG_MEMINFO = 8,
}

impl XDP_DIAG
{
	const __XDP_DIAG_MAX: i32 = (Self::XDP_DIAG_MEMINFO as i32) + 1;
	
	pub(super) const XDP_DIAG_MAX: i32 = Self::__XDP_DIAG_MAX - 1;
}
