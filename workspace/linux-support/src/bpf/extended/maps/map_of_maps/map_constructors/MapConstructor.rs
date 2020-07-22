// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
pub trait MapConstructor: Sized
{
	/// Creates a new `MapsOfMapArrayMap`.
	#[inline(always)]
	fn maps_of_map_array_map(maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, inner_map_maximum_entries: MaximumEntries, inner_map_access_permissions: Self::AccessPermissions, inner_map_invariant_arguments: Self::InvariantArguments, map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, index: u32, map_at_index_variable_arguments: Self::VariableArguments) -> Result<(MapsOfMapArrayMap<Self>, Self::Map), ()>
	{
		MapsOfMapArrayMap::new(maximum_entries, access_permissions, inner_map_maximum_entries, inner_map_access_permissions, inner_map_invariant_arguments, map_file_descriptors, map_name, parsed_bpf_type_format_map_data, index, map_at_index_variable_arguments)
	}
	
	/// Creates a new `MapsOfMapHashMap`.
	#[inline(always)]
	fn maps_of_map_hash_map<K: Copy>(maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, numa_node: Option<NumaNode>, preallocation: Preallocation, inner_map_maximum_entries: MaximumEntries, inner_map_access_permissions: Self::AccessPermissions, inner_map_invariant_arguments: Self::InvariantArguments, map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, key: &K, map_at_key_variable_arguments: Self::VariableArguments) -> Result<(MapsOfMapHashMap<K, Self>, Self::Map), ()>
	{
		MapsOfMapHashMap::new(maximum_entries, access_permissions, numa_node, preallocation, inner_map_maximum_entries, inner_map_access_permissions, inner_map_invariant_arguments, map_file_descriptors, map_name, parsed_bpf_type_format_map_data, key, map_at_key_variable_arguments)
	}
	
	#[doc(hidden)]
	const Singleton: Self;
	
	#[allow(missing_docs)]
	type Map: CanBeInnerMap;
	
	#[allow(missing_docs)]
	type AccessPermissions: Sized + Copy;
	
	#[allow(missing_docs)]
	type InvariantArguments: Sized + Copy;
	
	#[allow(missing_docs)]
	type VariableArguments;
	
	#[doc(hidden)]
	fn construct(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: Self::AccessPermissions, arguments_that_end_up_in_map_flags: Self::InvariantArguments, variable_arguments: Self::VariableArguments) -> Result<Self::Map, MapCreationError>;
}
