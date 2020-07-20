// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// When an array is created, all its elements are zeroed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayMap<V: Sized>
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
	marker: PhantomData<V>,
}

impl<V: Sized> CanBeInnerMap for ArrayMap<V>
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.map_file_descriptor
	}
}

impl<V: Sized> ArrayMap<V>
{
	/// New per-device.
	#[inline(always)]
	pub fn new_per_device(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, device: NetworkInterfaceIndex) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, MapType::ArrayPerDevice(Self::value_size(), maximum_entries, access_permissions, device), maximum_entries)
	}
	
	/// New per-CPU.
	#[inline(always)]
	pub fn new_per_cpu(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, MapType::ArrayPerCpu(Self::value_size(), maximum_entries, access_permissions), maximum_entries)
	}
	
	/// New system-wide.
	#[inline(always)]
	pub fn new_system_wide(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::new_system_wide_internal(map_file_descriptors, map_name, parsed_btf_map_data, maximum_entries, access_permissions, numa_node, MemoryMap::DoNotMemoryMap)
	}
	
	#[inline(always)]
	fn new_system_wide_internal(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>, memory_map: MemoryMap) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, MapType::ArraySystemWide(Self::value_size(), maximum_entries, access_permissions, numa_node, memory_map), maximum_entries)
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

	/// TODO: Batch operations are only supported for system wide array maps.
	///
	/// Get, batched.
	///
	/// Use `None` for `batch_position` when starting a new batch.
	/// Each value in `indices` must be valid.
	#[inline(always)]
	pub fn get_batch(&self, batch_position: Option<&OpaqueBatchPosition<u32>>, indices: &[u32]) -> Result<(Vec<V>, OpaqueBatchPosition<u32>, bool), Errno>
	{
		self.guard_indices(indices);
	
		self.map_file_descriptor.get_batch(batch_position, indices)
	}
	
	/// TODO: Batch operations are only supported for system wide array maps.
	///
	/// Set, batched.
	///
	/// `indices` and `values` must be the same length.
	/// Each value in `indices` must be valid.
	#[inline(always)]
	pub fn set_batch(&self, indices: &[u32], values: &[V]) -> Result<usize, Errno>
	{
		self.guard_indices_and_values(indices, values);
	
		self.map_file_descriptor.set_batch(indices, values, LockFlags::DoNotLock)
	}
	
	/// `indices` and `values` must be the same length.
	/// Each value in `indices` must be valid.
	#[inline(always)]
	fn set_batch_locked(&self, indices: &[u32], values: &[V]) -> Result<usize, Errno>
	{
		self.guard_indices_and_values(indices, values);
		
		self.map_file_descriptor.set_batch(indices, values, LockFlags::Lock)
	}
	
	/// Looks up an index; should always succeed.
	pub fn get(&self, index: u32) -> V
	{
		self.guard_index(index);
		
		self.map_file_descriptor.get(&index).expect("index should always exist")
	}
	
	/// Update existing.
	pub fn set(&self, index: u32, value: &V)
	{
		self.guard_index(index);
		
		self.map_file_descriptor.set(&index, value, LockFlags::DoNotLock).expect("index should always exist")
	}
	
	#[inline(always)]
	fn set_locked(&self, index: u32, mut value: V)
	{
		self.guard_index(index);
		
		self.map_file_descriptor.set(&index, &mut value, LockFlags::Lock).expect("index should always exist")
	}
	
	#[inline(always)]
	fn guard_indices_and_values(&self, indices: &[u32], values: &[V])
	{
		self.guard_indices(indices);
		debug_assert_eq!(indices.len(), values.len());
	}
	
	#[inline(always)]
	fn guard_indices(&self, indices: &[u32])
	{
		debug_assert!(indices.len() <= self.maximum_entries.to_u32() as usize);
		
		if cfg!(debug_assertions)
		{
			for index in indices
			{
				self.guard_index(*index);
			}
		}
	}
	
	#[inline(always)]
	fn guard_index(&self, index: u32)
	{
		debug_assert!(index < self.maximum_entries.to_u32());
	}
	
	#[inline(always)]
	fn create(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, map_type: MapType, maximum_entries: MaximumEntries) -> Result<Self, MapCreationError>
	{
		MapFileDescriptor::create(map_file_descriptors, map_type, map_name, parsed_btf_map_data).map(|map_file_descriptor| Self::new(map_file_descriptor, maximum_entries))
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
}
