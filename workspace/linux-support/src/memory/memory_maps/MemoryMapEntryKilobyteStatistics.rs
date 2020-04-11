// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Statistics in kilobyte units.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryMapEntryKilobyteStatistics
{
	/// Resident set size.
	///
	/// This is the amount of mapping currently resident in RAM.
	///
	/// Known as `Rss` and `Pss`.
	pub resident_set_size: SizeAndProcessShareOfSize,

	/// Known as `Shared_Clean`, `Shared_Dirty` and `Shared_Hugetlb`.
	pub shared: CleanDirtyAndHuge,

	/// Known as `Private_Clean`, `Private_Dirty` and `Private_Hugetlb`.
	pub private: CleanDirtyAndHuge,

	/// Known as `Referenced`.
	///
	/// Amount of memory currently marked as referenced or accessed.
	pub referenced: Kilobyte,

	/// Known as `Anonymous`.
	///
	/// Will be zero for a file-backed mapping.
	pub anonymous: Kilobyte,

	/// Known as `AnonHugePages`.
	///
	/// Will be zero for a file-backed mapping.
	///
	/// See also `MemoryInformationName::TransparentHugePagesMemoryUsage`.
	pub anonymous_huge_pages: Kilobyte,

	/// Known as `LazyFree`.
	///
	/// See also `MemoryInformationName::LazyFree`.
	pub lazy_free: Kilobyte,

	/// Known as `ShmemPmdMapped`.
	///
	/// See also `MemoryInformationName::ShmemMemoryMappedIntoUserSpaceUsingHugePages`.
	pub shmem_memory_mapped_into_user_space_using_huge_pages: Kilobyte,

	/// Known as `FilePmdMapped`.
	pub file_memory_mapped_into_user_space_using_huge_pages: Kilobyte,

	/// Known as `Swap` and `SwapPss`.
	pub swap: SizeAndProcessShareOfSize,

	/// Known as `Locked`.
	pub locked: Kilobyte,

	/// Unknown.
	pub unknown: HashMap<Box<[u8]>, u64>,
}

impl AddAssign<&Self> for MemoryMapEntryKilobyteStatistics
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: &Self)
	{
		self.resident_set_size += &rhs.resident_set_size;
		self.shared += &rhs.shared;
		self.private += &rhs.private;
		self.referenced += rhs.referenced;
		self.anonymous += rhs.anonymous;
		self.anonymous_huge_pages += rhs.anonymous_huge_pages;
		self.lazy_free += rhs.lazy_free;
		self.shmem_memory_mapped_into_user_space_using_huge_pages += rhs.shmem_memory_mapped_into_user_space_using_huge_pages;
		self.file_memory_mapped_into_user_space_using_huge_pages += rhs.file_memory_mapped_into_user_space_using_huge_pages;
		self.swap += &rhs.swap;
		self.locked += rhs.locked;

		for (key, &kilobytes) in rhs.unknown.iter()
		{
			let value = self.unknown.get_mut(key);
			match value
			{
				None =>
				{
					self.unknown.insert(key.clone(), kilobytes);
				},

				Some(pointer) =>
				{
					*pointer = *pointer + kilobytes;
				}
			}
		}
	}
}
