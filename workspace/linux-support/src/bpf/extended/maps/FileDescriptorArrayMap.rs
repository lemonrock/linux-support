// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An array map specialized for use with file descriptors.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FileDescriptorArrayMap<'map_file_descriptor_label_map, FD: FileDescriptor>(ArrayMap<'map_file_descriptor_label_map, RawFd>, PhantomData<FD>);

impl<'map_file_descriptor_label_map, FD: FileDescriptor> FileDescriptorArrayMap<'map_file_descriptor_label_map, FD>
{
	/// Length.
	#[inline(always)]
	pub fn length(&self) -> NonZeroU32
	{
		self.0.length()
	}
	
	/// Gets the next index (key).
	///
	/// Returns `None` if the `index` is the last one (ie `length() - 1`).
	///
	/// Does not make a syscall.
	#[inline(always)]
	pub fn get_next_index(&self, index: u32) -> Option<u32>
	{
		self.0.get_next_index(index)
	}
	
	/// Returns a file descriptor, if there is one.
	#[allow(deprecated)]
	pub fn get(&self, index: u32) -> Option<FileDescriptorCopy<FD>>
	{
		let raw_fd = self.0.get(index);
		if raw_fd == 0
		{
			None
		}
		else
		{
			Some(FileDescriptorCopy::new(raw_fd))
		}
	}
	
	/// Update existing.
	#[inline(always)]
	pub fn set(&self, index: u32, file_descriptor: &FD)
	{
		self.0.set(index, file_descriptor.as_raw_fd())
	}
	
	/// New Extended BPF program array.
	#[inline(always)]
	fn create(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, map_type: impl FnOnce(MaximumEntries, AccessPermissions) -> MapType) -> Result<Self, MapCreationError>
	{
		ArrayMap::create(map_file_descriptors, map_name, parsed_btf_map_data, map_type(maximum_entries, access_permissions)).map(|array_map| Self(array_map, PhantomData))
	}
}

impl<'map_file_descriptor_label_map, FD: FileDescriptor> FileDescriptorArrayMap<'map_file_descriptor_label_map, ExtendedBpfProgramFileDescriptor>
{
	/// New Extended BPF program array.
	#[inline(always)]
	pub fn new_extended_bpf_program(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, maximum_entries, access_permissions, MapType::ProgramArray)
	}
}

impl<'map_file_descriptor_label_map, FD: FileDescriptor> FileDescriptorArrayMap<'map_file_descriptor_label_map, PerfEventProgramFileDescriptor>
{
	/// New Extended BPF program array.
	#[inline(always)]
	pub fn new_perf_event(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, maximum_entries, access_permissions, MapType::PerfEventArray)
	}
}

impl<'map_file_descriptor_label_map, FD: FileDescriptor> FileDescriptorArrayMap<'map_file_descriptor_label_map, CgroupFileDescriptor>
{
	/// New Extended BPF program array.
	#[inline(always)]
	pub fn new_cgroup(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, maximum_entries, access_permissions, MapType::CgroupArray)
	}
}
