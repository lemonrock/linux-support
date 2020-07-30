// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// "These are stored into IFLA_XDP_ATTACHED on dump".
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub(crate) enum XDP_ATTACHED
{
	XDP_ATTACHED_NONE = 0,
	
	XDP_ATTACHED_DRV = 1,
	
	XDP_ATTACHED_SKB = 2,
	
	XDP_ATTACHED_HW = 3,
	
	XDP_ATTACHED_MULTI = 4,
}
