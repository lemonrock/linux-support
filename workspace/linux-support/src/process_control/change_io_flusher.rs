// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Set IO flusher.
#[inline(always)]
pub fn change_io_flusher(enable_or_disable_io_flusher: bool) -> Result<(), Errno>
{
	let value = if enable_or_disable_io_flusher
	{
		1
	}
	else
	{
		0
	};
	process_control_wrapper2
	(
		PR_SET_IO_FLUSHER,
		value,
		result_must_be_zero,
		Err,
	)
}
