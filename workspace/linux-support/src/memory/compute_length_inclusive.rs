// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn compute_length_inclusive(exclusive_start: usize, inclusive_end: usize, original_length: usize) -> usize
{
	let length = inclusive_end.saturating_add(1).saturating_sub(exclusive_start);
	debug_assert!(length <= original_length);
	length
}
