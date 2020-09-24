// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An array map specialized for use with XDP.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExpressDataPathRedirectSocketArrayMap
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
}

impl CanBeInnerMap for ExpressDataPathRedirectSocketArrayMap
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.map_file_descriptor
	}
}

impl ExpressDataPathRedirectSocketArrayMap
{
	#[inline(always)]
	pub(crate) fn rehydrate(program_information: &bpf_prog_info, map_name: &MapName) -> Result<Self, MapRehydrateError>
	{
		use self::MapRehydrateError::*;
		
		let (map_file_descriptor, map_information) = program_information.filter_map_identifiers_for(KernelOnlyAccessPermissions::KernelReadAndWriteUserspaceReadWrite, map_name, bpf_map_type::BPF_MAP_TYPE_XSKMAP)?.ok_or(CouldNotGetExistingMapNamed(map_name.clone()))?;
		
		Ok
		(
			Self
			{
				map_file_descriptor: Rc::new(map_file_descriptor),
				maximum_entries: map_information.maximum_entries().ok_or(NoMaximumEntries)?,
			}
		)
	}
	
	/// A suitable map.
	pub fn new_express_data_path_redirect_socket_array_map_from_channels(map_name: &MapName, channels: Channels, map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, access_permissions: ExpressDataPathAccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		let maximum_entries = MaximumEntries::new(channels.maximum_number_of_queues().into());
		Self::new_express_data_path_redirect_socket_array(map_file_descriptors, map_name, None, maximum_entries, access_permissions, numa_node)
	}
	
	/// New u32 array.
	///
	/// Needs the capability `CAP_SYS_ADMIN`.
	#[inline(always)]
	pub fn new_express_data_path_redirect_socket_array(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: ExpressDataPathAccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::ExpressDataPathRedirectToSocketArray(maximum_entries, access_permissions, numa_node), maximum_entries)
	}
	
	/// Length.
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
	
	/// Insert or update existing.
	///
	/// `file_descriptor` must:-
	///
	/// * be `AF_XDP`.
	pub fn insert_or_set(&self, index: QueueIdentifier, file_descriptor: &ExpressDataPathSocketFileDescriptor) -> Result<(), ()>
	{
		self.guard_index(index);
		
		self.map_file_descriptor.insert_or_set(&index.0, &file_descriptor.as_raw_fd(), LockFlags::DoNotLock)
	}
	
	/// Insert.
	///
	/// `file_descriptor` must:-
	///
	/// * be `AF_XDP`.
	pub fn insert(&self, index: QueueIdentifier, file_descriptor: &ExpressDataPathSocketFileDescriptor) -> Result<(), InsertError>
	{
		self.guard_index(index);
		
		self.map_file_descriptor.insert(&index.0, &file_descriptor.as_raw_fd(), LockFlags::DoNotLock)
	}
	
	/// Removes a file descriptor.
	#[inline(always)]
	pub fn delete(&self, index: QueueIdentifier) -> Result<bool, Errno>
	{
		self.map_file_descriptor.delete(&index.0)
	}
	
	#[inline(always)]
	fn guard_index(&self, index: QueueIdentifier)
	{
		debug_assert!(index.0 < (self.maximum_entries.to_u32() as u16));
	}
	
	#[inline(always)]
	fn create(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, map_type: MapType, maximum_entries: MaximumEntries) -> Result<Self, MapCreationError>
	{
		MapFileDescriptor::create(map_file_descriptors, map_type, map_name, parsed_bpf_type_format_map_data).map(|map_file_descriptor| Self::new(map_file_descriptor, maximum_entries))
	}
	
	#[inline(always)]
	const fn new(map_file_descriptor: Rc<MapFileDescriptor>, maximum_entries: MaximumEntries) -> Self
	{
		Self
		{
			map_file_descriptor,
			maximum_entries,
		}
	}
}
