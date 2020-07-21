// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// When an array is created, all its elements are zeroed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SpinLockableArrayMap<V: 'static + Copy + HasReflectionInformation>(ArrayMap<SpinLockableValue<V>>);

impl<V: 'static + Copy + HasReflectionInformation> SpinLockableArrayMap<V>
{
	/// New system-wide.
	#[inline(always)]
	pub fn new_system_wide(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		let array_map = ArrayMap::new_system_wide_internal(map_file_descriptors, map_name, parsed_btf_map_data, maximum_entries, access_permissions, numa_node, MemoryMap::DoNotMemoryMap)?;
		Ok(Self(array_map))
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
	
	/// Indices.
	#[inline(always)]
	pub fn indices(&self) -> RangeInclusive<u32>
	{
		self.0.indices()
	}
	
	/// Get, batched.
	#[inline(always)]
	pub fn get_batch(&self, in_batch: Option<&OpaqueBatchPosition<u32>>, indices: &[u32]) -> Result<(Vec<SpinLockableValue<V>>, OpaqueBatchPosition<u32>, bool), Errno>
	{
		self.0.get_batch(in_batch, indices)
	}
	
	/// Set, batched.
	#[inline(always)]
	pub fn set_batch(&self, indices: &[u32], values: &[SpinLockableValue<V>]) -> Result<usize, Errno>
	{
		self.0.set_batch_locked(indices, values)
	}
	
	/// Looks up an index; should always succeed.
	#[inline(always)]
	pub fn get(&self, index: u32) -> V
	{
		let spin_lockable_value = self.0.get(index);
		spin_lockable_value.value
	}
	
	/// Update existing.
	#[allow(deprecated)]
	pub fn set(&self, index: u32, value: V)
	{
		self.0.set_locked(index, SpinLockableValue { spin_lock: unsafe { uninitialized() }, value })
	}
}
