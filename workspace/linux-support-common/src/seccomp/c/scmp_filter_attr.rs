// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum scmp_filter_attr
{
	#[doc(hidden)]
	_SCMP_FLTATR_MIN = 0,

	/// Default filter action (read only).
	SCMP_FLTATR_ACT_DEFAULT = 1,

	/// Bad architecture action.
	SCMP_FLTATR_ACT_BADARCH = 2,

	/// Set `NO_NEW_PRIVS` on filter load.
	SCMP_FLTATR_CTL_NNP = 3,

	/// Synchronize threads on filter load.
	SCMP_FLTATR_CTL_TSYNC = 4,

	/// Allow rules that specify syscall as `-1`.
	SCMP_FLTATR_API_TSKIP = 5,

	/// Log not-allowed actions.
	SCMP_FLTATR_CTL_LOG = 6,

	/// Disable Speculative Store Bypass mitigation.
	SCMP_FLTATR_CTL_SSB = 7,

	#[doc(hidden)]
	_SCMP_FLTATR_MAX,
}
