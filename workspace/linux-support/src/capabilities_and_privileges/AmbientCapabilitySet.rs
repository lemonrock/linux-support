// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Ambient capability set.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct AmbientCapabilitySet(pub Capabilities);

impl AmbientCapabilitySet
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn get_for_current_thread() -> Self
	{
		let mut set = BitSet::new();
		for capability in Capability::iter()
		{
			if capability.is_in_current_thread_ambient_set().unwrap_or(false)
			{
				set.add(capability)
			}
		}
		Self(Capabilities(set))
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn set_for_current_thread(&self) -> Result<(), AmbientCapabilityError>
	{
		let was = Self::get_for_current_thread();
		for capability in Capability::iter()
		{
			let old = was.0.contains(capability);
			let new = self.0.contains(capability);
			match (old, new)
			{
				(true, false) => capability.remove_from_current_thread_ambient_set()?,
				(false, true) => capability.add_to_current_thread_ambient_set()?,
				(true, true) | (false, false) => continue,
			}
		}
		Ok(())
	}

	/// Clears all ambient capabilities from the current thread.
	#[inline(always)]
	pub fn clear_current_thread_ambient_set()
	{
		let result: Result<(), io::Error> = process_control_wrapper2
		(
			PR_CAP_AMBIENT,
			PR_CAP_AMBIENT_CLEAR_ALL as usize,
			|non_negative_result| if likely!(non_negative_result == 0)
			{
				Ok(())
			}
			else
			{
				unreachable!("Positive result")
			},
		|error_number| panic!("Unexpected error code '{}' from prctl()", error_number)
		);
		result.unwrap()
	}
}
