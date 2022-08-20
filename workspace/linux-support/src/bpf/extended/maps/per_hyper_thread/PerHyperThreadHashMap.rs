// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// When a hash map is created it is empty.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerHyperThreadHashMap<K: Copy, V: Copy>
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
	number_of_possible_hyper_threads: NumberOfPossibleHyperThreads,
	marker: PhantomData<(K, V)>,
}

impl<K: Copy, V: Copy> CanBeInnerMap for PerHyperThreadHashMap<K, V>
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.map_file_descriptor
	}
}

impl<K: Copy, V: Copy> PerHyperThreadHashMap<K, V>
{
	/// New per-HyperThread.
	#[inline(always)]
	pub fn new_per_hyper_thread(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, number_of_possible_hyper_threads: NumberOfPossibleHyperThreads, preallocation: Preallocation) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::HashPerHyperThread(Self::key_size(), Self::value_size(), maximum_entries, access_permissions, preallocation), maximum_entries, number_of_possible_hyper_threads)
	}
	
	/// New least-recently used (LRU) hash with a LRU list shared amongst all HyperThreads.
	#[inline(always)]
	pub fn new_least_recently_used_per_hyper_thread(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, number_of_possible_hyper_threads: NumberOfPossibleHyperThreads) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::LeastRecentlyUsedHashPerHyperThread(Self::key_size(), Self::value_size(), maximum_entries, access_permissions), maximum_entries, number_of_possible_hyper_threads)
	}
	
	/// New least-recently used (LRU) hash with a LRU list per HyperThread.
	#[inline(always)]
	pub fn new_least_recently_used_per_hyper_thread_with_a_per_hyper_thread_least_recently_used_list(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, number_of_possible_hyper_threads: NumberOfPossibleHyperThreads) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::LeastRecentlyUsedHashPerHyperThreadWithAPerHyperThreadLeastRecentlyUsedList(Self::key_size(), Self::value_size(), maximum_entries, access_permissions,), maximum_entries, number_of_possible_hyper_threads)
	}
	
	/// Length.
	#[inline(always)]
	pub fn capacity(&self) -> NonZeroU32
	{
		self.maximum_entries.0
	}
	
	/// Freeze.
	#[inline(always)]
	pub fn freeze(&self) -> Result<(), SystemCallErrorNumber>
	{
		self.map_file_descriptor.freeze()
	}
	
	/// Iterator of keys.
	#[inline(always)]
	pub fn keys(&self) -> Result<KeyIterator<K>, SystemCallErrorNumber>
	{
		KeyIterator::new(&self.map_file_descriptor)
	}
	
	/// Allocates a reusable vector.
	#[inline(always)]
	pub unsafe fn allocate_values(&self) -> Vec<PerHyperThreadValue<V>>
	{
		self.number_of_possible_hyper_threads.uninitialized_per_hyper_thread_values()
	}
	
	/// Uses a reusable vector created with `self.allocate_values()`.
	///
	/// Returns a boolean with `true` for successfully found.
	#[inline(always)]
	pub unsafe fn get_values(&self, key: &K, allocate_values: &mut Vec<PerHyperThreadValue<V>>) -> bool
	{
		debug_assert_eq!(allocate_values.len(), self.values_length());
		self.map_file_descriptor.get_variably_sized_vector(key, allocate_values)
	}
	
	/// Uses a reusable vector created with `self.allocate_values()`.
	#[inline(always)]
	pub unsafe fn insert_or_set_values(&self, key: &K, allocate_values: &Vec<PerHyperThreadValue<V>>) -> Result<(), ()>
	{
		debug_assert_eq!(allocate_values.len(), self.values_length());
		self.map_file_descriptor.insert_or_set_variably_sized(key, &allocate_values[..])
	}
	
	/// Uses a reusable vector created with `self.allocate_values()`.
	#[inline(always)]
	pub unsafe fn insert_values(&self, key: &K, allocate_values: &Vec<PerHyperThreadValue<V>>) -> Result<(), InsertError>
	{
		debug_assert_eq!(allocate_values.len(), self.values_length());
		self.map_file_descriptor.insert_variably_sized(key, &allocate_values[..])
	}
	
	/// Uses a reusable vector created with `self.allocate_values()`.
	#[inline(always)]
	pub unsafe fn set_values(&self, key: &K, allocate_values: &Vec<PerHyperThreadValue<V>>) -> Result<(), ()>
	{
		debug_assert_eq!(allocate_values.len(), self.values_length());
		self.map_file_descriptor.set_variably_sized(key, &allocate_values[..])
	}
	
	/// Looks up a key.
	///
	/// ***Expensive*** as creates a vector first.
	pub fn get(&self, key: &K) -> Option<Vec<PerHyperThreadValue<V>>>
	{
		self.map_file_descriptor.get_variably_sized(key, self.values_length())
	}
	
	/// Insert or set (overwrite).
	///
	/// ***Expensive*** as creates a vector first.
	#[inline(always)]
	pub fn insert_or_set(&self, key: &K, initializer: &impl Fn(HyperThread) -> V) -> Result<(), ()>
	{
		let values = self.initialized_per_hyper_thread_values(initializer);
		self.map_file_descriptor.insert_or_set_variably_sized(key, &values[..])
	}
	
	/// Insert or fail.
	///
	/// ***Expensive*** as creates a vector first.
	#[inline(always)]
	pub fn insert(&self, key: &K, initializer: &impl Fn(HyperThread) -> V) -> Result<(), InsertError>
	{
		let values = self.initialized_per_hyper_thread_values(initializer);
		self.map_file_descriptor.insert_variably_sized(key, &values[..])
	}
	
	/// Set (overwrite).
	///
	/// ***Expensive*** as creates a vector first.
	#[inline(always)]
	pub fn set(&self, key: &K, initializer: &impl Fn(HyperThread) -> V) -> Result<(), ()>
	{
		let values = self.initialized_per_hyper_thread_values(initializer);
		self.map_file_descriptor.set_variably_sized(key, &values[..])
	}
	
	/// Delete.
	///
	/// Returns `Ok(true)` if `key` was present.
	#[inline(always)]
	pub fn delete(&self, key: &K) -> Result<bool, SystemCallErrorNumber>
	{
		self.map_file_descriptor.delete(key)
	}
	
	/// Get, batched.
	///
	/// Use `None` for `batch_position` when starting a new batch.
	/// Each value in `keys` must be valid.
	///
	/// ***Expensive*** as creates a vector first.
	#[inline(always)]
	pub fn get_batch(&self, batch_position: Option<&OpaqueBatchPosition<K>>, keys: &[K]) -> Result<(PerHyperThreadValues<V>, OpaqueBatchPosition<K>, bool), SystemCallErrorNumber>
	{
		self.guard_keys(keys);
		
		let mut batch_values = PerHyperThreadValues::new(keys.len(), self.number_of_possible_hyper_threads);
		
		let (count, out_batch, more) = self.map_file_descriptor.get_batch_variably_sized(batch_position, keys, batch_values.mutable_data())?;
		batch_values.set_number_of_validly_initialized_values(count);
		Ok((batch_values, out_batch, more))
	}
	
	/// Get and delete, batched.
	///
	/// ***Expensive*** as creates a vector first.
	#[inline(always)]
	pub fn get_and_delete_batch(&self, batch_position: Option<&OpaqueBatchPosition<K>>, keys: &[K]) -> Result<(PerHyperThreadValues<V>, OpaqueBatchPosition<K>, bool), SystemCallErrorNumber>
	{
		self.guard_keys(keys);
		
		let mut batch_values = PerHyperThreadValues::new(keys.len(), self.number_of_possible_hyper_threads);
		
		let (count, out_batch, more) = self.map_file_descriptor.get_and_delete_batch_variably_sized(batch_position, keys, batch_values.mutable_data())?;
		batch_values.set_number_of_validly_initialized_values(count);
		Ok((batch_values, out_batch, more))
	}
	
	/// Set, batched.
	///
	/// `keys` and `values` must be the same length.
	/// Each value in `keys` must be valid.
	#[inline(always)]
	pub fn set_batch(&self, keys: &[K], initializer: &impl Fn(usize, &K, &mut [PerHyperThreadValue<V>])) -> Result<usize, SystemCallErrorNumber>
	{
		self.guard_keys(keys);
		
		let mut batch_values = PerHyperThreadValues::new(keys.len(), self.number_of_possible_hyper_threads);
		batch_values.set_number_of_validly_initialized_values(keys.len());
		
		let mut row_index = 0;
		for key in keys
		{
			initializer(row_index, key, batch_values.values_at_row_mut(row_index));
			row_index += 1;
		}
		
		self.map_file_descriptor.set_batch_variably_sized(keys, batch_values.data())
	}
	
	/// Delete, batched.
	#[inline(always)]
	pub fn delete_batch(&self, keys: &[K]) -> Result<(usize, bool), SystemCallErrorNumber>
	{
		self.map_file_descriptor.delete_batch(keys)
	}
	
	#[inline(always)]
	fn initialized_per_hyper_thread_values(&self, initializer: &impl Fn(HyperThread) -> V) -> Vec<PerHyperThreadValue<V>>
	{
		self.number_of_possible_hyper_threads.initialized_per_hyper_thread_values(initializer)
	}
	
	#[inline(always)]
	fn values_length(&self) -> usize
	{
		self.number_of_possible_hyper_threads.length()
	}
	
	#[inline(always)]
	fn guard_keys(&self, keys: &[K])
	{
		assert_ne!(keys.len(), 0);
		debug_assert!(keys.len() <= self.maximum_entries.to_u32() as usize)
	}
	
	#[inline(always)]
	fn create(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, map_type: MapType, maximum_entries: MaximumEntries, number_of_possible_hyper_threads: NumberOfPossibleHyperThreads) -> Result<Self, MapCreationError>
	{
		MapFileDescriptor::create(map_file_descriptors, map_type, map_name, parsed_bpf_type_format_map_data).map(|map_file_descriptor| Self::new(map_file_descriptor, maximum_entries, number_of_possible_hyper_threads))
	}
	
	#[inline(always)]
	const fn new(map_file_descriptor: Rc<MapFileDescriptor>, maximum_entries: MaximumEntries, number_of_possible_hyper_threads: NumberOfPossibleHyperThreads) -> Self
	{
		Self
		{
			map_file_descriptor,
			maximum_entries,
			number_of_possible_hyper_threads,
			marker: PhantomData,
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
