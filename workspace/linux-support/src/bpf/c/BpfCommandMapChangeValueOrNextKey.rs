// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Copy, Clone)]
pub(crate) union BpfCommandMapChangeValueOrNextKey
{
	pub(crate) value: AlignedU64,
	
	pub(crate) next_key: AlignedU64,
}

impl Default for BpfCommandMapChangeValueOrNextKey
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe_zeroed()
	}
}

impl Debug for BpfCommandMapChangeValueOrNextKey
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "BpfCommandMapChangeValueOrNextKey {{ {} }}", unsafe { self.value })
	}
}
