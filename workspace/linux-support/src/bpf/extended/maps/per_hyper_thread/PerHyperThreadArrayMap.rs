// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// When an array is created, all its elements are zeroed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerHyperThreadArrayMap<V: Copy>
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
	number_of_possible_hyper_threads: NumberOfPossibleHyperThreads,
	marker: PhantomData<V>,
}

impl<V: Copy> CanBeInnerMap for PerHyperThreadArrayMap<V>
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.map_file_descriptor
	}
}

impl<V: Copy> PerHyperThreadArrayMap<V>
{
	/// New per-HyperThread.
	#[inline(always)]
	pub fn new_per_hyper_thread(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, number_of_possible_hyper_threads: NumberOfPossibleHyperThreads) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::ArrayPerHyperThread(Self::value_size(), maximum_entries, access_permissions), maximum_entries, number_of_possible_hyper_threads)
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
	
	/// Indices.
	#[inline(always)]
	pub fn indices(&self) -> RangeInclusive<u32>
	{
		0 ..= self.maximum_entries.0.get()
	}
	
	/// Allocates a reusable vector.
	#[inline(always)]
	pub unsafe fn allocate_values(&self) -> Vec<PerHyperThreadValue<V>>
	{
		self.number_of_possible_hyper_threads.uninitialized_per_hyper_thread_values()
	}
	
	/// Uses a reusable vector created with `self.allocate_values()`.
	#[inline(always)]
	pub unsafe fn get_values(&self, index: u32, allocate_values: &mut Vec<PerHyperThreadValue<V>>)
	{
		self.guard_index(index);
		
		debug_assert_eq!(allocate_values.len(), self.values_length());
		let found = self.map_file_descriptor.get_variably_sized_vector(&index, allocate_values);
		assert!(found, "index should always exist")
	}
	
	/// Uses a reusable vector created with `self.allocate_values()`.
	#[inline(always)]
	pub unsafe fn set_values(&self, index: u32, allocate_values: &Vec<PerHyperThreadValue<V>>)
	{
		self.guard_index(index);
		
		debug_assert_eq!(allocate_values.len(), self.values_length());
		self.map_file_descriptor.set_variably_sized(&index, &allocate_values[..]).expect("index should always exist")
	}
	
	/// Looks up an index; should always succeed.
	///
	/// ***Expensive*** as creates a vector first.
	pub fn get(&self, index: u32) -> Vec<PerHyperThreadValue<V>>
	{
		self.guard_index(index);
		
		self.map_file_descriptor.get_variably_sized(&index, self.values_length()).expect("index should always exist")
	}
	
	/// Update existing.
	///
	/// ***Expensive*** as creates a vector first.
	pub fn set(&self, index: u32, initializer: &impl Fn(HyperThread) -> V)
	{
		self.guard_index(index);
		
		let values = self.initialized_per_hyper_thread_values(initializer);
		self.map_file_descriptor.set_variably_sized(&index, &values[..]).expect("index should always exist")
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
	fn guard_index(&self, index: u32)
	{
		debug_assert!(index < self.maximum_entries.to_u32());
	}
	
	#[inline(always)]
	fn create(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, map_type: MapType, maximum_entries: MaximumEntries, number_of_possible_hyper_threads: NumberOfPossibleHyperThreads) -> Result<Self, MapCreationError>
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
}
