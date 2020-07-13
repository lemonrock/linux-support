// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memory-mapped array.
///
/// When created, all its elements are zeroed.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryMappedArrayMap<'map_file_descriptor_label_map, V: Sized>
{
	array_map: ArrayMap<'map_file_descriptor_label_map, V>,
	mapped_memory: MappedMemory,
}

impl<'map_file_descriptor_label_map, V: Sized> MemoryMappedArrayMap<'map_file_descriptor_label_map, V>
{
	/// Capacity.
	#[inline(always)]
	pub fn capacity(&self) -> NonZeroU32
	{
		self.array_map.capacity()
	}
	
	/// Element at index.
	///
	/// The element should be considered `MaybeUninit::zeroed()`.
	/// The element is not writable if the map was created with `AccessPermissions` that did not have permissions for user space write.
	#[inline(always)]
	pub fn element(&self, index: u32) -> NonNull<V>
	{
		debug_assert!(index < self.capacity().get());
		
		self.mapped_memory.virtual_address().pointer_to((index as usize) * size_of::<V>())
	}
	
	/// New instance.
	#[inline(always)]
	pub fn new(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, value_size: ValueSizeU32, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, MapCreationError>
	{
		let array_map = ArrayMap::new_system_wide_internal(map_file_descriptors, map_name, parsed_btf_map_data, value_size, maximum_entries, access_permissions, numa_node, MemoryMap::MemoryMap)?;
		
		let length = defaults.default_page_size().number_of_bytes_rounded_up_to_multiple_of_page_size((maximum_entries.to_u32() as u64) * (size_of::<V>() as u64));
		let length = unsafe { NonZeroU64::new_unchecked(length) };
		
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
		
		let mapped_memory = MappedMemory::from_file(array_map.map_file_descriptor, 0, length, AddressHint::any(), protection, Sharing::Shared, None, false, false, defaults).unwrap();
		
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
