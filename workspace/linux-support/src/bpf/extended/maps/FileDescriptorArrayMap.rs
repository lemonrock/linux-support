// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An array map specialized for use with file descriptors.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FileDescriptorArrayMap<'map_file_descriptor_label_map, FD: UsedAsValueInArrayMapDescriptor>(ArrayMap<'map_file_descriptor_label_map, RawFd>, PhantomData<FD>);

impl<'map_file_descriptor_label_map, FD: UsedAsValueInArrayMapDescriptor> FileDescriptorArrayMap<'map_file_descriptor_label_map, FD>
{
	/// Capacity.
	#[inline(always)]
	pub fn capacity(&self) -> NonZeroU32
	{
		self.0.capacity()
	}
	
	/// Get, batched.
	///
	/// Use `None` for `batch_position` when starting a new batch.
	/// Each value in `indices` must be valid.
	#[inline(always)]
	pub fn get_batch(&self, batch_position: Option<&OpaqueBatchPosition<u32>>, indices: &[u32]) -> Result<(Vec<Option<FileDescriptorCopy<FD>>>, OpaqueBatchPosition<u32>, bool), Errno>
	{
		let (vec, batch_position, more) = self.0.get_batch(batch_position, indices)?;
		Ok((FD::transmute_to_file_descriptor_copies(vec), batch_position, more))
	}
	
	/// Set, batched.
	///
	/// `indices` and `values` must be the same length.
	/// Each value in `indices` must be valid.
	#[inline(always)]
	pub fn set_batch(&self, indices: &[u32], values: &[Option<FileDescriptorCopy<FD>>]) -> Result<usize, Errno>
	{
		let values = FD::transmute_from_file_descriptor_copies(values);
		self.0.set_batch(indices, values)
	}
	
	/// Gets the next index (key).
	///
	/// Returns `None` if the `index` is the last one (ie `capacity() - 1`).
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
		FD::transmute_to_file_descriptor_copy(raw_fd)
	}
	
	/// Update existing.
	#[inline(always)]
	pub fn set(&self, index: u32, file_descriptor: &FD)
	{
		self.0.set(index, &file_descriptor.as_raw_fd())
	}
	
	#[inline(always)]
	fn create(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, map_type: impl FnOnce(MaximumEntries, AccessPermissions) -> MapType<'static>) -> Result<Self, MapCreationError>
	{
		ArrayMap::create(map_file_descriptors, map_name, parsed_btf_map_data, map_type(maximum_entries, access_permissions), maximum_entries).map(|array_map| Self(array_map, PhantomData))
	}
}

impl<'map_file_descriptor_label_map> FileDescriptorArrayMap<'map_file_descriptor_label_map, ExtendedBpfProgramFileDescriptor>
{
	/// New extended BPF program array.
	#[inline(always)]
	pub fn new_extended_bpf_program(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, maximum_entries, access_permissions, MapType::ProgramArray)
	}
}

impl<'map_file_descriptor_label_map> FileDescriptorArrayMap<'map_file_descriptor_label_map, PerfEventFileDescriptor>
{
	/// New perf event array.
	#[inline(always)]
	pub fn new_perf_event(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, maximum_entries, access_permissions, MapType::PerfEventArray)
	}
}

impl<'map_file_descriptor_label_map> FileDescriptorArrayMap<'map_file_descriptor_label_map, CgroupFileDescriptor>
{
	/// New cgroup array.
	#[inline(always)]
	pub fn new_cgroup(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, maximum_entries, access_permissions, MapType::CgroupArray)
	}
}
