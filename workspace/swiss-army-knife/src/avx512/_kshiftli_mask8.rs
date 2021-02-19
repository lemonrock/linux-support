// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This function is missing from Rust as of version 1.51.
///
#[cfg(target_feature = "avx512dq")]
#[inline(always)]
pub unsafe fn _kshiftli_mask8(a: __mmask8, count: u32) -> __mmask8
{
	macro_rules! _kshiftli_mask8_constified_count
	{
		($a: ident, $count: expr) =>
		{
			{
				let a = $a;
				const count: u8 = $count;
				
				let mut out: __mmask8;
				asm!
				(
					"kshiftlb {k} {k}, {count}",
				
					k = inlateout(kreg) a => out,
					count = const count,
					
					options
					(
						pure,nomem,
						preserves_flags,
						nostack,
					),
				);
				out
			}
		}
	}
	
	match count
	{
		0 => _kshiftli_mask8_constified_count!(a, 0),
		1 => _kshiftli_mask8_constified_count!(a, 1),
		2 => _kshiftli_mask8_constified_count!(a, 2),
		3 => _kshiftli_mask8_constified_count!(a, 3),
		4 => _kshiftli_mask8_constified_count!(a, 4),
		5 => _kshiftli_mask8_constified_count!(a, 5),
		6 => _kshiftli_mask8_constified_count!(a, 6),
		7 => _kshiftli_mask8_constified_count!(a, 7),
		_ => _kshiftli_mask8_constified_count!(a, 8),
	}
}
