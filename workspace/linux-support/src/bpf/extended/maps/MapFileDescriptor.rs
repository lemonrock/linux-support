// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A file descriptor for a BPF map.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapFileDescriptor(RawFd);

// TODO: batch lookup and update only supported on some array maps and some hash maps.
// For array: BPF_MAP_LOOKUP_BATCH
// For array: BPF_MAP_UPDATE_BATCH

// For hash: BPF_MAP_DELETE_ELEM
// For hash: BPF_MAP_GET_NEXT_KEY
// For hash: BPF_MAP_GET_NEXT_KEY
// For hash: BPF_MAP_LOOKUP_AND_DELETE_ELEM
// For hash: BPF_MAP_LOOKUP_AND_DELETE_BATCH
// For hash: BPF_MAP_DELETE_BATCH

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
	pub fn create<'map_file_descriptor_label_map>(map_file_descriptors: &'map_file_descriptor_label_map mut FileDescriptorLabelsMap<MapFileDescriptor>, map_type: MapType, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>) -> Result<&'map_file_descriptor_label_map Self, MapCreationError>
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
