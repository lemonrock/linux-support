// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Gives us 8 priority classes (`IOPRIO_CLASS`) with 13-bits of data for each class.
const IOPRIO_CLASS_SHIFT: u16 = 13;

const IOPRIO_PRIO_MASK: u16 = (1 << IOPRIO_CLASS_SHIFT) - 1;

/// `mask` is also called `ioprio`.
#[inline(always)]
pub(super) const fn IOPRIO_PRIO_CLASS(mask: u16) -> u16
{
	mask >> IOPRIO_CLASS_SHIFT
}

/// `mask` is also called `ioprio`.
#[inline(always)]
pub(super) const fn IOPRIO_PRIO_DATA(mask: u16) -> u16
{
	mask & IOPRIO_PRIO_MASK
}

/// Returns `mask` (also known as `ioprio`).
#[inline(always)]
pub(super) const fn IOPRIO_PRIO_VALUE(class: u16, data: u16) -> u16
{
	(class << IOPRIO_CLASS_SHIFT) | data
}
