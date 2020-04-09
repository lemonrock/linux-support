// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn without_suffix<'a>(name: &'a [u8], suffix: &'static [u8]) -> (&'a [u8], bool)
{
	let suffix_starts_at = name.len() - suffix.len();
	if &name[suffix_starts_at .. ] == suffix
	{
		(&name[0 .. suffix_starts_at], true)
	}
	else
	{
		(name, false)
	}
}
