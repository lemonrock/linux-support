// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// User return codes for XDP programs.
///
/// A valid XDP program must return one of these defined values.
/// All other return codes are reserved for future use.
/// Unknown return codes will result in packet drops and a warning via `bpf_warn_invalid_xdp_action()`.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum xdp_action
{
	XDP_ABORTED = 0,
	XDP_DROP = 1,
	XDP_PASS = 2,
	XDP_TX = 3,
	XDP_REDIRECT = 4,
}

impl Into<Immediate<'static>> for xdp_action
{
	#[inline(always)]
	fn into(self) -> Immediate<'static, i32>
	{
		Immediate(Offset::Known(self as u32 as i32))
	}
}
