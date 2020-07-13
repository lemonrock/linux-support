// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// When an array is created, all its elements are zeroed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayMap<'map_file_descriptor_label_map, V: Sized>
{
	map_file_descriptor: &'map_file_descriptor_label_map MapFileDescriptor,
	maximum_entries: NonZeroU32,
	marker: PhantomData<V>,
}

impl<'map_file_descriptor_label_map, V: Sized> ArrayMap<'map_file_descriptor_label_map, V>
{
	/// New per-device.
	#[inline(always)]
	pub fn new_per_device(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, value_size: ValueSizeU32, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, device: NetworkInterfaceIndex) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, MapType::ArrayPerDevice(value_size, maximum_entries, access_permissions, device))
	}
	
	/// New per-CPU.
	#[inline(always)]
	pub fn new_per_cpu(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, value_size: ValueSizeU32, maximum_entries: MaximumEntries, access_permissions: AccessPermissions) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, MapType::ArrayPerCpu(value_size, maximum_entries, access_permissions))
	}
	
	/// New system-wide.
	#[inline(always)]
	pub fn new_system_wide(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, value_size: ValueSizeU32, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::new_system_wide_internal(map_file_descriptors, map_name, parsed_btf_map_data, value_size, maximum_entries, access_permissions, numa_node, MemoryMap::DoNotMemoryMap)
	}
	
	#[inline(always)]
	fn new_system_wide_internal(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, value_size: ValueSizeU32, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>, memory_map: MemoryMap) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, MapType::ArraySystemWide(value_size, maximum_entries, access_permissions, numa_node, memory_map))
	}
	
	/// Length.
	#[inline(always)]
	pub fn length(&self) -> NonZeroU32
	{
		self.maximum_entries
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
	pub fn set(&self, index: u32, value: V)
	{
		self.update_existing(index, value, BPF_MAP_UPDATE_ELEM_flags::BPF_EXIST)
	}
	
	/// Update existing, locked.
	///
	/// Only valid for basic hash, basic array and cgroup local-storage when `V` has been created with a `bpf_spin_lock`.
	///
	/// This will panic if this array was not created with a spinlock; it is not valid on per-CPU and per-device arrays nor if the array has been memory-mapped.
	/// Additionally, the map needs to have been created with BTF data.
	/// Furthermore, `V` must be a struct whose first field is `spin_lock: bpf_spin_lock`.
	#[inline(always)]
	fn set_locked(&self, index: u32, value: V)
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
				flags,
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
	
	#[inline(always)]
	fn create(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, map_type: MapType) -> Result<Self, MapCreationError>
	{
		MapFileDescriptor::create(map_file_descriptors, map_type, map_name, parsed_btf_map_data).map(|map_file_descriptor| Self::new(map_file_descriptor, maximum_entries))
	}
	
	#[inline(always)]
	const fn new(map_file_descriptor: &'map_file_descriptor_label_map MapFileDescriptor, maximum_entries: MaximumEntries) -> Self
	{
		Self
		{
			map_file_descriptor,
			maximum_entries,
			marker: PhantomData
		}
	}
	
	#[inline(always)]
	fn map_fd(&self) -> RawFd
	{
		self.map_file_descriptor.as_raw_fd()
	}
}
