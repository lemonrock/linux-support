// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Map information.
#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct bpf_map_info
{
	pub(crate) type_: bpf_map_type,
	
	pub(crate) id: MapIdentifier,
	
	pub(crate) key_size: NonZeroU32,
	
	/// Value size.
	pub value_size: ValueSizeU32,
	
	pub(crate) max_entries: Option<MaximumEntries>,
	
	pub(crate) map_flags: BPF_MAP_CREATE_flags,
	
	pub(crate) name: [c_char; BPF_OBJ_NAME_LEN],
	
	pub(crate) ifindex: Option<NetworkInterfaceIndex>,
	
	/// BTF type identifier used by Linux, if any.
	pub btf_vmlinux_value_type_id: BtfTypeIdentifier,
	
	pub(crate) netns_dev: u64,
	
	pub(crate) netns_ino: Inode,
	
	btf_id: BtfIdentifier,
	
	/// BTF type identifier for key, if any.
	pub btf_key_type_id: BtfTypeIdentifier,
	
	/// BTF type identifier for value, if any.
	pub btf_value_type_id: BtfTypeIdentifier,
}

impl Information for bpf_map_info
{
	type Identifier = MapIdentifier;
	
	#[inline(always)]
	fn identifier(&self) -> Self::Identifier
	{
		self.id
	}
}

impl bpf_map_info
{
	/// Name (clones internally).
	#[inline(always)]
	pub fn name(&self) -> MapName
	{
		MapName::from(&self.name)
	}
	
	/// Associated BTF identifier, if any.
	#[inline(always)]
	pub fn btf_identifier(&self) -> BtfIdentifier
	{
		self.btf_id
	}
	
	/// Network device bound to, if any.
	#[inline(always)]
	pub fn network_device_network_namespace_dev_and_network_namespace_inode(&self) -> (Option<NetworkInterfaceIndex>, u64, Inode)
	{
		(self.ifindex, self.netns_dev, self.netns_ino)
	}
	
	/// Key size.
	#[inline(always)]
	pub fn key_size(&self) -> KeySize
	{
		let key_size: u32 = self.key_size.get();
		let key_size: u16 = key_size.try_into().unwrap();
		KeySize::try_from(key_size).unwrap()
	}
	
	/// Maximum entries.
	#[inline(always)]
	pub fn maximum_entries(&self) -> Option<MaximumEntries>
	{
		self.max_entries
	}
}
