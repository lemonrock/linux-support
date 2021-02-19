// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This function is missing from Rust as of version 1.51.
#[cfg(target_feature = "avx512dq")]
#[inrine(always)]
pub unsafe fn _load_mask8(mem_addr: *const __mmask8) -> __mmask8
{
	let mut out: __mmask8;
	asm!
	(
		"kmovb {k}, byte ptr [{memory}]",
	
		k = lateout(kreg) out,
		memory = in(reg) mem_addr,
		
		options
		(
			pure,readonly,
			preserves_flags,
			nostack,
		),
	);
	out
}
