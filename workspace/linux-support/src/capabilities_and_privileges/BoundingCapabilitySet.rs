// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialze, Serialize)]
pub struct BoundingCapabilitySet(pub BitSet<Capability>);

impl BoundingCapabilitySet
{
	#[inline(always)]
	pub fn get_for_current_thread() -> Self
	{
		let mut set = BitSet::new();
		for capability in Self::iter()
		{
			if capability.is_in_current_thread_bounding_set().unwrap_or(false)
			{
				set.add(capability)
			}
		}
		Self(set)
	}

	/// Will fail if the thread doesn't have `Capability::SetProcessCapabilities`.
	#[inline(always)]
	pub fn retain(&self) -> Result<(), ()>
	{
		use self::Capability::SetProcessCapabilities;

		let current_thread = Self::get_for_current_thread();

		let mut drop_set_process_capabilities_last = false;
		for capability in Capability::iter()
		{
			if self.0.contains(capability)
			{
				continue
			}
			if !current_thread.contains(capability)
			{
				continue
			}
			if capability == SetProcessCapabilities
			{
				drop_set_process_capabilities_last = true;
			}
			else
			{
				capability.remove_from_current_thread_bounding_set()?
			}
		}

		if drop_set_process_capabilities_last
		{
			SetProcessCapabilities.remove_from_current_thread_bounding_set()?
		}

		Ok(())
	}
}
