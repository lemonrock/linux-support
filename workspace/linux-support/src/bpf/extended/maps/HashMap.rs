// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// When a hash map is created it is empty.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HashMap<K: Copy, V: Copy>
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
	marker: PhantomData<(K, V)>,
}

impl<K: Copy, V: Copy> CanBeInnerMap for HashMap<K, V>
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.map_file_descriptor
	}
}

impl<K: Copy, V: Copy> HashMap<K, V>
{
	/// New per-device.
	#[inline(always)]
	pub fn new_per_device(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, device: NetworkInterfaceIndex) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::HashPerDevice(Self::key_size(), Self::value_size(), maximum_entries, access_permissions, device), maximum_entries)
	}
	
	/// New system-wide.
	#[inline(always)]
	pub fn new_system_wide(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, preallocation: Preallocation, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::HashSystemWide(Self::key_size(), Self::value_size(), maximum_entries, access_permissions, numa_node, preallocation), maximum_entries)
	}
	
	/// New least-recently used (LRU) hash with a LRU list shared amongst all HyperThreads.
	#[inline(always)]
	pub fn new_least_recently_used_system_wide(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::LeastRecentlyUsedHashSystemWide(Self::key_size(), Self::value_size(), maximum_entries, access_permissions, numa_node), maximum_entries)
	}
	
	/// New least-recently used (LRU) hash with a LRU list per HyperThread.
	#[inline(always)]
	pub fn new_least_recently_used_system_wide_with_a_per_hyper_thread_least_recently_used_list(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::LeastRecentlyUsedHashSystemWideWithAPerHyperThreadLeastRecentlyUsedList(Self::key_size(), Self::value_size(), maximum_entries, access_permissions,), maximum_entries)
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
	pub fn keys(&self) -> Result<KeyIterator<K>, Errno>
	{
		KeyIterator::new(&self.map_file_descriptor)
	}
	
	/// Get, batched.
	///
	/// Use `None` for `batch_position` when starting a new batch.
	/// Each value in `keys` must be valid.
	#[inline(always)]
	pub fn get_batch(&self, batch_position: Option<&OpaqueBatchPosition<K>>, keys: &[K]) -> Result<(Vec<V>, OpaqueBatchPosition<K>, bool), Errno>
	{
		self.guard_keys(keys);
		
		self.map_file_descriptor.get_batch(batch_position, keys)
	}
	
	/// Get and delete, batched.
	#[inline(always)]
	pub fn get_and_delete_batch(&self, batch_position: Option<&OpaqueBatchPosition<K>>, keys: &[K]) -> Result<(Vec<V>, OpaqueBatchPosition<K>, bool), Errno>
	{
		self.guard_keys(keys);
		
		self.map_file_descriptor.get_and_delete_batch(batch_position, keys)
	}
	
	/// Set, batched.
	///
	/// `keys` and `values` must be the same length.
	/// Each value in `keys` must be valid.
	#[inline(always)]
	pub fn set_batch(&self, keys: &[K], values: &[V]) -> Result<usize, Errno>
	{
		self.guard_keys_and_values(keys, values);
		
		self.map_file_descriptor.set_batch(keys, values, LockFlags::DoNotLock)
	}
	
	/// Delete, batched.
	#[inline(always)]
	pub fn delete_batch(&self, keys: &[K]) -> Result<(usize, bool), Errno>
	{
		self.map_file_descriptor.delete_batch(keys)
	}
	
	/// Looks up a key.
	#[inline(always)]
	pub fn get(&self, key: &K) -> Option<V>
	{
		self.map_file_descriptor.get(key)
	}
	
	/// Insert or set (overwrite).
	#[inline(always)]
	pub fn insert_or_set(&self, key: &K, value: &V) -> Result<(), ()>
	{
		self.map_file_descriptor.insert_or_set(key, value, LockFlags::DoNotLock)
	}
	
	/// Insert or fail.
	#[inline(always)]
	pub fn insert(&self, key: &K, value: &V) -> Result<(), InsertError>
	{
		self.map_file_descriptor.insert(key, value, LockFlags::DoNotLock)
	}
	
	/// Set (overwrite).
	#[inline(always)]
	pub fn set(&self, key: &K, value: &V) -> Result<(), ()>
	{
		self.map_file_descriptor.set(key, value, LockFlags::DoNotLock)
	}
	
	/// Delete.
	///
	/// Returns `Ok(true)` if `key` was present.
	#[inline(always)]
	pub fn delete(&self, key: &K) -> Result<bool, Errno>
	{
		self.map_file_descriptor.delete(key)
	}
	
	#[inline(always)]
	fn guard_keys_and_values(&self, keys: &[K], values: &[V])
	{
		self.guard_keys(keys);
		
		debug_assert_eq!(keys.len(), values.len())
	}
	
	#[inline(always)]
	fn guard_keys(&self, keys: &[K])
	{
		assert_ne!(keys.len(), 0);
		debug_assert!(keys.len() <= self.maximum_entries.to_u32() as usize)
	}
	
	#[inline(always)]
	fn create(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, map_type: MapType, maximum_entries: MaximumEntries) -> Result<Self, MapCreationError>
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
	fn value_size() -> ValueSizeU32
	{
		ValueSizeU32::try_from_value_size::<V>().unwrap()
	}
	
	#[inline(always)]
	fn key_size() -> KeySize
	{
		KeySize::try_from_key_size::<K>().unwrap()
	}
}
