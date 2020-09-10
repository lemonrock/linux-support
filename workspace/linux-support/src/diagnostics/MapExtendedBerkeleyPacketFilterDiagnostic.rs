// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MapExtendedBerkeleyPacketFilterDiagnostic
{
	pub id: MapIdentifier,
	
	pub name: MapName,
	
	pub btf_identifier: BpfTypeFormatIdentifier,
	
	pub network_device_network_namespace_dev_and_network_namespace_inode: (Option<NetworkInterfaceIndex>, u64, Inode),
	
	pub key_size: KeySize,
	
	pub maximum_entries: Option<MaximumEntries>,
	
	pub btf_vmlinux_value_type_id: BpfTypeFormatTypeIdentifier,
	
	pub btf_key_type_id: BpfTypeFormatTypeIdentifier,
	
	pub btf_value_type_id: BpfTypeFormatTypeIdentifier,
	
	pub type_: bpf_map_type,
	
	pub map_create_flags: BPF_MAP_CREATE_flags,
}

impl ExtendedBerkeleyPacketFilterIdentifierDiagnostic for MapExtendedBerkeleyPacketFilterDiagnostic
{
	type BFD = MapFileDescriptor;
	
	#[inline(always)]
	fn map(information: <Self::BFD as BpfFileDescriptor>::Information) -> Self
	{
		Self
		{
			id: information.identifier(),
			
			name: information.name(),
			
			btf_identifier: information.btf_identifier(),
			
			network_device_network_namespace_dev_and_network_namespace_inode: information.network_device_network_namespace_dev_and_network_namespace_inode(),
			
			key_size: information.key_size(),
			
			maximum_entries: information.maximum_entries(),
			
			btf_vmlinux_value_type_id: information.btf_vmlinux_value_type_id,
			
			btf_key_type_id: information.btf_key_type_id,
			
			btf_value_type_id: information.btf_value_type_id,
			
			type_: information.type_,
		
			map_create_flags: information.map_flags
		}
	}
}
