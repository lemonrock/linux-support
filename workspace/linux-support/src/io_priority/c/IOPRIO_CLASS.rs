// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// These are the io priority groups as implemented by the Completely Fair Queueing (CFQ) scheduler:-
///
/// * RT is the realtime class, it always gets premium service. Trumps BE.
/// * BE is the best-effort scheduling class, the default for any process. Trumps IDLE.
/// * IDLE is the idle scheduling class, it is only served when no one else is using the disk.
///
/// There are a maximum of 8 priority classes in the top 3 bits of the `ioprio`.
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub(super) enum IOPRIO_CLASS
{
	#[allow(dead_code)]
	IOPRIO_CLASS_NONE = 0,

	IOPRIO_CLASS_RT = 1,

	IOPRIO_CLASS_BE = 2,

	IOPRIO_CLASS_IDLE = 3,
}

impl Ord for IOPRIO_CLASS
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		(*self as u8).cmp(&(*other as u8)).reverse()
	}
}

impl PartialOrd for IOPRIO_CLASS
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}
