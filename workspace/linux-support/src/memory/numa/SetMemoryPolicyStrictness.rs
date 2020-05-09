// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used to set a memory policy.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum SetMemoryPolicyStrictness
{
	/// Just set policy.
	JustSetPolicy,

	/// Set policy and move pages if possible.
	SetPolicyAndMovePagesIfPossible,

	/// Set policy and move pages; fail if moving pages failed.
	SetPolicyAndMovePagesOrFail,
}

impl Default for SetMemoryPolicyStrictness
{
	#[inline(always)]
	fn default() -> Self
	{
		SetMemoryPolicyStrictness::SetPolicyAndMovePagesIfPossible
	}
}

impl SetMemoryPolicyStrictness
{
	/// Set memory address policy.
	#[inline(always)]
	pub fn set_memory_address_policy(&self, set_memory_policy: &SetMemoryPolicy, address: NonNull<u8>, length: usize) -> Result<(), ()>
	{
		use self::SetMemoryPolicyStrictness::*;

		match self
		{
			&JustSetPolicy => set_memory_policy.set_address_policy(address, length),

			&SetPolicyAndMovePagesIfPossible => set_memory_policy.set_address_policy_and_move_pages_if_possible(address, length),

			&SetPolicyAndMovePagesOrFail => set_memory_policy.set_address_policy_and_move_pages(address, length)?,
		}
		Ok(())
	}
}
