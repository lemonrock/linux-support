// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This function is missing from Rust as of version 1.51.
///
/// Internally, sets `CF` (Carry Flag) in `EFLAGS` by doing `b AND NOT a == 0` (Intel calls this `ANDN`).
/// Hence the `c` after `ktest` in the intrinsic function's name.
#[cfg(target_feature = "avx512dq")]
#[inline(always)]
pub unsafe fn _ktestc_mask8_u8(a: __mmask8, b: __mmask8) -> bool
{
	let mut out: i8;
	asm!
	(
		"ktestb {k0} {k1}",
		"setb {cf}",
		
		k0 = in(kreg) a,
		k1 = in(kreg) b,
		cf = out(reg_byte) out,
		
		options
		(
			pure,nomem,
			nostack,
		),
	);
	transmute(out)
}
