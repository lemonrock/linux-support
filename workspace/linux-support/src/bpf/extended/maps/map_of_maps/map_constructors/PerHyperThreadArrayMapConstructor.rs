// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerHyperThreadArrayMapConstructor<V: Copy>(PhantomData<V>);

impl<V: Copy> MapConstructor for PerHyperThreadArrayMapConstructor<V>
{
	const Singleton: Self = Self(PhantomData);
	
	type Map = PerHyperThreadArrayMap<V>;
	
	type AccessPermissions = AccessPermissions;
	
	type InvariantArguments = NumberOfPossibleHyperThreads;
	
	type VariableArguments = ();
	
	#[inline(always)]
	fn construct(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: Self::AccessPermissions, invariant_arguments: Self::InvariantArguments, _variable_arguments: Self::VariableArguments) -> Result<Self::Map, MapCreationError>
	{
		let number_of_possible_hyper_threads = invariant_arguments;
		PerHyperThreadArrayMap::new_per_hyper_thread(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, maximum_entries, access_permissions, number_of_possible_hyper_threads)
	}
}
