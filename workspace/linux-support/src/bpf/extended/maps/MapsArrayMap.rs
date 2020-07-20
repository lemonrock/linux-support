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
pub struct MapsArrayMap<MC: MapConstructor>
{
	underlying: FileDescriptorArrayMap<MapFileDescriptor>,
	inner_map_maximum_entries: MaximumEntries,
	inner_map_access_permissions: MC::AccessPermissions,
	inner_map_invariant_arguments: MC::InvariantArguments,
}

impl<MC: MapConstructor> MapsArrayMap<MC>
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
	
	/// Batch creation.
	///
	/// Although it avoids a number of syscalls, the overall algorithm requires several mallocs and other logic, and so may not be more efficient overall than calling `set()` many times.
	#[inline(always)]
	pub fn set_batch(&self, indices: &[u32], map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name_parsed_btf_map_data_and_variable_arguments: &[(&MapName, Option<&ParsedBtfMapData>, MC::VariableArguments)]) -> Result<Vec<MC::Map>, ()>
	where MC::VariableArguments: Clone
	{
		let length = map_name_parsed_btf_map_data_and_variable_arguments.len();
		let mut inner_map_raw_fds = Vec::with_capacity(length);
		let mut inner_maps = Vec::with_capacity(length);
		for &(map_name, parsed_btf_map_data, ref variable_arguments) in map_name_parsed_btf_map_data_and_variable_arguments
		{
			let variable_arguments = variable_arguments.clone();
			let inner_map = MC::construct(map_file_descriptors, map_name, parsed_btf_map_data, self.inner_map_maximum_entries, self.inner_map_access_permissions, self.inner_map_invariant_arguments, variable_arguments).map_err(|_| ())?;
			inner_map_raw_fds.push(inner_map.map_file_descriptor().as_raw_fd());
			inner_maps.push(inner_map)
		}
		self.underlying.map_file_descriptor.set_batch(indices, &inner_map_raw_fds[..], LockFlags::DoNotLock).map_err(|_| ())?;
		Ok(inner_maps)
	}
	
	/// Set the map at index to a newly-created map.
	#[inline(always)]
	pub fn set(&self, index: u32, map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, variable_arguments: MC::VariableArguments) -> Result<MC::Map, ()>
	{
		let inner_map = MC::construct(map_file_descriptors, map_name, parsed_btf_map_data, self.inner_map_maximum_entries, self.inner_map_access_permissions, self.inner_map_invariant_arguments, variable_arguments).map_err(|_| ())?;
		self.underlying.set(index, inner_map.map_file_descriptor())?;
		Ok(inner_map)
	}
	
	/// Removes a file descriptor.
	#[inline(always)]
	pub fn delete(&self, index: u32) -> Result<bool, Errno>
	{
		self.underlying.delete(index)
	}
	
	/// Creates a new map of maps, and populates index `0` with a map.
	///
	/// Returns the new maps of maps and the map value of index `0`.
	///
	/// This unfortunate design is because the Linux kernel object is effectively using 'prototype-orientated programming' when creating a map-of-maps.
	pub fn new(maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, inner_map_maximum_entries: MaximumEntries, inner_map_access_permissions: MC::AccessPermissions, inner_map_invariant_arguments: MC::InvariantArguments, map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, variable_arguments: MC::VariableArguments) -> Result<(Self, MC::Map), ()>
	{
		let inner_map = MC::construct(map_file_descriptors, map_name, parsed_btf_map_data, inner_map_maximum_entries, inner_map_access_permissions, inner_map_invariant_arguments, variable_arguments).map_err(|_| ())?;
		let template_map_file_descriptor = inner_map.map_file_descriptor();
		
		let this = Self
		{
			underlying: FileDescriptorArrayMap::new_map_of_maps(map_file_descriptors, map_name, parsed_btf_map_data, maximum_entries, access_permissions, template_map_file_descriptor).map_err(|_| ())?,
			inner_map_maximum_entries,
			inner_map_access_permissions,
			inner_map_invariant_arguments,
		};
		
		const Index0: u32 = 0;
		this.underlying.set(Index0, template_map_file_descriptor)?;
		Ok((this, inner_map))
	}
}
