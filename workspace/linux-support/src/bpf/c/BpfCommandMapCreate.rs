// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used by `BPF_MAP_CREATE` command.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct BpfCommandMapCreate
{
	/// one of enum bpf_map_type.
	pub(crate) map_type: bpf_map_type,
	
	/// size of key in bytes.
	pub(crate) key_size: NonZeroU32,
	
	/// size of value in bytes.
	pub(crate) value_size: NonZeroU32,
	
	/// max number of entries in a map.
	pub(crate) max_entries: u32,
	
	/// `BPF_MAP_CREATE` flags.
	pub(crate) map_flags: BPF_MAP_CREATE_flags,
	
	/// File descriptor pointing to the inner map, if any.
	pub(crate) inner_map_fd: RawFd,
	
	/// NUMA node (effective only if the `BPF_F_NUMA_NODE` flag is set.
	pub(crate) numa_node: u32,
	
	pub(crate) map_name: [c_char; BPF_OBJ_NAME_LEN],
	
	/// ifindex of netdev to create on.
	pub(crate) map_ifindex: Option<NetworkInterfaceIndex>,
	
	/// File descriptor pointing to a BTF (BPF Type Format) type data.
	pub(crate) btf_fd: RawFd,
	
	/// BTF (BPF Type Format) type_id of the key.
	pub(crate) btf_key_type_id: BtfTypeIdentifier,
	
	/// BTF (BPF Type Format) type_id of the value.
	pub(crate) btf_value_type_id: BtfTypeIdentifier,
	
	/// BTF (BPF Type Format) type_id of a kernel-struct stored as the map value.
	pub(crate) btf_vmlinux_value_type_id: BtfTypeIdentifier,
}
