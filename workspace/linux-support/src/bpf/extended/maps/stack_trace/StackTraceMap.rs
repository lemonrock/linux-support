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
	pub fn freeze(&self) -> Result<(), SystemCallErrorNumber>
	{
		self.map_file_descriptor.freeze()
	}
	
	/// Iterator of keys.
	#[inline(always)]
	pub fn keys(&self) -> Result<KeyIterator<u32>, SystemCallErrorNumber>
	{
		KeyIterator::new(&self.map_file_descriptor)
	}
	
	/// Allocates a reusable vector.
	#[inline(always)]
	pub fn allocate_values(&self) -> Vec<SF>
	{
		let length = self.allocate_values_length();
		let mut allocate_values = Vec::with_capacity(length);
		unsafe
		{
			write_bytes(allocate_values.as_mut_ptr(), 0x00, length);
			allocate_values.set_len(length)
		}
		allocate_values
	}
	
	/// Uses a reusable vector created with `self.allocate_values()`.
	///
	/// Returns `true` if found.
	pub unsafe fn get_values(&self, key: u32, allocate_values: &mut Vec<SF>) -> bool
	{
		self.guard_key(key);
		
		debug_assert_eq!(allocate_values.len(), self.allocate_values_length());
		self.map_file_descriptor.get_variably_sized_vector(&key, allocate_values)
	}
	
	#[inline(always)]
	fn allocate_values_length(&self) -> usize
	{
		self.stack_depth.to_count()
	}
	
	/// Looks up a key.
	///
	/// ***Expensive*** as creates a vector first.
	pub fn get(&self, key: u32) -> Option<Vec<SF>>
	{
		self.guard_key(key);
		
		self.map_file_descriptor.get_variably_sized(&key, self.allocate_values_length())
	}
	
	/// Delete.
	///
	/// Returns `Ok(true)` if `index` was present.
	#[inline(always)]
	pub fn delete(&self, key: u32) -> Result<bool, SystemCallErrorNumber>
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
	fn create(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, map_type: MapType, maximum_entries: MaximumEntries, stack_depth: StackDepth) -> Result<Self, MapCreationError>
	{
		MapFileDescriptor::create(map_file_descriptors, map_type, map_name, parsed_bpf_type_format_map_data).map(|map_file_descriptor| Self::new(map_file_descriptor, maximum_entries, stack_depth))
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
	pub fn new_instruction_pointer(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, numa_node: Option<NumaNode>, stack_depth: StackDepth) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::StackTraceInstructionPointerAddress(stack_depth, maximum_entries, access_permissions, numa_node), maximum_entries, stack_depth)
	}
}

impl StackTraceMap<bpf_stack_build_id>
{
	/// New for a `bpf_stack_build_id`.
	#[inline(always)]
	pub fn new_bpf_stack_build_id(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, numa_node: Option<NumaNode>, stack_depth: StackDepth) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::StackTraceBuildIdentifier(stack_depth, maximum_entries, access_permissions, numa_node), maximum_entries, stack_depth)
	}
}
