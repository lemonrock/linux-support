// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Returns the value of the `IA32_TSC_AUX` MSR.
#[inline(always)]
pub fn rdpid() -> IA32_TSC_AUX
{
	let mut pid: u64;
	unsafe
	{
		asm!
		(
			"rdpid {pid}",
			pid = out(reg) pid,
			options(att_syntax)
		)
	};
	IA32_TSC_AUX(pid as u32)
}
