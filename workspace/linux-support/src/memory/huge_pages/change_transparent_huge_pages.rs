// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Enable or disable transparent huge pages.
#[inline(always)]
pub fn change_transparent_huge_pages(enable_or_disable_transparent_huge_pages: bool) -> Result<(), Errno>
{
	let arg2 = if enable_or_disable_transparent_huge_pages
	{
		0
	}
	else
	{
		1
	};
	
	process_control_wrapper2(PR_SET_THP_DISABLE, arg2, result_must_be_zero, Err)
}
