// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Inner map can not be:-
///
/// * `BPF_MAP_TYPE_PROG_ARRAY`.
/// * `BPF_MAP_TYPE_CGROUP_STORAGE`.
/// * `BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE`.
/// * `BPF_MAP_TYPE_STRUCT_OPS`.
///
/// And presumably `BPF_MAP_TYPE_UNSPEC`.
///
/// Inner map can not itself be a map-of-maps.
/// Thus also inner map can also not be:-
///
/// * `BPF_MAP_TYPE_ARRAY_OF_MAPS`.
/// * `BPF_MAP_TYPE_HASH_OF_MAPS`.
///
/// Inner map can not be of type `SpinLockableArrayMap`.
///
/// Inner map elements must all have the same `maximum_entries`, `value_size` and `BPF_MAP_CREATE_flags` for all elements.
#[derive(Debug)]
pub struct MapsOfMapArrayMap<MC: MapConstructor>
{
	underlying: FileDescriptorArrayMap<MapFileDescriptor>,
	inner_map_maximum_entries: MaximumEntries,
	inner_map_access_permissions: MC::AccessPermissions,
	inner_map_invariant_arguments: MC::InvariantArguments,
}

impl<MC: MapConstructor> MapsOfMapArrayMap<MC>
{
	/// Capacity.
	#[inline(always)]
	pub fn capacity(&self) -> NonZeroU32
	{
		self.underlying.capacity()
	}
	
	/// Freeze.
	#[inline(always)]
	pub fn freeze(&self) -> Result<(), Errno>
	{
		self.underlying.freeze()
	}
	
	/// Indices.
	#[inline(always)]
	pub fn indices(&self) -> RangeInclusive<u32>
	{
		self.underlying.indices()
	}
	
	/// Set the map at index to a newly-created map.
	#[inline(always)]
	pub fn set(&self, index: u32, map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, variable_arguments: MC::VariableArguments) -> Result<MC::Map, ()>
	{
		let inner_map = MC::construct(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, self.inner_map_maximum_entries, self.inner_map_access_permissions, self.inner_map_invariant_arguments, variable_arguments).map_err(|_| ())?;
		self.underlying.set(index, inner_map.map_file_descriptor())?;
		Ok(inner_map)
	}
	
	/// Removes a map.
	#[inline(always)]
	pub fn delete(&self, index: u32) -> Result<bool, Errno>
	{
		self.underlying.delete(index)
	}
	
	/// Creates a new map of maps, and populates index `index` with a map.
	///
	/// Returns the new maps of maps and the map value of index `index`.
	///
	/// This unfortunate design is because the Linux kernel object is effectively using 'prototype-orientated programming' when creating a map-of-maps.
	pub fn new(maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, inner_map_maximum_entries: MaximumEntries, inner_map_access_permissions: MC::AccessPermissions, inner_map_invariant_arguments: MC::InvariantArguments, map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, index: u32, map_at_index_variable_arguments: MC::VariableArguments) -> Result<(Self, MC::Map), ()>
	{
		debug_assert!(index < maximum_entries.to_u32());
		
		let inner_map = MC::construct(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, inner_map_maximum_entries, inner_map_access_permissions, inner_map_invariant_arguments, map_at_index_variable_arguments).map_err(|_| ())?;
		let template_map_file_descriptor = inner_map.map_file_descriptor();
		
		let this = Self
		{
			underlying: FileDescriptorArrayMap::new_map_of_maps(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, maximum_entries, access_permissions, template_map_file_descriptor).map_err(|_| ())?,
			inner_map_maximum_entries,
			inner_map_access_permissions,
			inner_map_invariant_arguments,
		};
		
		this.underlying.set(index, template_map_file_descriptor)?;
		Ok((this, inner_map))
	}
}
