// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Similar to `NonZeroI128::new_unchecked()` but checks are made when compiling with debug assertions.
pub const fn new_non_zero_i128(value: i128) -> NonZeroI128
{
	if cfg!(debug_assertions)
	{
		if value == 0
		{
			panic!("Zero for NonZeroI128")
		}
	}
	
	unsafe { NonZeroI128::new_unchecked(value) }
}
