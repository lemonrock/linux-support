// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// splitn.
#[inline(always)]
pub fn splitn<'a>(slice: &'a [u8], n: usize, predicate: u8) -> ::std::slice::SplitN<'a, u8, impl FnMut(&u8) -> bool>
{
	slice.splitn(n, move |value| *value == predicate)
}
