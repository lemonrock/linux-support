// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A file descriptor for a BPF map.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapFileDescriptor(RawFd);


/// When an array is created, all its elements are zeroed.
pub struct ArrayMap<'map_file_descriptor_label_map, V: Sized>
{
	file_descriptor: &'map_file_descriptor_label_map MapFileDescriptor,
	maximum_entries: NonZeroU32,
	marker: PhantomData<V>,
}

impl<'map_file_descriptor_label_map, V: Sized> ArrayMap<'map_file_descriptor_label_map, V>
{
	pub fn new_per_device(value_size: ValueSizeU32, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, device: NetworkInterfaceIndex) -> Result<Self, MapCreationError>
	{
	}
	
	pub fn new_per_cpu(value_size: ValueSizeU32, maximum_entries: MaximumEntries, access_permissions: AccessPermissions) -> Result<Self, MapCreationError>
	{
	}
	
	pub fn new_system_wide(value_size: ValueSizeU32, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>, memory_map: MemoryMap) -> Result<Self, MapCreationError>
	{
	}
	
	/// Gets the next index (key).
	///
	/// Returns `None` if the `index` is the last one (ie `length() - 1`).
	///
	/// Does not make a syscall.
	#[inline(always)]
	pub fn get_next_index(&self, index: u32) -> Option<u32>
	{
		if unlikely!(index >= self.maximum_entries.get())
		{
			Some(0)
		}
		else if unlikely!(index == self.maximum_entries.get() - 1)
		{
			None
		}
		else
		{
			Some(index + 1)
		}
	}
	
	// TODO: batch lookup and update only supported on some array maps and some hash maps.
	
	/// Looks up an index; should always succeed.
	#[allow(deprecated)]
	pub fn get(&self, index: u32) -> V
	{
		debug_assert!(index < self.length());
		
		let mut value: V = unsafe { uninitialized() };
		
		let mut attr = bpf_attr
		{
			map_change: BpfCommandMapChange
			{
				map_fd: self.map_fd(),
				key: AlignedU64::from(&index),
				value_or_next_key: BpfCommandMapChangeValueOrNextKey
				{
					value: AlignedU64::from(&mut value),
				},
				flags: BPF_MAP_UPDATE_ELEM_flags::BPF_ANY,
			},
		};
		
		let result = attr.syscall(bpf_cmd::BPF_MAP_LOOKUP_ELEM);
		if likely!(result == 0)
		{
			value
		}
		else if likely!(result == -1)
		{
			panic!("Unexpected error {}", errno())
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_LOOKUP_ELEM)", result)
		}
	}
	
	/// Update existing.
	pub fn set_unlocked(&self, index: u32, value: V)
	{
		self.update_existing(index, value, BPF_MAP_UPDATE_ELEM_flags::BPF_EXIST)
	}
	
	/// Update existing, locked.
	///
	/// This will panic if this array was not created with a spinlock; it is not valid on per-CPU and per-device arrays.
	/// The rules for creating an array with a spinlock seem complicated.
	pub fn set_locked(&self, index: u32, value: V)
	{
		self.update_existing(index, value, BPF_MAP_UPDATE_ELEM_flags::BPF_EXIST | BPF_MAP_UPDATE_ELEM_flags::BPF_F_LOCK)
	}
	
	#[inline(always)]
	fn update_existing(&self, index: u32, value: V, flags: BPF_MAP_UPDATE_ELEM_flags)
	{
		debug_assert!(index < self.length());
		
		let mut attr = bpf_attr
		{
			map_change: BpfCommandMapChange
			{
				map_fd: self.map_fd(),
				key: AlignedU64::from(&index),
				value_or_next_key: BpfCommandMapChangeValueOrNextKey
				{
					value: AlignedU64::from(&value),
				},
				flags: BPF_MAP_UPDATE_ELEM_flags::BPF_EXIST | BPF_MAP_UPDATE_ELEM_flags::BPF_F_LOCK,
			},
		};
		
		let result = attr.syscall(bpf_cmd::BPF_MAP_UPDATE_ELEM);
		if likely!(result == 0)
		{
		}
		else if likely!(result == -1)
		{
			panic!("Error {} (?due to locking)?", errno())
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_UPDATE)", result)
		}
	}
	
	/// Length.
	#[inline(always)]
	pub fn length(&self) -> NonZeroU32
	{
		self.maximum_entries
	}
	
	#[inline(always)]
	fn map_fd(&self) -> RawFd
	{
		self.file_descriptor.0
	}
}

impl MapFileDescriptor
{
	/// `parsed_btf_map_data` must be `None` for `map_type` when it is:-
	///
	/// * `MapType::HashPerDevice`.
	/// * `MapType::ArrayPerDevice`.
	/// * `MapType::StructOps`.
	///
	/// `parsed_btf_map_data` must be `Some` for `map_type` when it is:-
	///
	/// * `MapType::SocketStorage`.
	pub fn create<'map_file_descriptor_label_map, Key: Sized, Value: Sized>(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, map_type: MapType, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>) -> Result<&'map_file_descriptor_label_map Self, MapCreationError>
	{
		use self::MapCreationError::*;
		
		let btf_key_value_type_identifiers = match parsed_btf_map_data
		{
			None => None,
			&Some(ParsedBtfMapData { btf_key_value_type_identifiers, .. }) => Some(btf_key_value_type_identifiers),
		};
		
		let (map_type, map_flags, (btf_fd, btf_key_type_id, btf_value_type_id, btf_vmlinux_value_type_id), offload_map_to_network_device, numa_node, inner_map_fd, key_size, value_size, max_entries) = map_type.to_values(parsed_btf_map_data, map_file_descriptors)?;
		
		let mut attributes = bpf_attr
		{
			map_create: BpfCommandMapCreate
			{
				map_type,
				key_size,
				value_size,
				max_entries,
				map_flags,
				inner_map_fd,
				numa_node,
				map_name: map_name.to_bpf_object_name(),
				map_ifindex,
				
				btf_fd,
				btf_key_type_id,
				btf_value_type_id,
				btf_vmlinux_value_type_id,
			}
		};
		
		let result = attributes.syscall(bpf_cmd::BPF_MAP_CREATE);
		if likely!(result >= 0)
		{
			Ok(map_file_descriptors.add(FileDescriptorLabel(Name(Cow::from(name.to_string()))), map_file_descriptor))
		}
		else if likely!(result == -1)
		{
			Err(MapCreationError::CreateFailed(errno()))
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_CREATE)", result)
		}
	}
}
