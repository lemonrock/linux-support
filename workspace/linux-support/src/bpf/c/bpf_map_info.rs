// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Map information.
#[allow(missing_docs)]
#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct bpf_map_info
{
	pub(crate) type_: bpf_map_type,
	
	/// Identifier.
	pub id: MapIdentifier,
	
	pub key_size: NonZeroU32,
	pub value_size: NonZeroU32,
	pub max_entries: u32,
	pub(crate) map_flags: BPF_MAP_CREATE_flags,
	pub(crate) name: [c_char; BPF_OBJ_NAME_LEN],
	pub ifindex: Option<NetworkInterfaceIndex>,
	pub btf_vmlinux_value_type_id: BtfTypeIdentifier,
	pub netns_dev: u64,
	pub netns_ino: Inode,
	pub btf_id: BtfIdentifier,
	pub btf_key_type_id: BtfTypeIdentifier,
	pub btf_value_type_id: BtfTypeIdentifier,
}

impl bpf_map_info
{
	/// Name (clones internally).
	#[inline(always)]
	pub fn name(&self) -> MapName
	{
		MapName::from(&self.name)
	}
}
