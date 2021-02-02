// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Mapped memory settings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappedMemorySettings
{
	page_size_or_huge_page_size_settings: PageSizeOrHugePageSizeSettings,
	
	configuration: MappedMemoryConfiguration,
}

impl MappedMemorySettings
{
	/// Create a new instance.
	#[inline(always)]
	pub fn new(configuration: MappedMemoryConfiguration, defaults: &DefaultHugePageSizes) -> Self
	{
		Self
		{
			page_size_or_huge_page_size_settings: PageSizeOrHugePageSizeSettings::from(configuration.page_size_preference, defaults),
			configuration,
		}
	}
	
	/// Anonymous memory map.
	#[inline(always)]
	pub fn anonymous_memory_map(&self, length: NonZeroU64) -> Result<MappedMemory, MemoryMapError>
	{
		self.configuration.anonymous_memory_map(length, &self.page_size_or_huge_page_size_settings)
	}

	/// From file memory map.
	#[inline(always)]
	pub fn from_file_memory_map<F: MemoryMappableFileDescriptor>(&self, file_descriptor: &F, offset: u64, length: NonZeroU64) -> Result<MappedMemory, MemoryMapError>
	{
		self.configuration.from_file_memory_map(file_descriptor, offset, length, &self.page_size_or_huge_page_size_settings)
	}
}
