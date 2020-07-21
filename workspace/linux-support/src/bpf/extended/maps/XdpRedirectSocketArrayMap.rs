// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An array map specialized for use with sockets.
///
/// Values can only be retrieved if `SV` is `u64` (weird internal restriction in Linux).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XdpRedirectSocketArrayMap
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
}

impl CanBeInnerMap for XdpRedirectSocketArrayMap
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.map_file_descriptor
	}
}

impl XdpRedirectSocketArrayMap
{
	/// New u32 array.
	///
	/// Needs the capability `CAP_SYS_ADMIN`.
	#[inline(always)]
	pub fn new_xdp_redirect_socket_array(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: XdpAccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, MapType::XdpRedirectToSocketArray(maximum_entries, access_permissions, numa_node), maximum_entries)
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
	
	/// Indices.
	#[inline(always)]
	pub fn indices(&self) -> RangeInclusive<u32>
	{
		0 ..= self.maximum_entries.0.get()
	}
	
	/// Insert or update existing.
	///
	/// `file_descriptor` must:-
	///
	/// * be `AF_XDP`.
	pub fn insert_or_set(&self, index: u32, file_descriptor: &impl FileDescriptor) -> Result<(), ()>
	{
		self.guard_index(index);
		
		self.map_file_descriptor.insert_or_set(&index, &file_descriptor.as_raw_fd(), LockFlags::DoNotLock)
	}
	
	/// Insert.
	///
	/// `file_descriptor` must:-
	///
	/// * be `AF_XDP`.
	pub fn insert(&self, index: u32, file_descriptor: &impl FileDescriptor) -> Result<(), InsertError>
	{
		self.guard_index(index);
		
		self.map_file_descriptor.insert(&index, &file_descriptor.as_raw_fd(), LockFlags::DoNotLock)
	}
	
	/// Removes a file descriptor.
	#[inline(always)]
	pub fn delete(&self, index: u32) -> Result<bool, Errno>
	{
		self.map_file_descriptor.delete(&index)
	}
	
	#[inline(always)]
	fn guard_index(&self, index: u32)
	{
		debug_assert!(index < self.maximum_entries.to_u32());
	}
	
	#[inline(always)]
	fn create(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, map_type: MapType, maximum_entries: MaximumEntries) -> Result<Self, MapCreationError>
	{
		MapFileDescriptor::create(map_file_descriptors, map_type, map_name, parsed_btf_map_data).map(|map_file_descriptor| Self::new(map_file_descriptor, maximum_entries))
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
