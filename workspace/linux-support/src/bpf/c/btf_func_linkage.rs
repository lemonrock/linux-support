// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Function linkage.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum btf_func_linkage
{
	/// `BTF_FUNC_STATIC`.
	Static = 0,
	
	/// `BTF_FUNC_GLOBAL`.
	Global = 1,
	
	/// `BTF_FUNC_EXTERN`.
	Extern = 2,
}

impl Default for btf_func_linkage
{
	#[inline(always)]
	fn default() -> Self
	{
		btf_func_linkage::Static
	}
}
