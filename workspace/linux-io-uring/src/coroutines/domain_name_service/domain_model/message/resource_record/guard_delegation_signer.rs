// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! guard_delegation_signer
{
	($self: ident, $end_of_name_pointer: ident, $end_of_message_pointer: ident, $resource_record_name: ident, $resource_record_visitor: ident, $ignored_callback: ident, $visit_callback: ident, $permit_delete: expr, $data_type: ident, $response_parsing_state: ident) =>
	{
		{
			let (time_to_live, resource_data) = $self.validate_class_is_internet_and_get_time_to_live_and_resource_data($resource_record_name, $end_of_name_pointer, $end_of_message_pointer, $data_type, $response_parsing_state)?;

			use self::DelegationSignerResourceRecordIgnoredBecauseReason::*;
			use self::DigestAlgorithmRejectedBecauseReason::*;
			use self::DnsSecDigest::*;

			const KeyTagSize: usize = 2;
			const SecurityAlgorithmTypeSize: usize = 1;
			const DigestTypeSize: usize = 1;
			const MinimumDigestSize: usize = 0;

			let length = resource_data.len();
			if unlikely!(length < KeyTagSize + SecurityAlgorithmTypeSize + DigestTypeSize + MinimumDigestSize)
			{
				return Err(ResourceDataForTypeDSOrCDSHasAnIncorrectLength(length))
			}

			let resource_data_end_pointer = resource_data.end_pointer();

			let security_algorithm_type = resource_data.u8(KeyTagSize);
			let security_algorithm = match SecurityAlgorithm::parse_security_algorithm(security_algorithm_type, $permit_delete, false)?
			{
				Left(security_algorithm) => security_algorithm,
				Right(security_algorithm_rejected_because_reason) =>
				{
					$resource_record_visitor.$ignored_callback($resource_record_name, SecurityAlgorithmRejected(security_algorithm_rejected_because_reason));
					return Ok(resource_data_end_pointer)
				}
			};

			const DigestOffset: usize = KeyTagSize + SecurityAlgorithmTypeSize + DigestTypeSize;

			let digest_type = resource_data.u8(KeyTagSize + SecurityAlgorithmTypeSize);
			let digest = match digest_type
			{
				0 => return Err(DigestAlgorithmTypeIsReservedByRfc3658),

				1 =>
				{
					$resource_record_visitor.$ignored_callback($resource_record_name, DigestAlgorithmRejected(Sha1IsBroken));
					return Ok(resource_data_end_pointer)
				}

				2 => guard_hash_digest_if_final_field!(resource_data, DigestOffset, 256, Sha2_256, ResourceDataForTypeDSOrCDSHasADigestLengthThatIsIncorrectForTheDigestType),

				3 =>
				{
					$resource_record_visitor.$ignored_callback($resource_record_name, DigestAlgorithmRejected(Gost94MayBeBroken));
					return Ok(resource_data_end_pointer)
				}

				4 => guard_hash_digest_if_final_field!(resource_data, DigestOffset, 384, Sha2_384, ResourceDataForTypeDSOrCDSHasADigestLengthThatIsIncorrectForTheDigestType),

				_ =>
				{
					$resource_record_visitor.$ignored_callback($resource_record_name, DigestAlgorithmRejected(DigestAlgorithmRejectedBecauseReason::Unassigned(digest_type)));
					return Ok(resource_data_end_pointer)
				}
			};

			let record = DelegationSigner
			{
				key_tag: resource_data.value::<KeyTag>(0),
				security_algorithm,
				digest,
			};

			$resource_record_visitor.$visit_callback($resource_record_name, time_to_live, record)?;
			Ok(resource_data_end_pointer)
		}
	}
}
