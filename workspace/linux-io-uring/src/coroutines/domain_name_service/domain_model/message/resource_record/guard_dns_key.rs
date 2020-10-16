// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! guard_dns_key
{
	($self: ident, $end_of_name_pointer: ident, $end_of_message_pointer: ident, $resource_record_name: ident, $resource_record_visitor: ident, $ignored_callback: ident, $visit_callback: ident, $permit_delete: expr, $data_type: ident, $response_parsing_state: ident) =>
	{
		{
			let (time_to_live, resource_data) = $self.validate_class_is_internet_and_get_time_to_live_and_resource_data($resource_record_name, $end_of_name_pointer, $end_of_message_pointer, $data_type, $response_parsing_state)?;

			use self::DnsKeyPurpose::*;
			use self::DnsKeyResourceRecordIgnoredBecauseReason::*;

			const FlagsSize: usize = 2;
			const ProtocolSize: usize = 1;
			const AlgorithmSize: usize = 1;
			const MinimumPublicKeySize: usize = 0;

			let length = resource_data.len();
			if unlikely!(length < FlagsSize + ProtocolSize)
			{
				return Err(ResourceDataForTypeDNSKEYOrCDNSKEYHasAnIncorrectLength(length))
			}

			let resource_data_end_pointer = resource_data.end_pointer();

			let protocol = resource_data.u8(FlagsSize);
			if unlikely!(protocol != 3)
			{
				$resource_record_visitor.$ignored_callback($resource_record_name, ProtocolWasNot3(protocol));
				return Ok(resource_data_end_pointer)
			}

			if unlikely!(length < FlagsSize + ProtocolSize + AlgorithmSize + MinimumPublicKeySize)
			{
				return Err(ResourceDataForTypeDNSKEYOrCDNSKEYHasAnIncorrectLength(length))
			}

			let flags = resource_data.u16_network_endian(0);

			const ZONE: u16 = 7;
			#[cfg(target_endian = "big")] const IsZoneKeyFlag: u16 = 1 << (15 - ZONE);
			#[cfg(target_endian = "little")] const IsZoneKeyFlag: u16 = 1 << ((15 - ZONE) - 8);

			const REVOKE: u16 = 8;
			#[cfg(target_endian = "big")] const RevokedFlag: u16 = 1 << (15 - REVOKE);
			#[cfg(target_endian = "little")] const RevokedFlag: u16 = 1 << ((15 - REVOKE) + 8);

			const SEP: u16 = 15;
			#[cfg(target_endian = "big")] const SecureEntryPointFlag: u16 = 1 << (15 - SEP);
			#[cfg(target_endian = "little")] const SecureEntryPointFlag: u16 = 1 << ((15 - SEP) + 8);

			const KnownFlags: u16 = IsZoneKeyFlag | IsZoneKeyFlag | SecureEntryPointFlag;

			if unlikely!(flags & !KnownFlags != 0)
			{
				$resource_record_visitor.$ignored_callback($resource_record_name, UnassignedFlags(flags));
				return Ok(resource_data_end_pointer)
			}

			let is_revoked = flags & RevokedFlag != 0;
			if unlikely!(is_revoked)
			{
				$resource_record_visitor.$ignored_callback($resource_record_name, Revoked);
				return Ok(resource_data_end_pointer)
			}

			let is_zone_key = flags & IsZoneKeyFlag != 0;
			let is_secure_entry_point = flags & SecureEntryPointFlag != 0;

			let purpose = if unlikely!(is_zone_key)
			{
				ZoneSigningKey { is_secure_entry_point }
			}
			else
			{
				if unlikely!(is_secure_entry_point)
				{
					$resource_record_visitor.$ignored_callback($resource_record_name, SecureEntryPointFlagSetButNotZoneKeyFlag);
					return Ok(resource_data_end_pointer)
				}
				KeySigningKey
			};

			let security_algorithm_type = resource_data.u8(FlagsSize + ProtocolSize);
			let security_algorithm = match SecurityAlgorithm::parse_security_algorithm(security_algorithm_type, $permit_delete, false)?
			{
				Left(security_algorithm) => security_algorithm,
				Right(security_algorithm_rejected_because_reason) =>
				{
					$resource_record_visitor.$ignored_callback($resource_record_name, SecurityAlgorithmRejected(security_algorithm_rejected_because_reason));
					return Ok(resource_data_end_pointer)
				}
			};

			let record = DnsKey
			{
				computed_key_tag: resource_data.key_tag(),
				purpose,
				security_algorithm,
				public_key: &resource_data[(FlagsSize + ProtocolSize + AlgorithmSize) .. ],
			};

			$resource_record_visitor.$visit_callback($resource_record_name, time_to_live, record)?;
			Ok(resource_data_end_pointer)
		}
	}
}
