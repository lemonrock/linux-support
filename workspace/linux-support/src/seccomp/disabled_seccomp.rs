// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Seccomp is disabled after this call.
#[inline(always)]
pub fn disabled_seccomp() -> Result<(), Errno>
{
	process_control_wrapper2(PR_SET_SECUREBITS,SECCOMP_MODE_DISABLED as usize,result_must_be_zero,Err)
}
