// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a set of resource limits.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct ResourceLimitsSet(HashMap<ResourceName, SoftAndHardResourceLimit>);

impl Default for ResourceLimitsSet
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::defaultish(ResourceLimit::Finite(ProcPath::default().maximum_number_of_open_file_descriptors().unwrap()))
	}
}

impl ResourceLimitsSet
{
	/// A generous default for resource limits suitable for a modern server.
	///
	/// Obtain `maximum_number_of_open_file_descriptors` from `ResourceLimit::maximum_number_of_open_file_descriptors()`.
	#[inline(always)]
	pub fn defaultish(maximum_number_of_open_file_descriptors: ResourceLimit) -> Self
	{
		let mut map = Self::defaultish_common(maximum_number_of_open_file_descriptors);

		use self::ResourceName::*;

		map.insert(MaximumSizeOfVirtualMemoryAddressSpaceInBytes, SoftAndHardResourceLimit::BothInfinite);
		map.insert(MaximumNumberOfBytesForPosixMessageQueues, SoftAndHardResourceLimit::BothZero);

		Self(map)
	}

	/// Applies the resource limits.
	#[inline(always)]
	pub fn change(&self)
	{
		for (resource_name, soft_and_hard_resource_limit) in &self.0
		{
			resource_name.set(soft_and_hard_resource_limit);
		}
	}

	#[inline(always)]
	fn defaultish_common(maximum_number_of_open_file_descriptors: ResourceLimit) -> HashMap<ResourceName, SoftAndHardResourceLimit>
	{
		// Ideally, these should be constants, but Rust's const fn is too limited and does not allow assert! or if.
		let _64_000: SoftAndHardResourceLimit = SoftAndHardResourceLimit::both(ResourceLimit::Finite(64_000));
		let _262_144: SoftAndHardResourceLimit = SoftAndHardResourceLimit::both(ResourceLimit::Finite(262_144));

		let mut map = HashMap::with_capacity(16);

		use self::ResourceName::*;

		map.insert(MaximumSizeOfFilesThatProcessCanCreateInBytes, SoftAndHardResourceLimit::BothInfinite);
		map.insert(CpuTimeLimitInSeconds, SoftAndHardResourceLimit::BothInfinite);
		map.insert(MaximumNumberOfFileDescriptors, SoftAndHardResourceLimit::both(maximum_number_of_open_file_descriptors));
		map.insert(MaximumSizeOfProcessResidentSetSizeInBytes, SoftAndHardResourceLimit::BothInfinite);
		map.insert(MaximumNumberOfProcessAndThreads, _64_000);
		map.insert(MaximumSizeOfACoreDumpFileInBytes, SoftAndHardResourceLimit::BothZero);

		map.insert(MaximumNumberOfBytesThatProcessCanMemLock, SoftAndHardResourceLimit::BothInfinite);
		map.insert(MaximumSizeOfProcessStackInBytes, _262_144); // 256Kb stacks!

		map
	}
}
