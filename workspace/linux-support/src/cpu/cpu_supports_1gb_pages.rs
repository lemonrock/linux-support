// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Only really has meaning for x86-64.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub fn cpu_supports_1gb_pages() -> bool
{
	CpuId::new().get_extended_function_info().unwrap().has_1gib_pages()
}

/// Only really has meaning for x86-64.
#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
pub fn cpu_supports_1gb_pages() -> bool
{
	false
}
