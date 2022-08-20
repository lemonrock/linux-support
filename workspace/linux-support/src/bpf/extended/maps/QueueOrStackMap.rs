// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// When an array is created, all its elements are zeroed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueueOrStackMap<V: Copy>
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
	marker: PhantomData<V>,
}

impl<V: Copy> CanBeInnerMap for QueueOrStackMap<V>
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.map_file_descriptor
	}
}

impl<V: Copy> QueueOrStackMap<V>
{
	/// New queue.
	#[inline(always)]
	pub fn new_queue(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::Queue(Self::value_size(), maximum_entries, access_permissions, numa_node), maximum_entries)
	}
	
	/// New stack.
	#[inline(always)]
	pub fn new_stack(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::Stack(Self::value_size(), maximum_entries, access_permissions, numa_node), maximum_entries)
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
	
	/// Push.
	#[inline(always)]
	pub fn push(&self, value: &V) -> Result<(), ()>
	{
		static IgnoredKey: u32 = 0;
		self.map_file_descriptor.insert_or_set(&IgnoredKey, value, LockFlags::DoNotLock)
	}
	
	/// Push, overwriting tail if full.
	#[inline(always)]
	pub fn push_overwriting_tail_if_full(&self, value: &V)
	{
		static IgnoredKey: u32 = 0;
		self.map_file_descriptor.set(&IgnoredKey, value, LockFlags::DoNotLock).expect("Should always succeed")
	}
	
	/// Pop.
	#[inline(always)]
	pub fn pop(&self) -> Result<Option<V>, SystemCallErrorNumber>
	{
		static IgnoredKey: u32 = 0;
		self.map_file_descriptor.lookup_and_delete(&IgnoredKey)
	}
	
	/// Peek.
	#[inline(always)]
	pub fn peek(&self) -> Option<V>
	{
		static IgnoredKey: u32 = 0;
		self.map_file_descriptor.get(&IgnoredKey)
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
	fn value_size() -> ValueSizeU32
	{
		ValueSizeU32::try_from_value_size::<V>().unwrap()
	}
}
