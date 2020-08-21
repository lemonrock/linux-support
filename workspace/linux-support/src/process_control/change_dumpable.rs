// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Needs to be called after process change.
///
/// Needs to be called after:-
///
/// * Changing effective user identifier;
/// * Changing effective group identifier;
/// * Changing file system user identifier;
/// * Changing file system group identifier;
/// * After `execve()`
#[inline(always)]
pub fn change_dumpable(enable_or_disable_dumpable: bool) -> Result<(), Errno>
{
	let value = if enable_or_disable_dumpable
	{
		1
	}
	else
	{
		0
	};
	
	process_control_wrapper2(PR_SET_DUMPABLE, value, result_must_be_zero, Err)
}
