// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An array map specialized for use with sockets.
///
/// Values can only be retrieved if `SV` is `u64` (weird internal restriction in Linux).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketArrayMap<SV: SocketValue>
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
	marker: PhantomData<SV>,
}

impl<SV: SocketValue> CanBeInnerMap for SocketArrayMap<SV>
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.map_file_descriptor
	}
}

impl<SV: SocketValue> SocketArrayMap<SV>
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
	/// * be Internet Protocol version 4 or version 6.
	/// * be UDP (and `SOCK_DGRAM`) or TCP (and `SOCK_STREAM`).
	/// * be either a listening TCP socket (using `listen()`), an established TCP socket (using ?) or a bound UDP socket (using `bind()`).
	pub fn insert_or_set(&self, index: u32, file_descriptor: &impl ListenerSocketFileDescriptor) -> Result<(), ()>
	{
		self.guard_index(index);
		
		self.map_file_descriptor.insert_or_set(&index, &Self::to_value(file_descriptor), LockFlags::DoNotLock)
	}
	
	/// Insert.
	///
	/// `file_descriptor` must:-
	///
	/// * be Internet Protocol version 4 or version 6.
	/// * be UDP (and `SOCK_DGRAM`) or TCP (and `SOCK_STREAM`).
	/// * be either a listening TCP socket (using `listen()`), an established TCP socket (using ?) or a bound UDP socket (using `bind()`).
	pub fn insert(&self, index: u32, file_descriptor: &impl ListenerSocketFileDescriptor) -> Result<(), InsertError>
	{
		self.guard_index(index);
		
		self.map_file_descriptor.insert(&index, &Self::to_value(file_descriptor), LockFlags::DoNotLock)
	}
	
	/// Update existing.
	///
	/// `file_descriptor` must:-
	///
	/// * be Internet Protocol version 4 or version 6.
	/// * be UDP (and `SOCK_DGRAM`) or TCP (and `SOCK_STREAM`).
	/// * be either a listening TCP socket (using `listen()`), an established TCP socket (using ?) or a bound UDP socket (using `bind()`).
	pub fn set(&self, index: u32, file_descriptor: &impl ListenerSocketFileDescriptor) -> Result<(), ()>
	{
		self.guard_index(index);
		
		self.map_file_descriptor.set(&index, &Self::to_value(file_descriptor), LockFlags::DoNotLock)
	}
	
	/// Removes a file descriptor.
	#[inline(always)]
	pub fn delete(&self, index: u32) -> Result<bool, Errno>
	{
		self.map_file_descriptor.delete(&index)
	}
	
	#[inline(always)]
	fn to_value(file_descriptor: &impl ListenerSocketFileDescriptor) -> SV
	{
		let raw_fd = file_descriptor.as_raw_fd();
		SV::from_raw_fd(raw_fd)
	}
	
	#[inline(always)]
	fn guard_index(&self, index: u32)
	{
		debug_assert!(index < self.maximum_entries.to_u32());
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
}

impl SocketArrayMap<u32>
{
	/// New u32 array.
	///
	/// Needs the capability `CAP_SYS_ADMIN`.
	#[inline(always)]
	pub fn new_u32(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::SocketArrayU32(maximum_entries, access_permissions, numa_node), maximum_entries)
	}
}

impl SocketArrayMap<u64>
{
	/// New u64 array.
	///
	/// Needs the capability `CAP_SYS_ADMIN`.
	#[inline(always)]
	pub fn new_u64(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: KernelOnlyAccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, MapType::SocketArrayU64(maximum_entries, access_permissions, numa_node), maximum_entries)
	}
	
	/// Gets a socket cookie.
	pub fn get_socket_cookie(&self, index: u32) -> Option<SocketCookie>
	{
		self.guard_index(index);
		
		self.map_file_descriptor.get::<u32, SocketCookie>(&index)
	}
}
