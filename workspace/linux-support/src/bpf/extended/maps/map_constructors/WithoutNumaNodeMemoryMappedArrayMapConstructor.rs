// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WithoutNumaNodeMemoryMappedArrayMapConstructor<V: Copy>(PhantomData<V>);

impl<V: Copy> MapConstructor for WithoutNumaNodeMemoryMappedArrayMapConstructor<V>
{
	type Map = MemoryMappedArrayMap<V>;
	
	type AccessPermissions = AccessPermissions;
	
	type InvariantArguments = ();
	
	type VariableArguments = Rc<DefaultPageSizeAndHugePageSizes>;
	
	#[inline(always)]
	fn construct(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: Self::AccessPermissions, _arguments_that_end_up_in_map_flags: Self::InvariantArguments, variable_arguments: Self::VariableArguments) -> Result<Self::Map, MapCreationError>
	{
		let defaults = variable_arguments;
		MemoryMappedArrayMap::new(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, maximum_entries, access_permissions, None, &defaults)
	}
}
