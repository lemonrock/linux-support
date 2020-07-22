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
pub struct MapsHashMap<K: Copy, MC: MapConstructor>
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
	inner_map_maximum_entries: MaximumEntries,
	inner_map_access_permissions: MC::AccessPermissions,
	inner_map_invariant_arguments: MC::InvariantArguments,
	marker: PhantomData<K>,
}

impl<K: Copy, MC: MapConstructor> MapsHashMap<K, MC>
{
	/// Capacity.
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
	pub fn keys(&self) -> Result<KeyIterator<K>, Errno>
	{
		KeyIterator::new(&self.map_file_descriptor)
	}
	
	/// Insert or set (overwrite).
	#[inline(always)]
	pub fn insert_or_set(&self, key: &K, map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, variable_arguments: MC::VariableArguments) -> Result<MC::Map, ()>
	{
		let inner_map = self.inner_map(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, variable_arguments)?;
		self.map_file_descriptor.insert_or_set(key, &inner_map.map_file_descriptor().as_raw_fd(), LockFlags::DoNotLock)?;
		Ok(inner_map)
	}
	
	/// Insert or fail.
	#[inline(always)]
	pub fn insert(&self, key: &K, map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, variable_arguments: MC::VariableArguments) -> Result<MC::Map, ()>
	{
		let inner_map = self.inner_map(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, variable_arguments)?;
		self.map_file_descriptor.insert(key, &inner_map.map_file_descriptor().as_raw_fd(), LockFlags::DoNotLock).map_err(|_| ())?;
		Ok(inner_map)
	}
	
	/// Set the map at key to a newly-created map.
	#[inline(always)]
	pub fn set(&self, key: &K, map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, variable_arguments: MC::VariableArguments) -> Result<MC::Map, ()>
	{
		let inner_map = self.inner_map(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, variable_arguments)?;
		self.map_file_descriptor.set(key, &inner_map.map_file_descriptor().as_raw_fd(), LockFlags::DoNotLock)?;
		Ok(inner_map)
	}
	
	/// Removes a map.
	#[inline(always)]
	pub fn delete(&self, key: &K) -> Result<bool, Errno>
	{
		self.map_file_descriptor.delete(key)
	}
	
	/// Creates a new map of maps, and populates key `key` with a map.
	///
	/// Returns the new maps of maps and the map value of key `key`.
	///
	/// This unfortunate design is because the Linux kernel object is effectively using 'prototype-orientated programming' when creating a map-of-maps.
	pub fn new(maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, numa_node: Option<NumaNode>, preallocation: Preallocation, inner_map_maximum_entries: MaximumEntries, inner_map_access_permissions: MC::AccessPermissions, inner_map_invariant_arguments: MC::InvariantArguments, map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, key: &K, map_at_key_variable_arguments: MC::VariableArguments) -> Result<(Self, MC::Map), ()>
	{
		let inner_map = Self::inner_map_construct(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, map_at_key_variable_arguments, inner_map_maximum_entries, inner_map_access_permissions, inner_map_invariant_arguments)?;
		let template_map_file_descriptor = inner_map.map_file_descriptor();
		
		let this = Self
		{
			map_file_descriptor: MapFileDescriptor::create(map_file_descriptors, MapType::HashOfMaps(Self::key_size(), maximum_entries, access_permissions, template_map_file_descriptor, numa_node, preallocation), map_name, parsed_bpf_type_format_map_data).map_err(|_| ())?,
			maximum_entries,
			inner_map_maximum_entries,
			inner_map_access_permissions,
			inner_map_invariant_arguments,
			marker: PhantomData,
		};
		
		this.map_file_descriptor.set(key, &template_map_file_descriptor.as_raw_fd(), LockFlags::DoNotLock)?;
		Ok((this, inner_map))
	}
	
	#[inline(always)]
	fn inner_map(&self, map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, variable_arguments: MC::VariableArguments) -> Result<MC::Map, ()>
	{
		Self::inner_map_construct(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, variable_arguments, self.inner_map_maximum_entries, self.inner_map_access_permissions, self.inner_map_invariant_arguments)
	}
	
	#[inline(always)]
	fn inner_map_construct(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, variable_arguments: MC::VariableArguments, inner_map_maximum_entries: MaximumEntries, inner_map_access_permissions: MC::AccessPermissions, inner_map_invariant_arguments: MC::InvariantArguments) -> Result<MC::Map, ()>
	{
		MC::construct(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, inner_map_maximum_entries, inner_map_access_permissions, inner_map_invariant_arguments, variable_arguments).map_err(|_| ())
	}
	
	#[inline(always)]
	fn key_size() -> KeySize
	{
		KeySize::try_from_key_size::<K>().unwrap()
	}
}
