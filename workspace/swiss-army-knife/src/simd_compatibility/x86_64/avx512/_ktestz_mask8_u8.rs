// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This function is missing from Rust as of version 1.51.
///
/// Internally, sets `ZF` (Zero Flag) in `EFLAGS` by doing `b AND a == 0`.
/// Hence the `z` after `ktest` in the intrinsic function's name.
#[cfg(target_feature = "avx512dq")]
#[inline(always)]
pub unsafe fn _ktestz_mask8_u8(a: __mmask8, b: __mmask8) -> bool
{
	let mut out: i8;
	asm!
	(
		"ktestb {k0} {k1}",
		"sete {zf}",
		
		k0 = in(kreg) a,
		k1 = in(kreg) b,
		zf = out(reg_byte) out,

		options
		(
			pure,nomem,
			nostack,
		),
	);
	transmute(out)
}
