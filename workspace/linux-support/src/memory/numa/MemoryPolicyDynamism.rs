// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Edge use cases.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum MemoryPolicyDynamism
{
	/// No dynamism.
	NoDynamism = 0,

	/// `MPOL_F_RELATIVE_NODES`.
	Relative = 0x4000,

	/// `MPOL_F_STATIC_NODES`.
	Static = 0x8000,
}

impl Default for MemoryPolicyDynamism
{
	#[inline(always)]
	fn default() -> Self
	{
		MemoryPolicyDynamism::NoDynamism
	}
}
