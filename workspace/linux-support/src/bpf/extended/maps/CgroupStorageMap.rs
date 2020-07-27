// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Cgroup storage.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CgroupStorageMap<V: Copy>
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
	marker: PhantomData<V>,
}

impl<V: Copy> CgroupStorageMap<V>
{
	/// New system-wide.
	#[inline(always)]
	pub fn new_system_wide(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::CgroupStorageSystemWide(Self::value_size(), access_permissions, numa_node), maximum_entries)
	}
	
	/// Length.
	#[inline(always)]
	pub fn capacity(&self) -> NonZeroU32
	{
		self.maximum_entries.0
	}
	
	/// Freeze.
	#[inline(always)]
	pub fn freeze(&self) -> Result<(), Errno>
	{
		self.map_file_descriptor.freeze()
	}
	
	/// Iterator of keys.
	#[inline(always)]
	pub fn keys(&self) -> Result<KeyIterator<bpf_cgroup_storage_key>, Errno>
	{
		KeyIterator::new(&self.map_file_descriptor)
	}
	
	/// Get.
	#[inline(always)]
	pub fn get(&self, key: &bpf_cgroup_storage_key) -> Option<V>
	{
		self.map_file_descriptor.get(key)
	}
	
	/// Insert or set.
	#[inline(always)]
	pub fn insert_or_set(&self, key: &bpf_cgroup_storage_key, value: &V) -> Result<(), ()>
	{
		self.map_file_descriptor.insert_or_set(key, value, LockFlags::DoNotLock)
	}
	
	/// Delete.
	#[inline(always)]
	pub fn delete(&self, key: &bpf_cgroup_storage_key) -> Result<bool, Errno>
	{
		self.map_file_descriptor.delete(key)
	}
	
	#[inline(always)]
	fn create(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, map_type: MapType, maximum_entries: MaximumEntries) -> Result<Self, MapCreationError>
	{
		MapFileDescriptor::create(map_file_descriptors, map_type, map_name, parsed_bpf_type_format_map_data).map(|map_file_descriptor| Self::new(map_file_descriptor, maximum_entries))
	}
	
	#[inline(always)]
	const fn new(map_file_descriptor: Rc<MapFileDescriptor>, maximum_entries: MaximumEntries) -> Self
	{
		Self
		{
			map_file_descriptor,
			maximum_entries,
			marker: PhantomData
		}
	}
	
	#[inline(always)]
	fn value_size() -> ValueSizeU16
	{
		ValueSizeU16::try_from_value_size::<V>().unwrap()
	}
}
