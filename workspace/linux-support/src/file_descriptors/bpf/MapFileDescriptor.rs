// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A file descriptor for a BPF map.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapFileDescriptor(RawFd);

impl Drop for MapFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for MapFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for MapFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for MapFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for MapFileDescriptor
{
}

impl MemoryMappableFileDescriptor for MapFileDescriptor
{
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
	pub(crate) fn create<'map_file_descriptor_label_map>(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, map_type: MapType, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>) -> Result<&'map_file_descriptor_label_map Self, MapCreationError>
	{
		let (map_type, map_flags, (btf_fd, btf_key_type_id, btf_value_type_id, btf_vmlinux_value_type_id), map_ifindex, numa_node, inner_map_fd, key_size, value_size, max_entries) = map_type.to_values(parsed_btf_map_data, map_file_descriptors)?;
		
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
			let file_descriptor_label = FileDescriptorLabel::from(map_name);
			let map_file_descriptor = Self(result);
			Ok(map_file_descriptors.add(file_descriptor_label, map_file_descriptor)?)
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
