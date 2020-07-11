// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// BTF data to describe the map.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParsedBtfMapData
{
	data: ParsedBtfData,
	key_type_identifier: BtfTypeIdentifier,
	value_type_identifier: BtfTypeIdentifier,
}

impl ParsedBtfMapData
{
	pub(crate) fn to_values(parsed_btf_map_data: Option<&Self>, vmlinux_value_type_identifier: BtfTypeIdentifier, offload_map_to_network_device: Option<NetworkInterfaceIndex>) -> Result<(RawFd, BtfTypeIdentifier, BtfTypeIdentifier, BtfTypeIdentifier, Option<NetworkInterfaceIndex>), MapCreationError>
	{
		const NoBtfFileDescriptor: RawFd = 0;
		
		const NoBtfData: (RawFd, BtfTypeIdentifier, BtfTypeIdentifier, BtfTypeIdentifier, Option<NetworkInterfaceIndex>) = (NoBtfFileDescriptor, BtfTypeIdentifier::Void, BtfTypeIdentifier::Void, vmlinux_value_type_identifier, None);
		
		if vmlinux_value_type_identifier.is_non_void()
		{
			debug_assert_eq!(offload_map_to_network_device, None);
			return Ok(NoBtfData)
		}
		
		match (parsed_btf_map_data, offload_map_to_network_device)
		{
			(&Some(Self { data, key_type_identifier, value_type_identifier }), None) =>
			{
				use self::MapCreationError::*;
				
				match (key_type_identifier, value_type_identifier)
				{
					(BtfTypeIdentifier::Void, BtfTypeIdentifier::Void) => (),
					
					(_, BtfTypeIdentifier::Void) => return Err(BtfMapHasAKeyTypeIdentifierButNotAValueTypeIdentifier),
					
					(BtfTypeIdentifier::Void, _) => return Err(BtfMapHasAVoidKeyTypeIdentifierButNotNotAValueTypeIdentifier),
					
					(_, _) => (),
				}
				
				Ok((data.to_raw_file_descriptor(), key_type_identifier, value_type_identifier, vmlinux_value_type_identifier, None))
			}
			
			_ => Ok(NoBtfData),
		}
	}
}
