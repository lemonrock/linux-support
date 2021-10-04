// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "movbe"))]
#[inline(always)]
fn load_movbe_16(bytes: &Unaligned16) -> u16
{
	let source_memory = bytes.pointer();
	let out;
	unsafe
	{
		asm!
		(
			"movbe {destination_register:x}, [{source_memory_location}]",
			destination_register = out(reg) out,
			source_memory_location = in(reg) source_memory,
			options(pure, preserves_flags, readonly, nostack)
		);
	}
	out
}
