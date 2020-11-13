// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Thread local allocator configuration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ThreadLocalAllocatorConfiguration
{
	/// Thread local heap size.
	pub heap_size: NonZeroU64,
	
	/// Maximum bytes to waste when choosing a memory mapping?
	pub inclusive_maximum_bytes_wasted: u64,
	
	/// Use a strict NUMA memory policy for more efficiency?
	pub strict_numa_memory_policy: bool,
	
	/// Block size hint.
	pub block_size_hint: NonZeroUsize,
}

impl Default for ThreadLocalAllocatorConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			heap_size: new_non_zero_u64(1024 * 1024 * 1024),
			inclusive_maximum_bytes_wasted: 0,
			strict_numa_memory_policy: false,
			block_size_hint: new_non_zero_usize(64),
		}
	}
}

impl ThreadLocalAllocatorConfiguration
{
	/// Mapped memory settings.
	#[inline(always)]
	pub fn mapped_memory_settings(&self, defaults: &DefaultPageSizeAndHugePageSizes) -> MappedMemorySettings
	{
		MappedMemorySettings
		{
			address_hint: AddressHint::any(),
			protection: Protection::ReadWrite,
			sharing: Sharing::Private,
			huge_memory_page_size: self.huge_memory_page_size(defaults),
			prefault: true,
			reserve_swap_space: false,
			numa_memory_policy: self.numa_memory_policy(),
			lock: Some(MemoryLockSettings::Normal),
			advice: fast_secure_hash_set!
			{
				MemoryAdvice::DontFork
			}
		}
	}
	
	#[inline(always)]
	fn huge_memory_page_size(&self, defaults: &DefaultPageSizeAndHugePageSizes) -> Option<Option<HugePageSize>>
	{
		defaults.best_fit_huge_page_size_if_any(self.heap_size.get(), self.inclusive_maximum_bytes_wasted).map(|huge_page_size| Some(huge_page_size))
	}
	
	#[inline(always)]
	fn numa_memory_policy(&self) -> Option<(SetMemoryPolicy, SetMemoryPolicyStrictness)>
	{
		use self::SetMemoryPolicyStrictness::*;
		
		if self.strict_numa_memory_policy
		{
			Some((SetMemoryPolicy::BindCurrent(), SetPolicyAndMovePagesOrFail))
		}
		else
		{
			Some((SetMemoryPolicy::Local, JustSetPolicy))
		}
	}
}
