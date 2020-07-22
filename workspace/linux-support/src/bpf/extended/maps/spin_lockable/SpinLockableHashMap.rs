// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// When a hash map is created it is empty.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SpinLockableHashMap<K: Copy, V: 'static + Copy + HasReflectionInformation>(crate::bpf::extended::maps::HashMap<K, SpinLockableValue<V>>);

impl<K: Copy, V: 'static + Copy + HasReflectionInformation> SpinLockableHashMap<K, V>
{
	/// New system-wide.
	#[inline(always)]
	pub fn new_system_wide(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: &ParsedBpfTypeFormatMapData, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, preallocation: Preallocation, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		let hash_map = crate::bpf::extended::maps::HashMap::new_system_wide(map_file_descriptors, map_name, Some(parsed_bpf_type_format_map_data), maximum_entries, access_permissions, preallocation, numa_node)?;
		Ok(Self(hash_map))
	}
	
	/// Capacity.
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
	pub fn keys(&self) -> Result<KeyIterator<K>, Errno>
	{
		self.0.keys()
	}
	
	/// Get, batched.
	///
	/// Use `None` for `batch_position` when starting a new batch.
	/// Each value in `keys` must be valid.
	#[inline(always)]
	pub fn get_batch(&self, batch_position: Option<&OpaqueBatchPosition<K>>, keys: &[K]) -> Result<(Vec<SpinLockableValue<V>>, OpaqueBatchPosition<K>, bool), Errno>
	{
		self.0.get_batch(batch_position, keys)
	}
	
	/// Delete, batched.
	#[inline(always)]
	pub fn delete_batch(&self, keys: &[K]) -> Result<(usize, bool), Errno>
	{
		self.0.delete_batch(keys)
	}
	
	/// Looks up a key.
	#[inline(always)]
	pub fn get(&self, key: &K) -> Option<SpinLockableValue<V>>
	{
		self.0.get(key)
	}
	
	/// Insert or set (overwrite).
	#[inline(always)]
	pub fn insert_or_set(&self, key: &K, value: &SpinLockableValue<V>) -> Result<(), ()>
	{
		self.0.map_file_descriptor.insert_or_set(key, value, LockFlags::Lock)
	}
	
	/// Insert or fail.
	#[inline(always)]
	pub fn insert(&self, key: &K, value: &SpinLockableValue<V>) -> Result<(), InsertError>
	{
		self.0.map_file_descriptor.insert(key, value, LockFlags::Lock)
	}
	
	/// Set (overwrite).
	#[inline(always)]
	pub fn set(&self, key: &K, value: &SpinLockableValue<V>) -> Result<(), ()>
	{
		self.0.map_file_descriptor.set(key, value, LockFlags::Lock)
	}
	
	/// Delete.
	///
	/// Returns `Ok(true)` if `key` was present.
	#[inline(always)]
	pub fn delete(&self, key: &K) -> Result<bool, Errno>
	{
		self.0.delete(key)
	}
	
	/// Set, batched.
	///
	/// `keys` and `values` must be the same length.
	/// Each value in `keys` must be valid.
	#[inline(always)]
	pub fn set_batch(&self, keys: &[K], values: &[SpinLockableValue<V>]) -> Result<usize, Errno>
	{
		self.0.guard_keys_and_values(keys, values);
		
		self.0.map_file_descriptor.set_batch(keys, values, LockFlags::Lock)
	}
}
