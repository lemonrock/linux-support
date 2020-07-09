// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A file descriptor for a BPF map.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapFileDescriptor(RawFd);


// if (attr->map_type != BPF_MAP_TYPE_ARRAY &&
// attr->map_type != BPF_MAP_TYPE_HASH)

impl MapFileDescriptor
{
	/// `numa_node` must be online.
	///
	/// If `offload_map_to_network_device` is `Some`, then:-
	/// * `parsed_btf_map_data` is ignored as BTF information is not permitted.
	/// * The capability `CAP_SYS_ADMIN` is needed.
	pub fn create<Key: Sized, Value: Sized>(map_type: MapType, map_name: &MapName, numa_node: Option<NumaNode>, parsed_btf_map_data: Option<&ParsedBtfMapData>, access_permissions: AccessPermissions) -> Result<Self, MapCreationError>
	{
		use self::MapCreationError::*;
		
		// This is size_of::<u32>() for:-
		// * BPF_MAP_TYPE_ARRAY
		// * BPF_MAP_TYPE_PROG_ARRAY
		let key_size;
		
		// This is size_of::<RawFd>() for:-
		// * BPF_MAP_TYPE_PERF_EVENT_ARRAY
		// * BPF_MAP_TYPE_CGROUP_ARRAY
		// * BPF_MAP_TYPE_ARRAY_OF_MAPS
		// * BPF_MAP_TYPE_PROG_ARRAY
		// * BPF_MAP_TYPE_HASH_OF_MAPS
		let value_size;
		
		
		let (map_type, map_flags, vmlinux_value_type_identifier, offload_map_to_network_device) = map_type.to_values();
		
		let map_flags = map_flags | access_permissions.to_map_flags();
		
		let (numa_node, map_flags) = match numa_node
		{
			None => (0, map_flags),
			Some(numa_node) => (numa_node.into(), map_flags | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE),
		};
		
		let (btf_fd, btf_key_type_id, btf_value_type_id, btf_vmlinux_value_type_id, map_ifindex) = ParsedBtfMapData::to_values(parsed_btf_map_data, vmlinux_value_type_identifier, offload_map_to_network_device)?;
		
		/*
		
		BPF_F_NO_PREALLOC;
		BPF_F_CLONE;
		
		// only if doesn't have a spinlock.
		BPF_F_MMAPABLE;
		
		 */
		
		
		
		let mut attributes = bpf_attr
		{
			map_create: BpfCommandMapCreate
			{
				map_type,
				key_size: X,
				value_size: X,
				max_entries: X,
				map_flags,
				inner_map_fd: X,
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
			Ok(Self(result))
		}
		else if likely!(result == 0)
		{
			match errno().0
			{
				ENOTSUPP => panic!("BTF settings are not allowed if the map is offloaded to a network device"),
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_CREATE)", result)
		}
	}
	
}
