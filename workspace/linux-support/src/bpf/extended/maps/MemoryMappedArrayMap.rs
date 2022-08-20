// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memory-mapped array.
///
/// When created, all its elements are zeroed.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryMappedArrayMap<V: Copy>
{
	array_map: ArrayMap<V>,
	mapped_memory: MappedMemory,
}

impl<V: Copy> CanBeInnerMap for MemoryMappedArrayMap<V>
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.array_map.map_file_descriptor()
	}
}

impl<V: Copy> MemoryMappedArrayMap<V>
{
	/// Capacity.
	#[inline(always)]
	pub fn capacity(&self) -> NonZeroU32
	{
		self.array_map.capacity()
	}
	
	/// Freeze.
	#[inline(always)]
	pub fn freeze(&self) -> Result<(), SystemCallErrorNumber>
	{
		self.array_map.freeze()
	}
	
	/// Indices.
	#[inline(always)]
	pub fn indices(&self) -> RangeInclusive<u32>
	{
		self.array_map.indices()
	}
	
	/// Element at index.
	///
	/// The element should be considered `MaybeUninit::zeroed()`.
	/// The element is not writable if the map was created with `AccessPermissions` that did not have permissions for user space write.
	#[inline(always)]
	pub fn element(&self, index: u32) -> NonNull<V>
	{
		debug_assert!(index < self.capacity().get());
		
		self.mapped_memory.virtual_address().aligned_pointer_to_value((index as usize) * size_of::<V>())
	}
	
	/// New instance.
	#[inline(always)]
	pub fn new(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		let array_map = ArrayMap::new_system_wide_internal(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, maximum_entries, access_permissions, numa_node, MemoryMap::MemoryMap)?;
		
		let length = PageSize::default().number_of_bytes_rounded_up_to_multiple_of_page_size((maximum_entries.to_u32() as u64) * (size_of::<V>() as u64));
		let length = new_non_zero_u64(length);
		
		use self::AccessPermissions::*;
		use self::Protection::*;
		let protection = match access_permissions
		{
			KernelReadUserspaceWrite => ReadWrite,
			KernelReadUserspaceReadWrite => ReadWrite,
			KernelWriteUserspaceRead => Read,
			KernelWriteUserspaceReadWrite => ReadWrite,
			KernelReadAndWriteUserspaceRead => Read,
			KernelReadAndWriteUserspaceWrite => ReadWrite,
			KernelReadAndWriteUserspaceReadWrite => ReadWrite,
		};
		
		let page_size_or_huge_page_size_settings = PageSizeOrHugePageSizeSettings::for_default_page_size();
		let mapped_memory = MappedMemory::from_file(array_map.map_file_descriptor.deref(), 0, length, AddressHint::any(), protection, Sharing::Shared, false, false, &page_size_or_huge_page_size_settings).unwrap();
		
		Ok
		(
			Self
			{
				array_map,
				mapped_memory,
			}
		)
	}
}
