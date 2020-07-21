// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A hash map specialized for use with sockets.
///
/// Values can only be retrieved if `SV` is `u64` (weird internal restriction in Linux).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketHashMap<K: Copy, SV: SocketValue>
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
	marker: PhantomData<(K, SV)>,
}

impl<K: Copy, SV: SocketValue> CanBeInnerMap for SocketHashMap<K, SV>
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.map_file_descriptor
	}
}

impl<K: Copy, SV: SocketValue> SocketHashMap<K, SV>
{
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
	
	/// Iterator of keys.
	#[inline(always)]
	pub fn keys(&self) -> Result<KeyIterator<K>, Errno>
	{
		KeyIterator::new(&self.map_file_descriptor)
	}
	
	/// Insert or update exitings.
	///
	/// `file_descriptor` must:-
	///
	/// * be Internet Protocol version 4 or version 6.
	/// * be UDP (and `SOCK_DGRAM`) or TCP (and `SOCK_STREAM`).
	/// * be either a listening TCP socket (using `listen()`), an established TCP socket (using ?) or a bound UDP socket (using `bind()`).
	pub fn insert_or_set(&self, key: &K, file_descriptor: &impl ListenerSocketFileDescriptor) -> Result<(), ()>
	{
		self.map_file_descriptor.insert_or_set(key, &Self::to_value(file_descriptor), LockFlags::DoNotLock)
	}
	
	/// Insert.
	///
	/// `file_descriptor` must:-
	///
	/// * be Internet Protocol version 4 or version 6.
	/// * be UDP (and `SOCK_DGRAM`) or TCP (and `SOCK_STREAM`).
	/// * be either a listening TCP socket (using `listen()`), an established TCP socket (using ?) or a bound UDP socket (using `bind()`).
	pub fn insert(&self, key: &K, file_descriptor: &impl ListenerSocketFileDescriptor) -> Result<(), InsertError>
	{
		self.map_file_descriptor.insert(key, &Self::to_value(file_descriptor), LockFlags::DoNotLock)
	}
	
	/// Update existing.
	///
	/// `file_descriptor` must:-
	///
	/// * be Internet Protocol version 4 or version 6.
	/// * be UDP (and `SOCK_DGRAM`) or TCP (and `SOCK_STREAM`).
	/// * be either a listening TCP socket (using `listen()`), an established TCP socket (using ?) or a bound UDP socket (using `bind()`).
	pub fn set(&self, key: &K, file_descriptor: &impl ListenerSocketFileDescriptor) -> Result<(), ()>
	{
		self.map_file_descriptor.set(key, &Self::to_value(file_descriptor), LockFlags::DoNotLock)
	}
	
	/// Removes a file descriptor.
	#[inline(always)]
	pub fn delete(&self, key: &K) -> Result<bool, Errno>
	{
		self.map_file_descriptor.delete(key)
	}
	
	#[inline(always)]
	fn to_value(file_descriptor: &impl ListenerSocketFileDescriptor) -> SV
	{
		let raw_fd = file_descriptor.as_raw_fd();
		SV::from_raw_fd(raw_fd)
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
			marker: PhantomData
		}
	}
	
	#[inline(always)]
	fn key_size() -> KeySize
	{
		KeySize::try_from_key_size::<K>().unwrap()
	}
}

impl<K: Copy> SocketHashMap<K, u32>
{
	/// New u32 hash map.
	///
	/// Needs the capability `CAP_SYS_ADMIN`.
	#[inline(always)]
	pub fn new_u32(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, MapType::SocketHashU32(Self::key_size(), maximum_entries, access_permissions, numa_node), maximum_entries)
	}
}

impl<K: Copy> SocketHashMap<K, u64>
{
	/// New u64 hash map.
	///
	/// Needs the capability `CAP_SYS_ADMIN`.
	#[inline(always)]
	pub fn new_u64(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, MapType::SocketHashU64(Self::key_size(), maximum_entries, access_permissions, numa_node), maximum_entries)
	}
	
	/// Gets a socket cookie.
	pub fn get_socket_cookie(&self, key: &K) -> Option<SocketCookie>
	{
		self.map_file_descriptor.get::<K, SocketCookie>(key)
	}
}
