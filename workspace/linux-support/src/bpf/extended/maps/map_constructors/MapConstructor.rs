// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
pub trait MapConstructor
{
	#[allow(missing_docs)]
	type Map: CanBeInnerMap;
	
	#[allow(missing_docs)]
	type AccessPermissions: Sized + Copy;
	
	#[allow(missing_docs)]
	type InvariantArguments: Sized + Copy;
	
	#[allow(missing_docs)]
	type VariableArguments;
	
	#[allow(missing_docs)]
	fn construct(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: Self::AccessPermissions, arguments_that_end_up_in_map_flags: Self::InvariantArguments, variable_arguments: Self::VariableArguments) -> Result<Self::Map, MapCreationError>;
}
