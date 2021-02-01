// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Mapped memory configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MappedMemoryConfiguration
{
	/// Address hints.
	#[serde(default)] pub address_hint: AddressHint,

	/// Protection.
	#[serde(default)] pub protection: Protection,

	/// Sharing.
	#[serde(default)] pub sharing: Sharing,

	/// Huge page size choices.
	#[serde(default)] pub page_size_preference: PageSizePreference,

	/// Prefault (pre-populate).
	#[serde(default = "MappedMemoryConfiguration::prefault_default")] pub prefault: bool,

	/// Reserve swap space.
	#[serde(default)] pub reserve_swap_space: bool,

	/// NUMA memory policy.
	#[serde(default)] pub numa_memory_policy: Option<(SetMemoryPolicy, SetMemoryPolicyStrictness)>,

	/// Lock memory so it can't be swapped out?
	///
	/// Forces memory to be resident in RAM, which ensures overcommit can not occur.
	///
	/// Make sure the `rlimit()` is configured correctly before using this.
	#[serde(default = "MappedMemoryConfiguration::lock_default")] pub lock: Option<MemoryLockSettings>,
	
	/// Advice.
	///
	/// Defaults to `DontFork`.
	#[serde(default = "MappedMemoryConfiguration::advice_default")] pub advice: HashSet<MemoryAdvice>,
}

impl Default for MappedMemoryConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			address_hint: AddressHint::default(),
			protection: Protection::default(),
			sharing: Sharing::default(),
			page_size_preference: PageSizePreference::default(),
			prefault: Self::prefault_default(),
			reserve_swap_space: false,
			numa_memory_policy: None,
			lock: Self::lock_default(),
			advice: Self::advice_default(),
		}
	}
}

impl MappedMemoryConfiguration
{
	/// Turns into settings.
	#[inline(always)]
	pub fn into_settings(self, defaults: &DefaultPageSizeAndHugePageSizes) -> MappedMemorySettings
	{
		MappedMemorySettings::new(self, defaults)
	}
	
	#[inline(always)]
	fn anonymous_memory_map(&self, length: NonZeroU64, page_size_or_huge_page_size_settings: &PageSizeOrHugePageSizeSettings) -> Result<MappedMemory, MemoryMapError>
	{
		let mapped_memory = MappedMemory::anonymous(length, self.address_hint, self.protection, self.sharing, self.prefault, self.reserve_swap_space, page_size_or_huge_page_size_settings)?;
		self.configure(mapped_memory)
	}

	#[inline(always)]
	fn from_file_memory_map<F: MemoryMappableFileDescriptor>(&self, file_descriptor: &F, offset: u64, length: NonZeroU64, page_size_or_huge_page_size_settings: &PageSizeOrHugePageSizeSettings) -> Result<MappedMemory, MemoryMapError>
	{
		let mapped_memory = MappedMemory::from_file(file_descriptor, offset, length, self.address_hint, self.protection, self.sharing, self.prefault, self.reserve_swap_space, page_size_or_huge_page_size_settings)?;
		self.configure(mapped_memory)
	}

	#[inline(always)]
	fn configure(&self, mapped_memory: MappedMemory) -> Result<MappedMemory, MemoryMapError>
	{
		use self::MemoryMapError::*;

		if let Some((set_memory_policy, set_memory_policy_strictness)) = self.numa_memory_policy.as_ref()
		{
			set_memory_policy_strictness.set_memory_address_policy(set_memory_policy, mapped_memory.virtual_address().into(), mapped_memory.mapped_size_in_bytes()).map_err(|_: ()| CouldNotSetNumaMemoryPolicy)?
		}

		if let Some(lock) = self.lock
		{
			let all_locked = mapped_memory.lock(lock).map_err(|cause| CouldNotLockMemory(cause, lock))?;
			if !all_locked
			{
				return Err(CouldNotLockAllMappedMemory)
			}
		}

		for &advice in self.advice.iter()
		{
			mapped_memory.advise(advice).map_err(|cause| CouldNotApplyMemoryAdvice(cause, advice))?
		}

		Ok(mapped_memory)
	}
	
	#[inline(always)]
	const fn prefault_default() -> bool
	{
		true
	}
	
	#[inline(always)]
	const fn lock_default() -> Option<MemoryLockSettings>
	{
		Some(MemoryLockSettings::Normal)
	}
	
	#[inline(always)]
	fn advice_default() -> HashSet<MemoryAdvice>
	{
		use self::MemoryAdvice::*;
		
		fast_secure_hash_set!
		{
			DontFork
		}
	}
}
