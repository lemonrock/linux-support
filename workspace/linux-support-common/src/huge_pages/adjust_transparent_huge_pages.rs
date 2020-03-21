// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Enable or disable transparent huge pages.
#[inline(always)]
fn adjust_transparent_huge_pages(enable_transparent_huge_pages: bool)
{
	let value = if enable_transparent_huge_pages
	{
		1
	}
	else
	{
		0
	};
	unsafe { prctl(PR_SET_THP_DISABLE, value as c_ulong) };
}
