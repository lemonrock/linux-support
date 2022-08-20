// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An array map specialized for use with file descriptors.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileDescriptorArrayMap<FD: UsedAsValueInArrayMapDescriptor>
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
	marker: PhantomData<FD>,
}

impl<FD: UsedAsValueInArrayMapDescriptor> FileDescriptorArrayMap<FD>
{
	/// Capacity.
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
	
	/// Indices.
	#[inline(always)]
	pub fn indices(&self) -> RangeInclusive<u32>
	{
		0 ..= self.maximum_entries.0.get()
	}
	
	/// Update existing.
	#[inline(always)]
	pub fn set(&self, index: u32, file_descriptor: &FD) -> Result<(), ()>
	{
		self.map_file_descriptor.set_for_file_descriptor_array_map(&index, &file_descriptor.as_raw_fd())
	}
	
	/// Removes a file descriptor.
	pub fn delete(&self, index: u32) -> Result<bool, SystemCallErrorNumber>
	{
		self.map_file_descriptor.delete(&index)
	}
	
	#[inline(always)]
	fn create(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, map_type: impl FnOnce(MaximumEntries, KernelOnlyAccessPermissions) -> MapType<'static>) -> Result<Self, MapCreationError>
	{
		let map_file_descriptor = MapFileDescriptor::create(map_file_descriptors, map_type(maximum_entries, access_permissions), map_name, parsed_bpf_type_format_map_data)?;
		Ok
		(
			Self
			{
				map_file_descriptor,
				maximum_entries,
				marker: PhantomData,
			}
		)
	}
}

impl<FD: ProvidesIdentifierWhenUsedAsValueInArrayMapDescriptor> FileDescriptorArrayMap<FD>
{
	/// Returns an identifier.
	///
	/// It may be that this *always* returns `Some(identifier)` and the `identifier` may not be valid; the Linux API isn't documented at all and the source code in Linux is the usual C spaghetti.
	pub fn get(&self, index: u32) -> Option<FD::Identifier>
	{
		self.map_file_descriptor.get(&index).map(|raw_identifier| FD::Identifier::from(raw_identifier))
	}
}

impl CanBeInnerMap for FileDescriptorArrayMap<PerfEventFileDescriptor>
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.map_file_descriptor
	}
}

impl FileDescriptorArrayMap<PerfEventFileDescriptor>
{
	/// New perf event array.
	#[inline(always)]
	pub fn new_perf_event(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, maximum_entries, access_permissions, MapType::PerfEventArray)
	}
}

impl CanBeInnerMap for FileDescriptorArrayMap<CgroupFileDescriptor>
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.map_file_descriptor
	}
}

impl FileDescriptorArrayMap<CgroupFileDescriptor>
{
	/// New cgroup array.
	#[inline(always)]
	pub fn new_cgroup(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, maximum_entries, access_permissions, MapType::CgroupArray)
	}
}

impl FileDescriptorArrayMap<ExtendedBpfProgramFileDescriptor>
{
	/// New extended BPF program array.
	#[inline(always)]
	pub fn new_extended_bpf_program(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, maximum_entries, access_permissions, MapType::ProgramArray)
	}
}

impl FileDescriptorArrayMap<MapFileDescriptor>
{
	#[inline(always)]
	fn new_map_of_maps(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, template_map_file_descriptor: &MapFileDescriptor) -> Result<Self, MapCreationError>
	{
		let map_file_descriptor = MapFileDescriptor::create(map_file_descriptors, MapType::ArrayOfMaps(maximum_entries, access_permissions, template_map_file_descriptor), map_name, parsed_bpf_type_format_map_data)?;
		Ok
		(
			Self
			{
				map_file_descriptor,
				maximum_entries,
				marker: PhantomData,
			}
		)
	}
}
