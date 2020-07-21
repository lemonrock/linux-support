// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Not a true map; nothing is stored.
///
/// Instead, provides a way to associate data with a particular socket.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpinLockableSocketStorageMap<V: 'static + Copy + HasReflectionInformation>(SocketStorageMap<SpinLockableValue<V>>);

impl<V: 'static + Copy + HasReflectionInformation> SpinLockableSocketStorageMap<V>
{
	/// Needs the capability `CAP_SYS_ADMIN`.
	#[inline(always)]
	pub fn new_socket_storage(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: &ParsedBtfMapData, maximum_entries: MaximumEntries, clone_from_listener: CloneFromListener) -> Result<Self, MapCreationError>
	{
		let socket_storage_map = SocketStorageMap::new_socket_storage(map_file_descriptors, map_name, Some(parsed_btf_map_data), maximum_entries, clone_from_listener)?;
		Ok(Self(socket_storage_map))
	}
	
	/// Length.
	#[inline(always)]
	pub fn capacity(&self) -> NonZeroU32
	{
		self.0.capacity()
	}
	
	/// Freeze.
	#[inline(always)]
	pub fn freeze(&self) -> Result<(), Errno>
	{
		self.0.freeze()
	}
	
	/// Get.
	#[inline(always)]
	pub fn get<SD: SocketData>(&self, key: &SocketFileDescriptor<SD>) -> Option<SpinLockableValue<V>>
	{
		self.0.get(key)
	}
	
	/// Insert or update.
	#[inline(always)]
	pub fn insert_or_set<SD: SocketData>(&self, key: &SocketFileDescriptor<SD>, value: &SpinLockableValue<V>) -> Result<(), ()>
	{
		self.0.map_file_descriptor.insert_or_set(&key.as_raw_fd(), value, LockFlags::Lock)
	}
	
	/// Insert.
	#[inline(always)]
	pub fn insert<SD: SocketData>(&self, key: &SocketFileDescriptor<SD>, value: &SpinLockableValue<V>) -> Result<(), InsertError>
	{
		self.0.map_file_descriptor.insert(&key.as_raw_fd(), value, LockFlags::Lock)
	}
	
	/// Update.
	#[inline(always)]
	pub fn set<SD: SocketData>(&self, key: &SocketFileDescriptor<SD>, value: &SpinLockableValue<V>) -> Result<(), ()>
	{
		self.0.map_file_descriptor.set(&key.as_raw_fd(), value, LockFlags::Lock)
	}
	
	/// Removes.
	#[inline(always)]
	pub fn delete<SD: SocketData>(&self, key: &SocketFileDescriptor<SD>) -> Result<bool, Errno>
	{
		self.0.delete(key)
	}
}
