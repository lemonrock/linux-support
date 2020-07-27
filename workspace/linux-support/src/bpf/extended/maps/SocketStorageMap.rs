// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Not a true map; nothing is stored.
///
/// Instead, provides a way to associate data with a particular socket.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketStorageMap<V: Copy>
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
	marker: PhantomData<V>,
}

impl<V: Copy> CanBeInnerMap for SocketStorageMap<V>
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.map_file_descriptor
	}
}

impl<V: Copy> SocketStorageMap<V>
{
	/// New u64 array.
	///
	/// Needs the capability `CAP_SYS_ADMIN`.
	#[inline(always)]
	pub fn new_socket_storage(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, clone_from_listener: CloneFromListener) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::SocketStorage(Self::value_size(), maximum_entries, clone_from_listener), maximum_entries)
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
	
	/// Get.
	#[inline(always)]
	pub fn get<SD: SocketData>(&self, key: &SocketFileDescriptor<SD>) -> Option<V>
	{
		self.map_file_descriptor.get(&key.as_raw_fd())
	}
	
	/// Insert or update.
	#[inline(always)]
	pub fn insert_or_set<SD: SocketData>(&self, key: &SocketFileDescriptor<SD>, value: &V) -> Result<(), ()>
	{
		self.map_file_descriptor.insert_or_set(&key.as_raw_fd(), value, LockFlags::DoNotLock)
	}
	
	/// Insert.
	#[inline(always)]
	pub fn insert<SD: SocketData>(&self, key: &SocketFileDescriptor<SD>, value: &V) -> Result<(), InsertError>
	{
		self.map_file_descriptor.insert(&key.as_raw_fd(), value, LockFlags::DoNotLock)
	}
	
	/// Update.
	#[inline(always)]
	pub fn set<SD: SocketData>(&self, key: &SocketFileDescriptor<SD>, value: &V) -> Result<(), ()>
	{
		self.map_file_descriptor.set(&key.as_raw_fd(), value, LockFlags::DoNotLock)
	}
	
	/// Removes.
	#[inline(always)]
	pub fn delete<SD: SocketData>(&self, key: &SocketFileDescriptor<SD>) -> Result<bool, Errno>
	{
		self.map_file_descriptor.delete(&key.as_raw_fd())
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
			marker: PhantomData
		}
	}
	
	#[inline(always)]
	fn value_size() -> ValueSizeU16
	{
		ValueSizeU16::try_from_value_size::<V>().unwrap()
	}
}
