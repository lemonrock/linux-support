// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Stack trace map.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StackTraceMap<SF: StackFrame>
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
	stack_depth: StackDepth,
	marker: PhantomData<SF>,
}

impl<SF: StackFrame> CanBeInnerMap for StackTraceMap<SF>
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.map_file_descriptor
	}
}

impl<SF: StackFrame> StackTraceMap<SF>
{
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
	pub fn keys(&self) -> Result<KeyIterator<u32>, Errno>
	{
		KeyIterator::new(&self.map_file_descriptor)
	}
	
	/// Looks up a key.
	pub fn get(&self, key: u32) -> Option<Vec<SF>>
	{
		self.guard_key(key);
		
		let length = self.stack_depth.to_count();
		self.map_file_descriptor.get_variably_sized(&key, length)
	}
	
	/// Delete.
	///
	/// Returns `Ok(true)` if `index` was present.
	#[inline(always)]
	pub fn delete(&self, key: u32) -> Result<bool, Errno>
	{
		self.guard_key(key);
		
		self.map_file_descriptor.delete(&key)
	}
	
	#[inline(always)]
	fn guard_key(&self, key: u32)
	{
		debug_assert!(key < self.maximum_entries.to_u32());
	}
	
	#[inline(always)]
	fn create(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, map_type: MapType, maximum_entries: MaximumEntries, stack_depth: StackDepth) -> Result<Self, MapCreationError>
	{
		MapFileDescriptor::create(map_file_descriptors, map_type, map_name, parsed_btf_map_data).map(|map_file_descriptor| Self::new(map_file_descriptor, maximum_entries, stack_depth))
	}
	
	#[inline(always)]
	const fn new(map_file_descriptor: Rc<MapFileDescriptor>, maximum_entries: MaximumEntries, stack_depth: StackDepth) -> Self
	{
		Self
		{
			map_file_descriptor,
			maximum_entries,
			stack_depth,
			marker: PhantomData
		}
	}
}

impl StackTraceMap<InstructionPointer>
{
	/// New for an `InstructionPointer`.
	///
	/// For user space, this models stack frames with addresses.
	/// For kernel space, this models stack frames with kernel symbols; one can look up the 64-bit instruction pointer in `/proc/kallsyms` (if present).
	#[inline(always)]
	pub fn new_instruction_pointer(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, numa_node: Option<NumaNode>, stack_depth: StackDepth) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, MapType::StackTraceInstructionPointerAddress(stack_depth, maximum_entries, access_permissions, numa_node), maximum_entries, stack_depth)
	}
}

impl StackTraceMap<bpf_stack_build_id>
{
	/// New for a `bpf_stack_build_id`.
	#[inline(always)]
	pub fn new_bpf_stack_build_id(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, numa_node: Option<NumaNode>, stack_depth: StackDepth) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, MapType::StackTraceBuildIdentifier(stack_depth, maximum_entries, access_permissions, numa_node), maximum_entries, stack_depth)
	}
}
