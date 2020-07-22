// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Spin-lockable Cgroup storage.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpinLockableCgroupStorageMap<V: 'static + Copy + HasReflectionInformation>(CgroupStorageMap<SpinLockableValue<V>>);

impl<V: 'static + Copy + HasReflectionInformation> SpinLockableCgroupStorageMap<V>
{
	/// New system-wide.
	#[inline(always)]
	pub fn new_system_wide(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: &ParsedBpfTypeFormatMapData, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		let cgroup_storage_map = CgroupStorageMap::new_system_wide(map_file_descriptors, map_name, Some(parsed_bpf_type_format_map_data), maximum_entries, access_permissions, numa_node)?;
		Ok(Self(cgroup_storage_map))
	}
	
	/// Length.
	#[inline(always)]
	pub fn capacity(&self) -> NonZeroU32
	{
		self.0.capacity()
	}
	
	/// Freeze.
	#[inline(always)]
	pub fn freeze(&self) -> Result<(), Errno>
	{
		self.0.freeze()
	}
	
	/// Iterator of keys.
	#[inline(always)]
	pub fn keys(&self) -> Result<KeyIterator<bpf_cgroup_storage_key>, Errno>
	{
		self.0.keys()
	}
	
	/// Get.
	#[inline(always)]
	pub fn get(&self, key: &bpf_cgroup_storage_key) -> Option<SpinLockableValue<V>>
	{
		self.0.get(key)
	}
	
	/// Insert or set.
	#[inline(always)]
	pub fn insert_or_set(&self, key: &bpf_cgroup_storage_key, value: &SpinLockableValue<V>) -> Result<(), ()>
	{
		self.0.map_file_descriptor.insert_or_set(key, value, LockFlags::Lock)
	}
	
	/// Delete.
	#[inline(always)]
	pub fn delete(&self, key: &bpf_cgroup_storage_key) -> Result<bool, Errno>
	{
		self.0.delete(key)
	}
}
