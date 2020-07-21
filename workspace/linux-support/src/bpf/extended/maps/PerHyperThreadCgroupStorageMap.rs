// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Per-HyperThread cgroup storage.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerHyperThreadCgroupStorageMap<V: Copy>
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
	number_of_possible_hyper_threads: NumberOfPossibleHyperThreads,
	marker: PhantomData<V>,
}

impl<V: Copy> PerHyperThreadCgroupStorageMap<V>
{
	/// New per-HyperThread.
	///
	/// Weirdly, unlike all other per-HyperThread maps, this one supports allocating on a NumaNode.
	#[inline(always)]
	pub fn new_per_hyper_thread(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, number_of_possible_hyper_threads: NumberOfPossibleHyperThreads, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, MapType::CgroupStoragePerHyperThread(Self::value_size(), access_permissions, numa_node), maximum_entries, number_of_possible_hyper_threads)
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
	
	/// Allocates a reusable vector.
	#[inline(always)]
	pub unsafe fn allocate_values(&self) -> Vec<PerHyperThreadValue<V>>
	{
		self.number_of_possible_hyper_threads.uninitialized_per_hyper_thread_values()
	}
	
	/// Uses a reusable vector created with `self.allocate_values()`.
	#[inline(always)]
	pub unsafe fn get_values(&self, key: &bpf_cgroup_storage_key, allocate_values: &mut Vec<PerHyperThreadValue<V>>)
	{
		debug_assert_eq!(allocate_values.len(), self.values_length());
		let found = self.map_file_descriptor.get_variably_sized_vector(key, allocate_values);
		assert!(found, "index should always exist")
	}
	
	/// Uses a reusable vector created with `self.allocate_values()`.
	#[inline(always)]
	pub unsafe fn insert_or_set_values(&self, key: &bpf_cgroup_storage_key, allocate_values: &Vec<PerHyperThreadValue<V>>)
	{
		debug_assert_eq!(allocate_values.len(), self.values_length());
		self.map_file_descriptor.set_variably_sized(key, &allocate_values[..]).expect("index should always exist")
	}
	
	/// Get.
	///
	/// ***Expensive*** as creates a vector first.
	pub fn get(&self, key: &bpf_cgroup_storage_key) -> Option<Vec<PerHyperThreadValue<V>>>
	{
		self.map_file_descriptor.get_variably_sized(key, self.values_length())
	}
	
	/// Insert or set.
	///
	/// ***Expensive*** as creates a vector first.
	pub fn insert_or_set(&self, key: &bpf_cgroup_storage_key, initializer: &impl Fn(HyperThread) -> V) -> Result<(), ()>
	{
		let values = self.initialized_per_hyper_thread_values(initializer);
		self.map_file_descriptor.set_variably_sized(key, &values[..])
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
	
	/// Delete.
	#[inline(always)]
	pub fn delete(&self, key: &bpf_cgroup_storage_key) -> Result<bool, Errno>
	{
		self.map_file_descriptor.delete(key)
	}
	
	#[inline(always)]
	fn create(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, map_type: MapType, maximum_entries: MaximumEntries, number_of_possible_hyper_threads: NumberOfPossibleHyperThreads) -> Result<Self, MapCreationError>
	{
		MapFileDescriptor::create(map_file_descriptors, map_type, map_name, parsed_btf_map_data).map(|map_file_descriptor| Self::new(map_file_descriptor, maximum_entries, number_of_possible_hyper_threads))
	}
	
	#[inline(always)]
	const fn new(map_file_descriptor: Rc<MapFileDescriptor>, maximum_entries: MaximumEntries, number_of_possible_hyper_threads: NumberOfPossibleHyperThreads) -> Self
	{
		Self
		{
			map_file_descriptor,
			maximum_entries,
			number_of_possible_hyper_threads,
			marker: PhantomData
		}
	}
	
	#[inline(always)]
	fn value_size() -> ValueSizeU16
	{
		ValueSizeU16::try_from_value_size::<V>().unwrap()
	}
}
