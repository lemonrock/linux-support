// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Memory policy details.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NumaMemoryPolicyDetails
{
	/// Memory policy.
	pub set_memory_policy: SetMemoryPolicy,

	/// Dynamism.
	///
	/// Any `NumaNode` or `BitSet<NumaNode>` in `SetMemoryPolicy` has a different meaning to what might be expected!
	pub memory_policy_dynamism: MemoryPolicyDynamism,
}

impl NumaMemoryPolicyDetails
{
	#[inline(always)]
	const fn new(set_memory_policy: SetMemoryPolicy, memory_policy_dynamism: MemoryPolicyDynamism) -> Self
	{
		Self
		{
			set_memory_policy,
			memory_policy_dynamism,
		}
	}
}
