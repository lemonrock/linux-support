// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! guard_hash_digest_if_final_field
{
	($resource_data: ident, $digest_offset: ident, $digest_size_in_bits: expr, $name: ident, $dns_protocol_error: ident) =>
	{
		{
			let digest_data = &$resource_data[$digest_offset .. ];

			let length = digest_data.len();

			const BitsInAByte: usize = 8;
			const DigestSizeInBytes: usize = $digest_size_in_bits / BitsInAByte;

			if unlikely!(length != DigestSizeInBytes)
			{
				return Err($dns_protocol_error(length))
			}

			$name(digest_data.start_pointer().unsafe_cast::<[u8; DigestSizeInBytes]>())
		}
	}
}

macro_rules! ipsec_like_public_key
{
	($public_key_algorithm_type: ident, $resource_data: ident, $public_key_starts_at_offset: ident, $public_key_length: ident, $resource_data_end_pointer: ident, $dsa_callback: block, $unassigned_callback: block) =>
	{
		{
			use self::PublicKey::*;

			match $public_key_algorithm_type
			{
				0 => if unlikely!($public_key_length != 0)
				{
					return Err(ResourceDataForTypeIPSECKEYOrHIPHasWrongLengthForNoPublicKey($public_key_length))
				}
				else
				{
					None
				},

				1 =>
				{
					$unassigned_callback;
					return Ok($resource_data_end_pointer)
				}

				2 =>
				{
					if unlikely!($public_key_length == 0)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForRSAPublicKey($public_key_length))
					}

					let public_key_data = &$resource_data[$public_key_starts_at_offset .. $public_key_starts_at_offset + $public_key_length];

					const FirstByteSize: usize = 1;

					let first_byte_of_exponent_length = public_key_data.u8(0);
					let (exponent_and_modulus, exponent_length) = if first_byte_of_exponent_length == 0
					{
						const SecondAndThirdBytesSize: usize = 2;

						const SizeSize: usize = FirstByteSize + SecondAndThirdBytesSize;

						if unlikely!(public_key_data.len() < SizeSize)
						{
							return Err(ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForRSAPublicKeyForAThreeByteExponentLength($public_key_length))
						}

						(&public_key_data[SizeSize .. ], public_key_data.u16_as_usize(FirstByteSize))
					}
					else
					{
						(&public_key_data[FirstByteSize .. ], first_byte_of_exponent_length as usize)
					};

					if unlikely!(exponent_length == 0)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasAZeroExponentForARSAPublicKey)
					}

					if unlikely!(exponent_and_modulus.len() < exponent_length)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForARSAPublicKeyForExponentLength)
					}

					let modulus_length = exponent_and_modulus.len() - exponent_length;
					if unlikely!(modulus_length == 0)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasAZeroModulusForARSAPublicKey)
					}

					let rsa_public_key = RsaPublicKey
					{
						exponent: &exponent_and_modulus[ .. exponent_length],
						modulus: &exponent_and_modulus[exponent_length .. ],
					};

					Some(RSA(rsa_public_key))
				}

				3 =>
				{
					const BitsInAByte: usize = 8;

					if unlikely!($public_key_length != 256 / BitsInAByte || $public_key_length != 384 / BitsInAByte)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasAUnusualLengthForAnECDSAPublicKey($public_key_length))
					}

					let public_key_data = &$resource_data[$public_key_starts_at_offset .. $public_key_starts_at_offset + $public_key_length];

					let ec_dsa_public_key = EcDsaPublicKey
					{
						Q: public_key_data,
					};

					Some(ECDSA(ec_dsa_public_key))
				}

				_ =>
				{
					$unassigned_callback;
					return Ok($resource_data_end_pointer)
				}
			}
		}
	}
}

macro_rules! guard_delegation_signer
{
	($self: ident, $end_of_name_pointer: ident, $end_of_message_pointer: ident, $resource_record_name: ident, $resource_record_visitor: ident, $ignored_callback: ident, $visit_callback: ident, $permit_delete: expr, $data_type: ident, $response_parsing_state: ident) =>
	{
		{
			let (time_to_live, resource_data) = $self.validate_class_is_internet_and_get_time_to_live_and_resource_data($end_of_name_pointer, $end_of_message_pointer, $data_type, $response_parsing_state)?;

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

macro_rules! guard_dns_key
{
	($self: ident, $end_of_name_pointer: ident, $end_of_message_pointer: ident, $resource_record_name: ident, $resource_record_visitor: ident, $ignored_callback: ident, $visit_callback: ident, $permit_delete: expr, $data_type: ident, $response_parsing_state: ident) =>
	{
		{
			let (time_to_live, resource_data) = $self.validate_class_is_internet_and_get_time_to_live_and_resource_data($end_of_name_pointer, $end_of_message_pointer, $data_type, $response_parsing_state)?;

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

pub(crate) struct ResourceRecord;

impl ResourceRecord
{
	const MinimumSize: usize = Name::MinimumSize + ResourceRecordFooter::MinimumSize;

	const BitsInAByte: usize = 8;

	const MinimumCharacterStringSize: usize = 1;

	const ExtendedDns0OptRecordWithoutOptionsSize: usize = Self::MinimumSize;
	
	pub(crate) const UdpRequestorsPayloadSize: u16 = (4096 - TcpMessage::TcpBufferLengthSize) as u16;

	#[inline(always)]
	pub(crate) fn write_extended_dns_0_opt_for_query(end_of_authority_section_pointer: usize) -> usize
	{
		const RootNameSize: usize = Name::MinimumSize;
		const RootName: u8 = 0x00;
		end_of_authority_section_pointer.set_u8(RootName);
		let mut current_pointer = end_of_authority_section_pointer + 1;

		current_pointer.set_u16_bytes(DataType::OPT.0);
		current_pointer += ResourceRecordFooter::DataSize;

		current_pointer.set_u16(Self::UdpRequestorsPayloadSize.to_be());
		current_pointer += ResourceRecordFooter::RequestorsUdpPayloadSize;

		current_pointer.set_u32_bytes(ExtendedResponseCodeAndFlags::new_for_query());
		current_pointer += ResourceRecordFooter::ExtendedRCodeAndFlagsSize;

		const NoOptionsSize: BigEndianU16 = [0, 0];
		current_pointer.set_u16_bytes(NoOptionsSize.to_be());
		current_pointer + ResourceRecordFooter::OptionsSize
	}

	#[inline(always)]
	pub(crate) fn parse_answer_section_resource_record_in_response<'message>(&'message mut self, question_q_type: DataType, end_of_message_pointer: usize, parsed_labels: &mut ParsedLabels, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (resource_record_name, end_of_name_pointer, (type_upper, type_lower)) = self.validate_minimum_record_size_and_parse_name_and_resource_record_type(end_of_message_pointer, parsed_labels)?;

		let (question_q_type_upper, question_q_type_lower) =  question_q_type.upper_and_lower();

		if likely!(type_upper == 0x00 && type_upper == question_q_type_upper)
		{
			if unlikely!(type_lower == DataType::CNAME_lower && type_lower == question_q_type_lower)
			{
				if likely!(response_parsing_state.have_yet_to_see_an_answer_section_cname_resource_record)
				{
					response_parsing_state.have_yet_to_see_an_answer_section_cname_resource_record = true;
					self.handle_cname(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, response_parsing_state)
				}
				else
				{
					Err(MoreThanOneCNAMERecordIsNotValidInAnswerSection)
				}
			}
			else if unlikely!(type_lower == DataType::DNAME_lower && type_lower == question_q_type_lower)
			{
				if likely!(response_parsing_state.have_yet_to_see_an_answer_section_cname_resource_record)
				{
					response_parsing_state.have_yet_to_see_an_answer_section_cname_resource_record = true;
					self.handle_dname(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, response_parsing_state)
				}
				else
				{
					Err(MoreThanOneCNAMERecordIsNotValidInAnswerSection)
				}
			}
			else
			{
				self.dispatch_resource_record_type(end_of_name_pointer, end_of_message_pointer, resource_record_name, parsed_labels, resource_record_visitor, response_parsing_state, true, false, (type_upper, type_lower))
			}
		}
		else
		{
			if likely!(type_upper == 0x00)
			{
				match type_lower
				{
					DataType::CNAME_lower =>
					{
						if likely!(response_parsing_state.have_yet_to_see_an_answer_section_cname_resource_record)
						{
							response_parsing_state.have_yet_to_see_an_answer_section_cname_resource_record = true;
							self.handle_cname(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, response_parsing_state)
						}
						else
						{
							Err(MoreThanOneCNAMERecordIsNotValidInAnswerSection)
						}
					}

					DataType::DNAME_lower =>
					{
						if likely!(response_parsing_state.have_yet_to_see_an_answer_section_cname_resource_record)
						{
							response_parsing_state.have_yet_to_see_an_answer_section_cname_resource_record = true;
							self.handle_dname(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state)
						}
						else
						{
							Err(MoreThanOneDNAMERecordIsNotValidInAnswerSection)
						}
					}

					DataType::RRSIG_lower => self.handle_rrsig(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

					_ => return Err(ResourceRecordTypeIsNotValidInAnswerSection(DataType([type_upper, type_lower])))
				}
			}
			else
			{
				return Err(ResourceRecordTypeIsNotValidInAnswerSection(DataType([type_upper, type_lower])))
			}
		}
	}

	/// Returns `Ok(end_of_resource_data_pointer)` unless there is an error.
	#[inline(always)]
	pub(crate) fn parse_authority_section_resource_record_in_response<'message>(&'message mut self, end_of_message_pointer: usize, parsed_labels: &mut ParsedLabels, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (resource_record_name, end_of_name_pointer, (type_upper, type_lower)) = self.validate_minimum_record_size_and_parse_name_and_resource_record_type(end_of_message_pointer, parsed_labels)?;

		// List of type codes based on https://serverfault.com/questions/899714/can-authority-rrs-contain-anything-but-ns-and-soa-additional-rrs-anything-but-a .
		if likely!(type_upper == 0x00)
		{
			match type_lower
			{
				// Referral.
				DataType::NS_lower => self.handle_ns(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, response_parsing_state),

				// Negative Response.
				DataType::SOA_lower => self.handle_soa(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, response_parsing_state),

				// Referral.
				DataType::DS_lower => self.handle_ds(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				// Signing negative response or referral.
				DataType::RRSIG_lower => self.handle_rrsig(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				// Signing negative response.
				DataType::NSEC_lower => self.handle_nsec(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				// Signing negative response.
				DataType::NSEC3_lower => self.handle_nsec3(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				_ => Err(ResourceRecordTypeIsNotValidInAuthoritySection(DataType([type_upper, type_lower])))
			}
		}
		else
		{
			Err(ResourceRecordTypeIsNotValidInAuthoritySection(DataType([type_upper, type_lower])))
		}
	}

	/// Returns `Ok(end_of_resource_data_pointer)` unless there is an error.
	#[inline(always)]
	pub(crate) fn parse_additional_section_resource_record_in_response<'message>(&'message mut self, end_of_message_pointer: usize, parsed_labels: &mut ParsedLabels, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (parsed_name_iterator, end_of_name_pointer, (type_upper, type_lower)) = self.validate_minimum_record_size_and_parse_name_and_resource_record_type(end_of_message_pointer, parsed_labels)?;

		self.dispatch_resource_record_type(end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, parsed_labels, resource_record_visitor, response_parsing_state, false, true, (type_upper, type_lower))
	}

	/// Compression of names within `RDATA` is a mess.
	///
	/// RFC 3597, Section 4, Paragraph 2 restricts the records to which name (label) compression can be applied to be those defined in RFC 1035 which implicitly contain a name, hence:-
	///
	/// * `CNAME`
	/// * `MB`
	/// * `MD`
	/// * `MF`
	/// * `MG`
	/// * `MINFO`
	/// * `MR`
	/// * `MX`
	/// * `NS`
	/// * `PTR`
	/// * `SOA`
	///
	/// Of these, many are obsolete, leaving the list as:-
	///
	/// * `CNAME`
	/// * `MX`
	/// * `NS`
	/// * `PTR`
	/// * `SOA`
	///
	/// Additionally:-
	///
	/// * RFC 2163 permits compression to `PX` records;
	/// * RFC 2535 permits compression in `SIG` and `NXT` records;
	/// * RFC 3597 permits compression in `RP`, `AFSDB`, `RT` and `NAPTR` records;
	/// * RFC 3597 prohibits compression in `PX`, `SIG` and `NXT` records;
	/// * RFC 2782 prohibits compression in `SRV` records but the original RFC 2052 mandated it;
	/// * RFC 3597 prohibits compression for all future record types;
	/// * RFC 6672 prohibits compression for `DNAME`, but historically, there was confusion in the original RFC 2672 about whether it was permitted.
	///
	/// Of the records listed in the clause above, all are obsolete apart from `NAPTR`, `SRV` and `DNAME`.
	///
	/// Observations:-
	///
	/// * Given the history of `SRV`, it seems prudent to permit compression.
	/// * Given the similarity of `DNAME` to `CNAME`, and the historic confusion, it seems prudent to permit compression;
	///
	/// This gives a list of
	///
	/// * `CNAME`
	/// * `MX`
	/// * `NS`
	/// * `PTR`
	/// * `SOA`
	/// * `NAPTR`
	/// * `SRV`
	/// * `DNAME`
	#[inline(always)]
	fn dispatch_resource_record_type<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, parsed_labels: &mut ParsedLabels, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>, soa_is_permitted: bool, opt_is_permitted: bool, (type_upper, type_lower): (u8, u8)) -> Result<usize, DnsProtocolError>
	{
		match type_upper
		{
			0x00 => match type_lower
			{
				DataType::SIG0_lower => self.handle_obsolete_meta_type(end_of_name_pointer, end_of_message_pointer, response_parsing_state, DataType::SIG0, "Only really useful for updates, which, frankly, are probably better done out-of-band than using DNS; regardless, when using DNS over TLS a client certificate is much more useful"),

				DataType::A_lower => self.handle_a(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::NS_lower => self.handle_ns(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, response_parsing_state),

				DataType::MD_lower => self.handle_very_obsolete_record_type(DataType::MD),

				DataType::MF_lower => self.handle_very_obsolete_record_type(DataType::MF),

				DataType::CNAME_lower => self.handle_cname(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, response_parsing_state),

				DataType::SOA_lower => if likely!(soa_is_permitted)
				{
					self.handle_soa(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, response_parsing_state)
				}
				else
				{
					Err(StartOfAuthorityResourceRecordTypeIsNotPermittedInThisSection)
				},

				DataType::MB_lower => self.handle_very_obsolete_record_type(DataType::MB),

				DataType::MG_lower => self.handle_very_obsolete_record_type(DataType::MG),

				DataType::MR_lower => self.handle_very_obsolete_record_type(DataType::MR),

				DataType::NULL_lower => self.handle_very_obsolete_record_type(DataType::NULL),

				DataType::WKS_lower => self.handle_very_obsolete_record_type(DataType::WKS),

				DataType::PTR_lower => self.handle_ptr(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, response_parsing_state),

				DataType::HINFO_lower => self.handle_hinfo(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::MINFO_lower => self.handle_very_obsolete_record_type(DataType::MINFO),

				DataType::MX_lower => self.handle_mx(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, response_parsing_state),

				DataType::TXT_lower => self.handle_txt(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::RP_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, response_parsing_state, DataType::RP, "Used in some rare circumstances; some legacy records may remain"),

				DataType::AFSDB_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, response_parsing_state, DataType::AFSDB, "Replaced by use of SRV records; some legacy records may remain"),

				DataType::X25_lower => self.handle_very_obsolete_record_type(DataType::X25),

				DataType::ISDN_lower => self.handle_very_obsolete_record_type(DataType::ISDN),

				DataType::RT_lower => self.handle_very_obsolete_record_type(DataType::RT),

				DataType::NSAP_lower => self.handle_very_obsolete_record_type(DataType::NSAP),

				DataType::NSAP_PTR_lower => self.handle_very_obsolete_record_type(DataType::NSAP_PTR),

				DataType::SIG_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, response_parsing_state, DataType::SIG, "Not used now SIG(0) is available; some legacy records may remain"),

				DataType::KEY_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, response_parsing_state, DataType::KEY, "Replaced by IPSECKEY and various DNSSEC records; some legacy records may remain"),

				DataType::PX_lower => self.handle_very_obsolete_record_type(DataType::PX),

				DataType::GPOS_lower => self.handle_very_obsolete_record_type(DataType::GPOS),

				DataType::AAAA_lower => self.handle_aaaa(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::LOC_lower => self.handle_loc(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::NXT_lower => self.handle_very_obsolete_record_type(DataType::NXT),

				DataType::EID_lower => self.handle_very_obsolete_record_type(DataType::EID),

				DataType::NIMLOC_lower => self.handle_very_obsolete_record_type(DataType::NIMLOC),

				DataType::SRV_lower => self.handle_srv(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, response_parsing_state),

				DataType::ATMA_lower => self.handle_very_obsolete_record_type(DataType::ATMA),

				DataType::NAPTR_lower => self.handle_naptr(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::KX_lower => self.handle_kx(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::CERT_lower => self.handle_cert(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::A6_lower => self.handle_very_obsolete_record_type(DataType::A6),

				DataType::DNAME_lower => self.handle_dname(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::SINK_lower => self.handle_very_obsolete_record_type(DataType::SINK),

				MetaType::OPT_lower => if likely!(opt_is_permitted)
				{
					self.handle_opt(end_of_name_pointer, end_of_message_pointer, response_parsing_state)
				}
				else
				{
					Err(ExtendedDnsOptResourceRecordTypeIsNotPermittedInThisSection)
				},

				DataType::APL_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, response_parsing_state, DataType::APL, "Some legacy records may remain"),

				DataType::DS_lower => self.handle_ds(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::SSHFP_lower => self.handle_sshfp(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::IPSECKEY_lower => self.handle_ipseckey(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::NSEC_lower => self.handle_nsec(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::RRSIG_lower => self.handle_rrsig(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::DNSKEY_lower => self.handle_dnskey(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::DHCID_lower => self.handle_dhcid(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::NSEC3_lower => self.handle_nsec3(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::NSEC3PARAM_lower => self.handle_nsec3param(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::TLSA_lower => self.handle_tlsa(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::SMIMEA_lower => self.handle_smimea(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				54 => self.handle_unassigned(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, 0x00, 54),

				DataType::HIP_lower => self.handle_hip(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::NINFO_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, response_parsing_state, DataType::NINFO, "No RFC or RFC draft and probably not deployed"),

				DataType::RKEY_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, response_parsing_state, DataType::RKEY, "No RFC or RFC draft and probably not deployed"),

				DataType::TALINK_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, response_parsing_state, DataType::TALINK, "No RFC or RFC draft and probably not deployed"),

				DataType::CDS_lower => self.handle_cds(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::CDNSKEY_lower => self.handle_cdnskey(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::OPENPGPKEY_lower => self.handle_openpgpkey(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::CSYNC_lower => self.handle_csync(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::ZONEMD_lower => self.handle_possible_future_standard(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state, DataType::ZONEMD),

				64 ..= 98 => self.handle_unassigned(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, 0x00, type_lower),

				DataType::SPF_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, response_parsing_state, DataType::SPF, "RFC 7208 deprecated this record type; some legacy records may remain"),

				DataType::UINFO_lower => self.handle_very_obsolete_record_type(DataType::UINFO),

				DataType::UID_lower => self.handle_very_obsolete_record_type(DataType::UID),

				DataType::GID_lower => self.handle_very_obsolete_record_type(DataType::GID),

				DataType::UNSPEC_lower => self.handle_very_obsolete_record_type(DataType::UNSPEC),

				DataType::NID_lower => self.handle_nid(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::L32_lower => self.handle_l32(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::L64_lower => self.handle_l64(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::LP_lower => self.handle_lp(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::EUI48_lower => self.handle_eui48(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::EUI64_lower => self.handle_eui64(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				110 ..= 127 => self.handle_unassigned(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, 0x00, type_lower),

				128 ..= 248 => Err(UnknownQueryTypeOrMetaType(0x00, type_lower)),

				MetaType::TKEY_lower => self.handle_obsolete_meta_type(end_of_name_pointer, end_of_message_pointer, response_parsing_state, DataType::TKEY, "Only really useful for updates, which, frankly, are probably better done out-of-band than using DNS; regardless, when using DNS over TLS a client certificate is much more useful"),

				MetaType::TSIG_lower => self.handle_obsolete_meta_type(end_of_name_pointer, end_of_message_pointer, response_parsing_state, DataType::TSIG, "Only really useful for updates, which, frankly, are probably better done out-of-band than using DNS; regardless, when using DNS over TLS a client certificate is much more useful"),

				QueryType::IXFR_lower => Err(QueryTypeIXFRShouldNotOccurOutsideOfAQuestionSectionEntry),

				QueryType::AXFR_lower => Err(QueryTypeAXFRShouldNotOccurOutsideOfAQuestionSectionEntry),

				QueryType::MAILB_lower => Err(QueryTypeMAILBShouldNotOccurOutsideOfAQuestionSectionEntry),

				QueryType::MAILA_lower => Err(QueryTypeMAILAShouldNotOccurOutsideOfAQuestionSectionEntry),

				QueryType::Asterisk_lower => Err(QueryTypeAsteriskShouldNotOccurOutsideOfAQuestionSectionEntry),
			},

			0x01 => match type_lower
			{
				DataType::URI_lower => self.handle_uri(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::CAA_lower => self.handle_caa(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state),

				DataType::DOA_lower => self.handle_possible_future_standard(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state, DataType::DOA),

				DataType::AMTRELAY_lower => self.handle_possible_future_standard(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, response_parsing_state, DataType::AMTRELAY),
				
				_ => self.handle_unassigned(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, 0x01, type_lower),
			},

			0x02 ..= 0x7F => self.handle_unassigned(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, type_upper, type_lower),

			0x80 => match type_lower
			{
				DataType::TA_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, response_parsing_state, DataType::TA, "DNSSEC Trust Anchors were never widely deployed; some legacy records may remain"),

				DataType::DLV_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, response_parsing_state, DataType::DLV, "DNSSEC Lookaside Validation is not longer supported now that all root nameservers support DNSSEC; some legacy records may remain"),

				_ => self.handle_unassigned(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, 0x80, type_lower),
			},

			0x81 ..= 0xEF => self.handle_unassigned(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, type_upper, type_lower),

			_ => Err(ReservedRecordType(type_upper, type_lower))
		}
	}

	/// Record types that died a very long time ago.
	#[inline(always)]
	fn handle_very_obsolete_record_type<'message>(&'message self, data_type: DataType) -> Result<usize, DnsProtocolError>
	{
		Err(VeryObsoleteResourceRecordType(data_type))
	}

	/// Record types that died, never became popular or widespread or never proceeded even to a RFC draft.
	#[inline(always)]
	fn handle_obsolete_or_very_obscure_record_type<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, response_parsing_state: &mut ResponseParsingState<'message>, data_type: DataType, _reason: &'static str) -> Result<usize, DnsProtocolError>
	{
		let (_time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, data_type, response_parsing_state)?;

		Ok(resource_data.end_pointer())
	}

	/// Meta types, that, with the coming of DNS over TLS, are obsolete.
	#[inline(always)]
	fn handle_obsolete_meta_type<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, response_parsing_state: &mut ResponseParsingState<'message>, data_type: DataType, _reason: &'static str) -> Result<usize, DnsProtocolError>
	{
		let (_time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, data_type, response_parsing_state)?;

		Ok(resource_data.end_pointer())
	}

	/// Data types that are draft RFCs or similar and may need to be supported by clients of this library.
	#[inline(always)]
	fn handle_possible_future_standard<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>, data_type: DataType) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, data_type, response_parsing_state)?;

		resource_record_visitor.handle_possible_future_standard(resource_record_name, time_to_live, resource_data, data_type)?;
		Ok(resource_data.end_pointer())
	}

	/// Data types that aren't officially registered with IANA.
	#[inline(always)]
	fn handle_unassigned<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, type_upper: u8, type_lower: u8) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, data_type, response_parsing_state)?;

		resource_record_visitor.unassigned(resource_record_name, time_to_live, resource_data, DataType([type_upper, type_lower]))?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_a<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_internet_protocol_address_only(end_of_name_pointer, end_of_message_pointer, DataType::A, response_parsing_state)?;
		resource_record_visitor.A(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_ns<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, parsed_labels: &mut ParsedLabels, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_name_only(end_of_name_pointer, end_of_message_pointer, parsed_labels, DataType::NS, response_parsing_state)?;
		resource_record_visitor.NS(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_cname<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, parsed_labels: &mut ParsedLabels, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_name_only(end_of_name_pointer, end_of_message_pointer, parsed_labels, DataType::CNAME, response_parsing_state)?;
		resource_record_visitor.CNAME(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_soa<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, parsed_labels: &mut ParsedLabels, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		if unlikely!(!response_parsing_state.have_yet_to_see_a_soa_resource_record)
		{
			return Err(MoreThanOneStatementOfAuthorityResourceRecord)
		}

		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::SOA, response_parsing_state)?;

		let start_of_resource_data = resource_data.start_pointer();

		let (primary_name_server, end_of_mname_pointer) = parsed_labels.parse_name_in_slice(resource_data)?;
		let (responsible_person_email_address, end_of_rname_pointer) = parsed_labels.parse_name_in_slice(&mut resource_data[(end_of_mname_pointer - start_of_resource_data) .. ])?;

		let end_of_resource_data = start_of_resource_data + resource_data.len();
		if likely!((end_of_resource_data - end_of_rname_pointer) == size_of::<StartOfAuthorityFooter>())
		{
			let start_of_authority = StartOfAuthority
			{
				primary_name_server,
				responsible_person_email_address,
				footer: end_of_rname_pointer.unsafe_cast::<StartOfAuthorityFooter>(),
			};

			resource_record_visitor.SOA(resource_record_name, time_to_live, start_of_authority)?;

			response_parsing_state.have_yet_to_see_a_soa_resource_record = false;

			Ok(resource_data.end_pointer())
		}
		else
		{
			Err(StartOfAuthorityIsIncorrectSizeAfterParsingMNAMEAndRNAME)
		}
	}

	#[inline(always)]
	fn handle_ptr<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, parsed_labels: &mut ParsedLabels, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_name_only(end_of_name_pointer, end_of_message_pointer, parsed_labels, DataType::PTR, response_parsing_state)?;
		resource_record_visitor.PTR(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_hinfo<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::HINFO, response_parsing_state)?;

		let length = resource_data.len();

		const MinimumCpuSize: usize = ResourceRecord::MinimumCharacterStringSize;
		const MinimumOsSize: usize = ResourceRecord::MinimumCharacterStringSize;

		if unlikely!(length < MinimumCpuSize + MinimumOsSize)
		{
			return Err(ResourceDataForTypeHINFOHasTooShortALength(length))
		}

		let mut character_strings_iterator = CharacterStringsIterator::new(resource_data)?;

		let cpu = character_strings_iterator.next().unwrap_or(Err(ResourceDataForTypeHINFOWouldHaveCpuDataOverflow(length)))?;

		let os = character_strings_iterator.next().unwrap_or(Err(ResourceDataForTypeHINFOWouldHaveOsDataOverflow(length)))?;

		if likely!(character_strings_iterator.is_empty())
		{
			let record = HostInformation
			{
				cpu,
				os,
			};

			resource_record_visitor.HINFO(resource_record_name, time_to_live, record)?;
			Ok(resource_data.end_pointer())
		}
		else
		{
			Err(ResourceDataForTypeHINFOWouldHaveUnusuedDataRemaining)
		}
	}

	#[inline(always)]
	fn handle_mx<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, parsed_labels: &mut ParsedLabels, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::MX, response_parsing_state)?;

		const PreferenceSize: usize = 2;
		const MinimumMailServerNameSize: usize = ResourceRecord::MinimumNameSize;

		let length = resource_data.len();

		if unlikely!(length < PreferenceSize + MinimumMailServerNameSize)
		{
			return Err(ResourceDataForTypeMXHasTooShortALength(length))
		}

		let record = MailExchange
		{
			preference: resource_data.u16(0),
			mail_server_name: parsed_labels.parse_name_in_slice_with_nothing_left(&mut resource_data[PreferenceSize .. ])?,
		};

		resource_record_visitor.MX(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_txt<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::TXT, response_parsing_state)?;

		let text_strings_iterator = CharacterStringsIterator::new(resource_data)?;

		resource_record_visitor.TXT(resource_record_name, time_to_live, text_strings_iterator)?;

		if likely!(text_strings_iterator.is_empty())
		{
			Ok(resource_data.end_pointer())
		}
		else
		{
			Err(ResourceDataForTypeTXTWouldHaveUnusuedDataRemaining)
		}
	}

	#[inline(always)]
	fn handle_aaaa<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_internet_protocol_address_only(end_of_name_pointer, end_of_message_pointer, DataType::AAAA, response_parsing_state)?;
		resource_record_visitor.AAAA(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_loc<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::LOC, response_parsing_state)?;

		let length = resource_data.len();
		if unlikely!(length != size_of::<Location>())
		{
			return Err(ResourceDataForTypeLOCHasAnIncorrectLength(length))
		}

		let location = resource_data.cast::<Location>(0);

		let version = location.version()?;
		debug_assert_eq!(version, LocationVersion::Version0, "Why are we supporting a version other than 0 of LOC records?");

		resource_record_visitor.LOC(resource_record_name, time_to_live, location)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_srv<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, parsed_labels: &mut ParsedLabels, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::SRV, response_parsing_state)?;

		const PrioritySize: usize = 2;
		const WeightSize: usize = 2;
		const PortSize: usize = 2;
		const MinimumTargetNameSize: usize = ResourceRecord::MinimumNameSize;

		let length = resource_data.len();
		if unlikely!(length < PrioritySize + WeightSize + PortSize + MinimumTargetNameSize)
		{
			return Err(ResourceDataForTypeSRVHasAnIncorrectLength(length))
		}

		let record = ServiceLocation
		{
			priority: resource_data.u16(0),
			weight: resource_data.u16(PrioritySize),
			port: resource_data.u16(PrioritySize + WeightSize),
			target: parsed_labels.parse_name_in_slice_with_nothing_left(&mut resource_data[(PrioritySize + WeightSize + PortSize) .. ])?,
		};

		resource_record_visitor.SRV(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_naptr<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::NAPTR, response_parsing_state)?;

		const OrderSize: usize = 2;
		const PreferenceSize: usize = 2;
		const MinimumFlagsSize: usize = ResourceRecord::MinimumCharacterStringSize;
		const MinimumServicesSize: usize = ResourceRecord::MinimumCharacterStringSize;
		const MinimumRegularExpressionSize: usize = ResourceRecord::MinimumCharacterStringSize;
		const MinimumDomainNameSize: usize = ResourceRecord::MinimumNameSize;

		let length = resource_data.len();
		if unlikely!(length < OrderSize + PreferenceSize + MinimumFlagsSize + MinimumServicesSize + MinimumRegularExpressionSize + MinimumDomainNameSize)
		{
			return Err(ResourceDataForTypeNAPTRHasAnIncorrectLength(length))
		}

		let order = resource_data.u16(0);
		let preference = resource_data.u16(OrderSize);

		let mut character_strings_iterator = CharacterStringsIterator::new(&resource_data[(OrderSize + PreferenceSize) .. ])?;

		let flags = character_strings_iterator.next().unwrap_or(Err(ResourceDataForTypeNAPTRIsMissingFlags))?;

		let services = character_strings_iterator.next().unwrap_or(Err(ResourceDataForTypeNAPTRIsMissingServices))?;

		let regular_expression = character_strings_iterator.next().unwrap_or(Err(ResourceDataForTypeNAPTRIsMissingRegularExpression))?;

		let remaining_resource_data = character_strings_iterator.remaining_resource_data();
		let start_of_name_pointer = remaining_resource_data.start_pointer();
		let resource_data_end_pointer = start_of_name_pointer + remaining_resource_data.len();

		let header = NamingAuthorityPointerHeader
		{
			order,
			preference,
			flags,
			services,
		};

		if regular_expression.is_empty()
		{
			let (domain_name, end_of_name_pointer) = WithoutCompressionParsedName::parse_without_compression(start_of_name_pointer, resource_data_end_pointer)?;
			if unlikely!(end_of_name_pointer != resource_data_end_pointer)
			{
				return Err(ResourceDataForTypeNAPTRHasDataLeftOver)
			}

			let record = NamingAuthorityPointerWithDomainName
			{
				header,
				domain_name
			};

			resource_record_visitor.NAPTR_domain_name(resource_record_name, time_to_live, record)?;
		}
		else
		{
			let end_of_name_pointer = start_of_name_pointer + 1;

			if unlikely!(end_of_name_pointer != resource_data_end_pointer)
			{
				return Err(ResourceDataForTypeNAPTRHasDataLeftOver)
			}

			let domain_name_byte = start_of_name_pointer.dereference_u8();
			if unlikely!(domain_name_byte != 0)
			{
				return Err(ResourceDataForTypeNAPTRHasBothARegularExpressionAndADomainName)
			}

			let record = NamingAuthorityPointerWithRegularExpression
			{
				header,
				regular_expression
			};

			resource_record_visitor.NAPTR_regular_expression(resource_record_name, time_to_live, record)?;
		}
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_kx<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::KX, response_parsing_state)?;

		let length = resource_data.len();

		const PreferenceSize: usize = 2;
		const MinimumKeyExchangeServerNameSize: usize = ResourceRecord::MinimumNameSize;

		if unlikely!(length < PreferenceSize + MinimumKeyExchangeServerNameSize)
		{
			return Err(ResourceDataForTypeKXHasTooShortALength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let (key_exchange_server_name, end_of_key_exchange_server_name) = WithoutCompressionParsedName::parse_without_compression(resource_data.start_pointer() + PreferenceSize, resource_data_end_pointer)?;

		if unlikely!(end_of_key_exchange_server_name != resource_data_end_pointer)
		{
			return Err(ResourceDataForTypeKXDataRemainingAfterKeyExchangeServerName)
		}

		let record = KeyExchange
		{
			preference: resource_data.u16(0),
			key_exchange_server_name,
		};

		resource_record_visitor.KX(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_cert<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::CERT, response_parsing_state)?;

		use self::CertificateResourceRecordIgnoredBecauseReason::*;
		use self::CertificateType::*;

		const CertificateTypeSize: usize = 2;
		const KeyTagSize: usize = 2;
		const AlgorithmSize: usize = 1;
		const MinimumCertificateOrCertificateRevocationListSize: usize = 0;
		const CertificateDataOffset: usize = CertificateTypeSize + KeyTagSize + AlgorithmSize;

		let length = resource_data.len();

		if unlikely!(length < CertificateTypeSize + KeyTagSize + AlgorithmSize + MinimumCertificateOrCertificateRevocationListSize)
		{
			return Err(ResourceDataForTypeCERTHasTooShortALength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let certificate_type_value_upper = resource_data.u8(0);
		let certificate_type_value_lower = resource_data.u8(1);
		let certificate_type = match certificate_type_value_upper
		{
			0x00 => match certificate_type_value_lower
			{
				0 => return Err(ResourceDataForTypeCERTUsesAReservedCertificateTypeValue(0)),

				1 => X509ASPerPkixCertificate(&resource_data[CertificateDataOffset .. ]),

				2 => SpkiCertificate(&resource_data[CertificateDataOffset .. ]),

				3 => OpenPgpPacket(&resource_data[CertificateDataOffset .. ]),

				4 => UrlOfAX509DataObject(&resource_data[CertificateDataOffset .. ]),

				5 => UrlOfASpkiCertificate(&resource_data[CertificateDataOffset .. ]),

				6 => FingerprintAndUrlOfAnOpenPgpPacket(&resource_data[CertificateDataOffset .. ]),

				7 => AttributeCertificate(&resource_data[CertificateDataOffset .. ]),

				8 => UrlOfAnAttributeCertificate(&resource_data[CertificateDataOffset .. ]),

				9 ..= 252 =>
				{
					resource_record_visitor.CERT_ignored(resource_record_name, CertificateTypeUnassigned(certificate_type_value_lower as u16));
					return Ok(resource_data_end_pointer)
				}

				253 =>
				{
					resource_record_visitor.CERT_ignored(resource_record_name, CertificateTypeUriPrivate);
					return Ok(resource_data_end_pointer)
				}

				254 =>
				{
					resource_record_visitor.CERT_ignored(resource_record_name, CertificateTypeOidPrivate);
					return Ok(resource_data_end_pointer)
				}

				255 => return Err(ResourceDataForTypeCERTUsesAReservedCertificateTypeValue(255)),
			},

			0x01 ..= 0xFE =>
			{
				resource_record_visitor.CERT_ignored(resource_record_name, CertificateTypeUnassigned((certificate_type_value_upper as u16) << 8 | (certificate_type_value_lower as u16)));
				return Ok(resource_data_end_pointer)
			}

			0xFF => if unlikely!(certificate_type_value_lower == 0xFF)
			{
				return Err(ResourceDataForTypeCERTUsesAReservedCertificateTypeValue(0xFF << 8 | (certificate_type_value_lower as u16)))
			}
			else
			{
				return Err(ResourceDataForTypeCERTUsesAnExperimentalCertificateTypeValue(0xFF << 8 | (certificate_type_value_lower as u16)))
			},
		};

		let security_algorithm_type = resource_data.u8(CertificateTypeSize + KeyTagSize);
		let security_algorithm = match SecurityAlgorithm::parse_security_algorithm(security_algorithm_type, false, false)?
		{
			Left(security_algorithm) => security_algorithm,

			Right(security_algorithm_rejected_because_reason) =>
			{
				resource_record_visitor.CERT_ignored(resource_record_name, SecurityAlgorithmRejected(security_algorithm_rejected_because_reason));
				return Ok(resource_data_end_pointer)
			}
		};

		let record = Certificate
		{
			key_tag: resource_data.value::<KeyTag>(CertificateTypeSize),
			security_algorithm,
			certificate_type,
		};

		resource_record_visitor.CERT(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_opt<'message>(&mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		if unlikely!(!response_parsing_state.have_yet_to_see_an_edns_opt_resource_record)
		{
			return Err(MoreThanOneExtendedDnsOptResourceRecord)
		}

		let start_of_name_pointer = self.name().as_usize_pointer_mut();
		if unlikely!(end_of_name_pointer - start_of_name_pointer != 1)
		{
			return Err(ExtendedDnsOptRecordNameTooLong)
		}

		let name_length_or_offset = start_of_name_pointer.dereference_u8();
		if unlikely!(name_length_or_offset != 0x00)
		{
			return Err(ExtendedDnsOptRecordNameNotRoot)
		}

		let requestors_udp_payload_size = max(512, self.requestors_udp_payload_size(end_of_name_pointer));

		let extended_response_code_and_flags = self.extended_response_code_and_flags(end_of_name_pointer);

		let upper_8_bits = extended_response_code_and_flags.extended_response_code_upper_8_bits();

		if unlikely!(upper_8_bits != 0)
		{
			return Err(ExtendedDnsOptUpper8BitsOfErrorNonZero(upper_8_bits))
		}

		let version = extended_response_code_and_flags.version()?;
		debug_assert_eq!(version, ExtendedDnsVersion::Version0, "Why do we support EDNS versions other than 0?");

		let dnssec_ok = extended_response_code_and_flags.dnssec_ok();

		extended_response_code_and_flags.z()?;

		let options = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		let mut start_of_option_offset = 0;
		let end_of_options_offset = options.len();
		while start_of_option_offset != end_of_options_offset
		{
			const OptionCodeSize: usize = 2;
			const OptionLengthSize: usize = 2;
			const MinimumOptionDataSize: usize = 2;

			if unlikely!(start_of_option_offset + OptionCodeSize + OptionLengthSize + MinimumOptionDataSize > end_of_options_offset)
			{
				return Err(ExtendedDnsOptionTooShort)
			}

			let option_length = options.u16_as_usize(start_of_option_offset + OptionCodeSize);
			if unlikely!(start_of_option_offset + option_length > end_of_options_offset)
			{
				return Err(ExtendedDnsOptionDataOverflows)
			}

			const LLQ_lower: u8 = 1;
			const UL_lower: u8 = 2;
			const NSID_lower: u8 = 3;
			const ReservedPendingUseAsOwnerOption_lower: u8 = 4;
			const DAU_lower: u8 = 5;
			const DHU_lower: u8 = 6;
			const N3U_lower: u8 = 7;
			const edns_client_subnet_lower: u8 = 8;
			const EDNS_EXPIRE_lower: u8 = 9;
			const COOKIE_lower: u8 = 10;
			const edns_tcp_keepalive_lower: u8 = 11;
			const Padding_lower: u8 = 12;
			const CHAIN_lower: u8 = 13;
			const edns_key_tag: u16 = 14;
			let option_code_lower = options.u8(start_of_option_offset);
			let option_code_upper = options.u8(start_of_option_offset + 1);
			match option_code_upper
			{
				0x00 => match option_code_lower
				{
					0 => return Err(ExtendedDnsOptionCodeWasReserved((0x00, 0x00))),

					LLQ_lower => return Err(ExtendedDnsOptionMustOnlyBeSetInARequest((0x00, LLQ_lower))),

					UL_lower => return Err(ExtendedDnsOptionMustOnlyBeSetInARequest((0x00, UL_lower))),

					NSID_lower => return Err(ExtendedDnsOptionMustOnlyBeSetInARequest((0x00, NSID_lower))),

					ReservedPendingUseAsOwnerOption_lower => (),

					DAU_lower => return Err(ExtendedDnsOptionMustOnlyBeSetInARequest((0x00, DAU_lower))),

					DHU_lower => return Err(ExtendedDnsOptionMustOnlyBeSetInARequest((0x00, DHU_lower))),

					N3U_lower => return Err(ExtendedDnsOptionMustOnlyBeSetInARequest((0x00, N3U_lower))),

					edns_client_subnet_lower => return Err(ExtendedDnsOptionMustOnlyBeSetInARequest((0x00, edns_client_subnet_lower))),

					EDNS_EXPIRE_lower => return Err(ExtendedDnsOptionMustOnlyBeSetInARequest((0x00, EDNS_EXPIRE_lower))),

					COOKIE_lower => return Err(ExtendedDnsOptionMustOnlyBeSetInARequest((0x00, COOKIE_lower))),

					edns_tcp_keepalive_lower => (),

					Padding_lower => (),

					CHAIN_lower => return Err(ExtendedDnsOptionMustOnlyBeSetInARequest((0x00, CHAIN_lower))),

					edns_key_tag_lower => return Err(ExtendedDnsOptionMustOnlyBeSetInARequest((0x00, edns_key_tag_lower))),
				}

				_ =>
				{
					let option_code = (option_code_upper as u16) << 8 | (option_code_lower as u16);
					if unlikely!(option_code >= 65001)
					{
						return Err(ExtendedDnsOptionCodeWasReserved((option_code_upper, option_code_lower)))
					}
				}
			}

			start_of_option_offset += OptionCodeSize + OptionLengthSize + option_length;
		}

		response_parsing_state.have_yet_to_see_an_edns_opt_resource_record = false;
		response_parsing_state.dnssec_ok = Some(dnssec_ok);

		Ok(options.end_pointer())
	}

	#[inline(always)]
	fn handle_dname<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::DNAME, response_parsing_state)?;

		let length = resource_data.len();

		const MinimumDNameSize: usize = ResourceRecord::MinimumNameSize;

		if unlikely!(length < MinimumDNameSize)
		{
			return Err(ResourceDataForTypeDNAMEHasTooShortALength(length))
		}

		let end_of_resource_data_pointer = resource_data.end_pointer();

		let (record, end_of_dname_pointer) = WithoutCompressionParsedName::parse_without_compression(resource_data.start_pointer(), end_of_resource_data_pointer)?;

		if unlikely!(end_of_dname_pointer != end_of_dname_pointer)
		{
			return Err(ResourceDataForTypeDNAMEDataRemainingAfterDName)
		}

		resource_record_visitor.DNAME(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_ds<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		const DataType: DataType = DataType::DS;
		guard_delegation_signer!(self, end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, DS_ignored, DS, false, DataType, response_parsing_state)
	}

	#[inline(always)]
	fn handle_sshfp<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::SSHFP, response_parsing_state)?;

		use self::SshFingerprintDigest::*;
		use self::SshFingerprintResourceRecordIgnoredBecauseReason::*;
		use self::SshPublicKeyAlgorithm::*;

		const PublicKeyAlgorithmSize: usize = 1;
		const DigestAlgorithmSize: usize = 1;
		const MinimumDigestSize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < PublicKeyAlgorithmSize + DigestAlgorithmSize + MinimumDigestSize)
		{
			return Err(ResourceDataForTypeSSHFPHasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let raw_public_key_algorithm = resource_data.u8(0);
		let public_key_algorithm: SshPublicKeyAlgorithm = match raw_public_key_algorithm
		{
			0 => return Err(ResourceDataForTypeSSHFPHasAReservedPublicKeyAlgorithm),

			1 => RSA,

			2 =>
			{
				resource_record_visitor.SSHFP_ignored(resource_record_name, PublicKeyAlgorithmDsaIsEffectivelyObsolete);
				return Ok(resource_data_end_pointer)
			}

			3 => ECDSA,

			4 => Ed25519,

			_ =>
			{
				resource_record_visitor.SSHFP_ignored(resource_record_name, PublicKeyAlgorithmUnassigned(raw_public_key_algorithm));
				return Ok(resource_data_end_pointer)
			}
		};

		const DigestOffset: usize = PublicKeyAlgorithmSize + DigestAlgorithmSize;

		let raw_digest_algorithm = resource_data.u8(1);
		let public_key_digest = match raw_digest_algorithm
		{
			0 => return Err(ResourceDataForTypeSSHFPHasAReservedDigestAlgorithm),

			1 =>
			{
				resource_record_visitor.SSHFP_ignored(resource_record_name, DigestAlgorithmSha1IsBroken);
				return Ok(resource_data_end_pointer)
			}

			2 => guard_hash_digest_if_final_field!(resource_data, DigestOffset, 256, Sha2_256, ResourceDataForTypeSSHFPAHasADigestLengthThatIsIncorrectForTheMatchingType),

			_ =>
			{
				resource_record_visitor.SSHFP_ignored(resource_record_name, DigestAlgorithmUnassigned(raw_digest_algorithm));
				return Ok(resource_data_end_pointer)
			}
		};

		let record = PublicKeyFingerprint
		{
			public_key_algorithm,
			public_key_digest,
		};

		resource_record_visitor.SSHFP(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_ipseckey<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::IPSECKEY, response_parsing_state)?;

		use self::Gateway::*;
		use self::IpsecKeyResourceRecordIgnoredBecauseReason::*;

		const PrecedenceSize: usize = 1;
		const GatewayTypeSize: usize = 1;
		const PublicKeyAlgorithmSize: usize = 1;
		const MinimumGatewaySize: usize = 0;
		const MinimumPublicKeySize: usize = 0;

		const GatewayFieldStartsAtOffset: usize = PrecedenceSize + GatewayTypeSize + PublicKeyAlgorithmSize;

		let length = resource_data.len();

		if unlikely!(length < PrecedenceSize + GatewayTypeSize + PublicKeyAlgorithmSize + MinimumGatewaySize + MinimumPublicKeySize)
		{
			return Err(ResourceDataForTypeIPSECKEYHasTooShortALength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let gateway_type = resource_data.u8(PrecedenceSize);
		let (public_key_starts_at_offset, gateway) = match gateway_type
		{
			0 => (GatewayFieldStartsAtOffset, None),

			1 =>
			{
				if unlikely!(length < GatewayFieldStartsAtOffset + size_of::<Ipv4Addr>())
				{
					return Err(ResourceDataForTypeIPSECKEYHasTooShortALengthForAnInternetProtocolVersion4Gateway(length))
				}
				let gateway = resource_data.value::<Ipv4Addr>(GatewayFieldStartsAtOffset);

				(GatewayFieldStartsAtOffset + size_of::<Ipv4Addr>(), Some(InternetProtocolVersion4(gateway)))
			}

			2 =>
			{
				if unlikely!(length < GatewayFieldStartsAtOffset + size_of::<Ipv6Addr>())
				{
					return Err(ResourceDataForTypeIPSECKEYHasTooShortALengthForAnInternetProtocolVersion6Gateway(length))
				}
				let gateway = resource_data.value::<Ipv6Addr>(GatewayFieldStartsAtOffset);

				(GatewayFieldStartsAtOffset + size_of::<Ipv6Addr>(), Some(InternetProtocolVersion6(gateway)))
			}

			3 =>
			{
				if unlikely!(length < GatewayFieldStartsAtOffset + 1)
				{
					return Err(ResourceDataForTypeIPSECKEYHasTooShortALengthForDomainNameGateway(length))
				}

				let resource_data_starts_at_pointer = resource_data.start_pointer();
				let start_of_name_pointer = resource_data_starts_at_pointer + GatewayFieldStartsAtOffset;
				let (domain_name, end_of_domain_name_pointer) = WithoutCompressionParsedName::parse_without_compression(start_of_name_pointer, start_of_name_pointer + length - GatewayFieldStartsAtOffset)?;

				(end_of_domain_name_pointer - resource_data_starts_at_pointer, Some(DomainName(domain_name)))
			}

			_ =>
			{
				resource_record_visitor.IPSECKEY_ignored(resource_record_name, GatewayTypeUnassigned(gateway_type));
				return Ok(resource_data_end_pointer)
			}
		};

		let public_key_algorithm_type = resource_data.u8(PrecedenceSize + GatewayTypeSize);
		let public_key_length = length - public_key_starts_at_offset;
		let public_key = ipsec_like_public_key!(public_key_algorithm_type, resource_data, public_key_starts_at_offset, public_key_length, resource_data_end_pointer, { resource_record_visitor.IPSECKEY_ignored(resource_record_name, PublicKeyAlgorithmDSAIsProbablyBroken) }, { resource_record_visitor.IPSECKEY_ignored(resource_record_name, PublicKeyAlgorithmUnassigned(public_key_algorithm_type)) });

		let record = IpsecPublicKey
		{
			precedence: resource_data.u8(0),
			gateway,
			public_key,
		};

		resource_record_visitor.IPSECKEY(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_nsec<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::NSEC, response_parsing_state)?;

		const MinimumNextSecureNameSize: usize = ResourceRecord::MinimumNameSize;

		let length = resource_data.len();
		if unlikely!(length < MinimumNextSecureNameSize + TypeBitmaps::MinimumTypeBitmapsSize)
		{
			return Err(ResourceDataForTypeNSECHasAnIncorrectLength(length))
		}

		let resource_data_pointer = resource_data.start_pointer();
		let resource_data_end_pointer = resource_data.end_pointer();

		let (next_domain_name, end_of_next_domain_name_pointer) = WithoutCompressionParsedName::parse_without_compression(resource_data_pointer, resource_data_end_pointer)?;

		let record = NextSecure
		{
			next_domain_name,
			type_bitmaps: TypeBitmaps::parse_type_bitmaps(&resource_data[(end_of_next_domain_name_pointer - resource_data_pointer) .. ])?,
		};

		resource_record_visitor.NSEC(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_rrsig<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::RRSIG, response_parsing_state)?;

		use self::ResourceRecordSetSignatureResourceRecordIgnoredBecauseReason::*;

		const TypeCoveredSize: usize = 2;
		const AlgorithmSize: usize = 1;
		const LabelsSize: usize = 1;
		const OriginalTimeToLiveSize: usize = 4;
		const SignatureExpirationSize: usize = 4;
		const SignatureInceptionSize: usize = 4;
		const KeyTagSize: usize = 2;
		const MinimumSignersNameSize: usize = ResourceRecord::MinimumNameSize;
		const MinimumSignatureSize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize + SignatureInceptionSize + KeyTagSize + MinimumSignersNameSize + MinimumSignatureSize)
		{
			return Err(ResourceDataForTypeRRSIGHasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let labels = resource_data.u8(TypeCoveredSize + AlgorithmSize);
		if unlikely!(labels > 126)
		{
			return Err(ResourceDataForTypeRRSIGHasMoreThan126Labels(labels))
		}

		let security_algorithm_type = resource_data.u8(TypeCoveredSize);
		let security_algorithm = match SecurityAlgorithm::parse_security_algorithm(security_algorithm_type, false, false)?
		{
			Left(security_algorithm) => security_algorithm,
			Right(security_algorithm_rejected_because_reason) =>
			{
				resource_record_visitor.RRSIG_ignored(resource_record_name, SecurityAlgorithmRejected(security_algorithm_rejected_because_reason));
				return Ok(resource_data_end_pointer)
			}
		};

		let signature_expiration_timestamp = resource_data.value::<SignatureTimestamp>(TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize);
		let signature_inception_timestamp = resource_data.value::<SignatureTimestamp>(TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize);
		match signature_expiration_timestamp.difference(&signature_inception_timestamp)
		{
			None =>
			{
				resource_record_visitor.RRSIG_ignored(resource_record_name, DifferenceInSignatureExpirationAndInceptionIsTooGreatForWrappingSerialNumberMathematics { signature_inception_timestamp, signature_expiration_timestamp });
				return Ok(resource_data_end_pointer)
			}

			Some((signature_expiration_seconds, signature_inception_seconds, difference)) => if unlikely!(difference <= 0)
			{
				resource_record_visitor.RRSIG_ignored(resource_record_name, DifferenceInSignatureInceptionAndExpirationWasNegativeOrZero { signature_inception_timestamp, signature_expiration_timestamp });
				return Ok(resource_data_end_pointer)
			}
			else
			{
				let now = get_time();

				const PeriodLengthInSeconds: i64 = ::std::u32::MAX as i64;

				// TODO: Increment this to 1 after Sunday, February 7, 2106 6:28:15 AM GMT.
				const ElapsedWrapAroundPoints: i64 = 0;
				const LastWrapAroundPoint: i64 = PeriodLengthInSeconds * ElapsedWrapAroundPoints;
				const NextWrapAroundPoint: i64 = LastWrapAroundPoint + PeriodLengthInSeconds;

				let signature_inception_timespec = if unlikely!(signature_inception_seconds > signature_expiration_seconds)
				{
					Timespec::new(NextWrapAroundPoint + signature_inception_seconds as i64, 0)
				}
				else
				{
					Timespec::new(LastWrapAroundPoint + signature_inception_seconds as i64, 0)
				};
				if unlikely!(signature_inception_timespec > now)
				{
					resource_record_visitor.RRSIG_ignored(resource_record_name, InceptionIsInTheFuture { signature_inception_timestamp, signature_expiration_timestamp });
					return Ok(resource_data_end_pointer)
				}

				let signature_expiration_timespec = Timespec::new(LastWrapAroundPoint + signature_expiration_seconds as i64, 0);
				if unlikely!(signature_expiration_timespec <= now)
				{
					resource_record_visitor.RRSIG_ignored(resource_record_name, Expired { signature_inception_timestamp, signature_expiration_timestamp });
					return Ok(resource_data_end_pointer)
				}
			},
		}

		let remaining_data = &resource_data[(TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize + SignatureInceptionSize + KeyTagSize) .. ];
		let remaining_data_pointer = remaining_data.start_pointer();

		let (signers_name, end_of_name_pointer) = WithoutCompressionParsedName::parse_without_compression(remaining_data.start_pointer(), resource_data_end_pointer)?;

		let signature_offset = TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize + SignatureInceptionSize + KeyTagSize + (end_of_name_pointer - remaining_data_pointer);
		let signature = &resource_data[signature_offset .. ];

		let record = ResourceRecordSetSignature
		{
			type_covered: resource_data.value::<DataType>(0),
			security_algorithm,
			labels,
			original_time_to_live: resource_data.value::<TimeToLiveInSeconds>(TypeCoveredSize + AlgorithmSize + LabelsSize),
			key_tag: resource_data.value::<KeyTag>(TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize + SignatureInceptionSize),
			signers_name,
			signature,
			rrsig_rdata_excluding_signature_field: &resource_data[ .. signature_offset],
		};

		resource_record_visitor.RRSIG(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_dnskey<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		const DataType: DataType = DataType::DNSKEY;
		guard_dns_key!(self, end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, DNSKEY_ignored, DNSKEY, false, DataType, response_parsing_state)
	}

	#[inline(always)]
	fn handle_dhcid<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::DHCID, response_parsing_state)?;

		use self::DhcidDigest::*;
		use self::DhcidResourceRecordIgnoredBecauseReason::*;

		const IdentifierTypeCodeSize: usize = 2;
		const DigestTypeCodeSize: usize = 1;
		const MinimumDigestSize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < IdentifierTypeCodeSize + DigestTypeCodeSize + MinimumDigestSize)
		{
			return Err(ResourceDataForTypeDHCIDHasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let identifier_type_code = resource_data.u16(0);
		let identifier_type: IdentifierType = match identifier_type_code
		{
			0x0000 ..= 0x0002 => unsafe { transmute(identifier_type_code) },

			0x0003 ..= 0xFFFE =>
			{
				resource_record_visitor.DHCID_ignored(resource_record_name, IdentifierTypeUnassigned(identifier_type_code));
				return Ok(resource_data_end_pointer)
			}

			_ => return Err(ResourceDataForTypeDHCIDHasAReservedIdentifierTypeCode)
		};

		const DigestOffset: usize = IdentifierTypeCodeSize + DigestTypeCodeSize;
		let digest = match resource_data.u8(IdentifierTypeCodeSize)
		{
			0 => return Err(ResourceDataForTypeDHCIDHasAReservedDigestTypeCode),

			1 => guard_hash_digest_if_final_field!(resource_data, DigestOffset, 256, Sha2_256, ResourceDataForTypeDHCIDHasADigestLengthThatIsIncorrectForTheMatchingType),

			digest_type_code @ _ =>
			{
				resource_record_visitor.DHCID_ignored(resource_record_name, DigestAlgorithmUnassigned(digest_type_code));
				return Ok(resource_data_end_pointer)
			}
		};

		let record = Dhcid
		{
			identifier_type,
			digest,
		};

		resource_record_visitor.DHCID(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_nsec3<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::NSEC3, response_parsing_state)?;

		use self::NextSecureVersion3ResourceRecordIgnoredBecauseReason::*;

		const HashAlgorithmSize: usize = 1;
		const FlagsSize: usize = 1;
		const IterationsSize: usize = 2;
		const SaltLengthSize: usize = 1;
		const MinimumSaltSize: usize = 0;
		const HashLengthSize: usize = 1;
		const MinimumHashSize: usize = 0;
		const SaltStartOffset: usize = HashAlgorithmSize + FlagsSize + IterationsSize + SaltLengthSize;

		let length = resource_data.len();
		if unlikely!(length < HashAlgorithmSize + FlagsSize + IterationsSize + SaltLengthSize + MinimumSaltSize + HashLengthSize + MinimumHashSize + TypeBitmaps::MinimumTypeBitmapsSize)
		{
			return Err(ResourceDataForTypeNSEC3HasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let salt_length = resource_data.u8_as_usize(HashAlgorithmSize + FlagsSize + IterationsSize);
		let salt_end_offset = HashAlgorithmSize + FlagsSize + IterationsSize + SaltLengthSize + salt_length;

		if unlikely!(salt_end_offset > length)
		{
			return Err(ResourceDataForTypeNSEC3HasAnOverflowingSaltLength(salt_length))
		}
		let salt = &resource_data[SaltStartOffset .. salt_end_offset];

		let hash_algorithm_number = resource_data.u8(0);
		let (next_hashed_owner_name, hash_end_offset) = match hash_algorithm_number
		{
			0 => return Err(ResourceDataForTypeNSEC3HasAReservedHashAlgorithm),

			1 =>
			{
				let hash_length = resource_data.u8_as_usize(salt_end_offset);

				const DigestSizeInBits: usize = 160;
				const DigestSize: usize = DigestSizeInBits / ResourceRecord::BitsInAByte;

				if unlikely!(hash_length != DigestSize)
				{
					return Err(ResourceDataForTypeNSEC3HasAnIncorrectHashLengthForASha1Hash(length))
				}

				let hash_start_offset = salt_end_offset + HashLengthSize;

				let hash_end_offset = hash_start_offset + hash_length;

				if unlikely!(hash_end_offset > length)
				{
					return Err(ResourceDataForTypeNSEC3HasAnOverflowingHashLength(hash_length))
				}

				let hash = NextSecureVersion3Hash::Sha_1(resource_data.cast::<[u8; DigestSize]>(hash_start_offset));

				(hash, hash_end_offset)
			}

			_ =>
			{
				resource_record_visitor.NSEC3_ignored(resource_record_name, UnassignedHashAlgorithm(hash_algorithm_number));
				return Ok(resource_data_end_pointer)
			}
		};

		let flags = resource_data.u8(HashAlgorithmSize);

		const OptOut: u8 = 7;
		const OptOutFlag: u8 = 1 << (7 - OptOut);
		const KnownFlags: u8 = OptOutFlag;

		if unlikely!(flags & !KnownFlags != 0)
		{
			resource_record_visitor.NSEC3_ignored(resource_record_name, UnassignedFlags(flags));
			return Ok(resource_data_end_pointer)
		}

		let record = NextSecureVersion3
		{
			opt_out: flags & OptOutFlag != 0,

			iterations: resource_data.u16(HashAlgorithmSize + FlagsSize),

			salt,

			next_hashed_owner_name,

			type_bitmaps: TypeBitmaps::parse_type_bitmaps(&resource_data[hash_end_offset .. ])?,
		};

		resource_record_visitor.NSEC3(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_nsec3param<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::NSEC3PARAM, response_parsing_state)?;

		use self::NextSecureVersion3ParametersResourceRecordIgnoredBecauseReason::*;

		const HashAlgorithmSize: usize = 1;
		const FlagsSize: usize = 1;
		const IterationsSize: usize = 2;
		const SaltLengthSize: usize = 1;
		const MinimumSaltSize: usize = 0;
		const SaltStartOffset: usize = HashAlgorithmSize + FlagsSize + IterationsSize + SaltLengthSize;

		let length = resource_data.len();
		if unlikely!(length < HashAlgorithmSize + FlagsSize + IterationsSize + SaltLengthSize + MinimumSaltSize)
		{
			return Err(ResourceDataForTypeNSEC3PARAMHasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let salt_length = resource_data.u8_as_usize(HashAlgorithmSize + FlagsSize + IterationsSize);
		let salt_end_offset = HashAlgorithmSize + FlagsSize + IterationsSize + SaltLengthSize + salt_length;

		if unlikely!(salt_end_offset > length)
		{
			return Err(ResourceDataForTypeNSEC3PARAMHasAnOverflowingSaltLength(salt_length))
		}
		let salt = &resource_data[SaltStartOffset .. salt_end_offset];

		let hash_algorithm_number = match resource_data.u8(0)
		{
			0 => return Err(ResourceDataForTypeNSEC3PARAMHasAReservedHashAlgorithm),

			1 => NextSecureVersion3Parameters::Sha1HashAlgorithmNumber,

			hash_algorithm_number @ _ =>
			{
				resource_record_visitor.NSEC3PARAM_ignored(resource_record_name, UnassignedHashAlgorithm(hash_algorithm_number));
				return Ok(resource_data_end_pointer)
			}
		};

		// We are meant to ignore the reserved flags.
		let flags = resource_data.u8(HashAlgorithmSize);
		if unlikely!(flags != 0)
		{
			resource_record_visitor.NSEC3PARAM_ignored(resource_record_name, UnassignedFlags(flags));
			return Ok(resource_data_end_pointer)
		}

		let record = NextSecureVersion3Parameters
		{
			hash_algorithm_number,

			iterations: resource_data.u16(HashAlgorithmSize + FlagsSize),

			salt,
		};

		resource_record_visitor.NSEC3PARAM(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_tlsa<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (resource_data_end_pointer, either) = self.handle_tlsa_or_smimea(end_of_name_pointer, end_of_message_pointer, DataType::TLSA, response_parsing_state)?;

		match either
		{
			Left((time_to_live, record)) => resource_record_visitor.TLSA(resource_record_name, time_to_live, record)?,

			Right(resource_record_ignored_because_reason) => resource_record_visitor.TLSA_ignored(resource_record_name, resource_record_ignored_because_reason),
		}

		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_smimea<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (resource_data_end_pointer, either) = self.handle_tlsa_or_smimea(end_of_name_pointer, end_of_message_pointer, DataType::SMIMEA, response_parsing_state)?;

		match either
		{
			Left((time_to_live, record)) => resource_record_visitor.SMIMEA(resource_record_name, time_to_live, record)?,

			Right(resource_record_ignored_because_reason) => resource_record_visitor.SMIMEA_ignored(resource_record_name, resource_record_ignored_because_reason),
		}

		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_hip<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::HIP, response_parsing_state)?;

		use self::HostIdentityProtocolResourceRecordIgnoredBecauseReason::*;

		const HostIdentityTagLengthSize: usize = 1;
		const PublicKeyAlgorithmTypeSize: usize = 1;
		const PublicKeyLengthSize: usize = 2;
		const MinimumHostIdentityTagLength: usize = 0;
		const MinimumPublicKeyLength: usize = 0;
		const MinimumNumberOfRendezvousServersIsOneSoMinimumNameSizeIsOne: usize = ResourceRecord::MinimumNameSize;
		const HostIdentityTagOffset: usize = HostIdentityTagLengthSize + PublicKeyAlgorithmTypeSize + PublicKeyLengthSize;

		let length = resource_data.len();
		if unlikely!(length < HostIdentityTagOffset + MinimumHostIdentityTagLength + MinimumPublicKeyLength + MinimumNumberOfRendezvousServersIsOneSoMinimumNameSizeIsOne)
		{
			return Err(ResourceDataForTypeHIPHasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let host_identity_tag_length = resource_data.u8_as_usize(0);
		if unlikely!(length < HostIdentityTagOffset + host_identity_tag_length + MinimumPublicKeyLength + MinimumNumberOfRendezvousServersIsOneSoMinimumNameSizeIsOne)
		{
			return Err(ResourceDataForTypeHIPHasAnIncorrectLength(length))
		}

		let public_key_algorithm_type = resource_data.u8(HostIdentityTagLengthSize);
		let public_key_starts_at_offset = HostIdentityTagOffset + host_identity_tag_length;
		let public_key_length = resource_data.u16_as_usize(HostIdentityTagLengthSize + PublicKeyAlgorithmTypeSize);
		let public_key = ipsec_like_public_key!(public_key_algorithm_type, resource_data, public_key_starts_at_offset, public_key_length, resource_data_end_pointer, { resource_record_visitor.HIP_ignored(resource_record_name, PublicKeyAlgorithmDSAIsProbablyBroken) }, { resource_record_visitor.HIP_ignored(resource_record_name, PublicKeyAlgorithmUnassigned(public_key_algorithm_type)) });

		let start_of_name_pointer = resource_data.start_pointer() + HostIdentityTagOffset + host_identity_tag_length + public_key_length;
		let (first_rendezvous_server_domain_name, true_end_of_name_pointer) = WithoutCompressionParsedName::parse_without_compression(start_of_name_pointer, resource_data_end_pointer)?;

		let remaining_rendezvous_servers_length = resource_data_end_pointer - true_end_of_name_pointer;
		let remaining_rendezvous_server_domain_names = true_end_of_name_pointer.unsafe_cast_slice::<u8>(remaining_rendezvous_servers_length);

		let record = HostIdentityProtocol
		{
			host_identity_tag: &resource_data[HostIdentityTagOffset .. public_key_starts_at_offset],

			public_key,

			first_rendezvous_server_domain_name,

			remaining_rendezvous_server_domain_names,
		};

		resource_record_visitor.HIP(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_cds<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		const DataType: DataType = DataType::CDS;
		guard_delegation_signer!(self, end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, CDS_ignored, CDS, true, DataType, response_parsing_state)
	}

	#[inline(always)]
	fn handle_cdnskey<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		const DataType: DataType = DataType::CDNSKEY;
		guard_dns_key!(self, end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, CDNSKEY_ignored, CDNSKEY, true, DataType, response_parsing_state)
	}

	#[inline(always)]
	fn handle_openpgpkey<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::OPENPGPKEY, response_parsing_state)?;

		resource_record_visitor.OPENPGPKEY(resource_record_name, time_to_live, resource_data)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_csync<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::CSYNC, response_parsing_state)?;

		use self::ChildSynchronizeResourceRecordIgnoredBecauseReason::*;

		const StartOfAuthoritySize: usize = 4;
		const FlagsSize: usize = 2;

		let length = resource_data.len();
		if unlikely!(length < StartOfAuthoritySize + FlagsSize + TypeBitmaps::MinimumTypeBitmapsSize)
		{
			return Err(ResourceDataForTypeCSYNCHasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let flags = resource_data.u16(StartOfAuthoritySize);

		const immediate: u16 = 0x0001;
		const soaminimum: u16 = 0x0002;
		const KnownFlags: u16 = immediate | soaminimum;

		if unlikely!(flags & !KnownFlags != 0)
		{
			resource_record_visitor.CSYNC_ignored(resource_record_name, UnassignedFlags(flags));
			return Ok(resource_data_end_pointer)
		}

		let record = ChildSynchronize
		{
			start_of_authority_serial: resource_data.value::<SerialNumber>(0),

			immediate: flags & immediate != 0,

			start_of_authority_minimum: flags & soaminimum != 0,

			type_bitmaps: TypeBitmaps::parse_type_bitmaps(&resource_data[(StartOfAuthoritySize + FlagsSize) .. ])?
		};

		resource_record_visitor.CSYNC(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_nid<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::NID, response_parsing_state)?;

		const PreferenceSize: usize = 2;
		const NodeIdentifierSize: usize = 8;

		let length = resource_data.len();
		if unlikely!(length != PreferenceSize + NodeIdentifierSize)
		{
			return Err(ResourceDataForTypeNIDHasAnIncorrectLength(length))
		}

		let record = NodeIdentifier
		{
			preference: resource_data.u16(0),
			node_identifier: resource_data.u64(PreferenceSize),
		};

		resource_record_visitor.NID(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_l32<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::L32, response_parsing_state)?;

		const PreferenceSize: usize = 2;
		const LocatorSize: usize = 4;

		let length = resource_data.len();
		if unlikely!(length != PreferenceSize + LocatorSize)
		{
			return Err(ResourceDataForTypeL32HasAnIncorrectLength(length))
		}

		let record = Locator32
		{
			preference: resource_data.u16(0),
			locator: resource_data.value::<Ipv4Addr>(PreferenceSize),
		};

		resource_record_visitor.L32(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_l64<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::L64, response_parsing_state)?;

		const PreferenceSize: usize = 2;
		const LocatorSize: usize = 8;

		let length = resource_data.len();
		if unlikely!(length != PreferenceSize + LocatorSize)
		{
			return Err(ResourceDataForTypeL64HasAnIncorrectLength(length))
		}

		let record = Locator64
		{
			preference: resource_data.u16(0),
			locator: resource_data.value::<[u8; LocatorSize]>(PreferenceSize),
		};

		resource_record_visitor.L64(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_lp<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::LP, response_parsing_state)?;

		const PreferenceSize: usize = 2;
		const MinimumNameSize: usize = ResourceRecord::MinimumNameSize;

		let length = resource_data.len();
		if unlikely!(length < PreferenceSize + MinimumNameSize)
		{
			return Err(ResourceDataForTypeLPHasTooShortALength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let domain_name_data = &resource_data[PreferenceSize .. ];
		let (domain_name, end_of_name_pointer) = WithoutCompressionParsedName::parse_without_compression(domain_name_data.start_pointer(), resource_data_end_pointer)?;
		if unlikely!(end_of_name_pointer != resource_data_end_pointer)
		{
			return Err(ResourceDataForTypeLPHasDataLeftOver)
		}

		let record = LocatorPointer
		{
			preference: resource_data.u16(0),
			domain_name,
		};

		resource_record_visitor.LP(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_eui48<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::EUI48, response_parsing_state)?;

		const Eui48Size: usize = 48 / ResourceRecord::BitsInAByte;

		let length = resource_data.len();
		if unlikely!(length != Eui48Size)
		{
			return Err(ResourceDataForTypeEUI48HasAnIncorrectLength(length))
		}

		let record = resource_data.value::<[u8; Eui48Size]>(0);

		resource_record_visitor.EUI48(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_eui64<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::EUI64, response_parsing_state)?;

		const Eui64Size: usize = 64 / ResourceRecord::BitsInAByte;

		let length = resource_data.len();
		if unlikely!(length != Eui64Size)
		{
			return Err(ResourceDataForTypeEUI64HasAnIncorrectLength(length))
		}

		let record = resource_data.value::<[u8; Eui64Size]>(0);

		resource_record_visitor.EUI64(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_uri<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::URI, response_parsing_state)?;

		const PrioritySize: usize = 2;
		const WeightSize: usize = 2;
		const MinimumTargetSize: usize = 1;

		let length = resource_data.len();
		if unlikely!(length < PrioritySize + WeightSize + MinimumTargetSize)
		{
			return Err(ResourceDataForTypeURIHasAnIncorrectLength(length))
		}

		let record = Uri
		{
			priority: resource_data.u16(0),
			weight: resource_data.u16(PrioritySize),
			target_uri: &resource_data[(PrioritySize + WeightSize) .. ],
		};

		resource_record_visitor.URI(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_caa<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: WithCompressionParsedName<'message>, resource_record_visitor: &mut impl ResourceRecordVisitor<'message>, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, DataType::CAA, response_parsing_state)?;

		use self::CertificateAuthorityAuthorizationResourceRecordIgnoredBecauseReason::*;

		const FlagsSize: usize = 1;
		const TagLengthSize: usize = 1;
		const MinimumTagSize: usize = 1;
		const MinimumValueSize: usize = 0;
		const PropertyTagOffset: usize = FlagsSize + TagLengthSize;

		let length = resource_data.len();
		if unlikely!(length < FlagsSize + TagLengthSize + MinimumTagSize + MinimumValueSize)
		{
			return Err(ResourceDataForTypeCAAHasAnIncorrectLength(length))
		}

		let tag_length = resource_data.u8_as_usize(FlagsSize);

		if unlikely!(tag_length == 0)
		{
			return Err(ResourceDataForTypeCAAHasAZeroTagLength)
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		if unlikely!(tag_length > 15)
		{
			resource_record_visitor.CAA_ignored(resource_record_name, TagLengthExceeded15(tag_length));
			return Ok(resource_data_end_pointer)
		}

		let property_value_offset = PropertyTagOffset + tag_length;

		if unlikely!(property_value_offset > length)
		{
			return Err(ResourceDataForTypeCAAHasAnIncorrectLength(length))
		}

		let flags = resource_data.u8(0);

		// // See <https://www.iana.org/assignments/pkix-parameters/pkix-parameters.xhtml>; note that bit 0 is MSB, ie bits are numbered from left-to-right.
		const IssuerCriticalFlagBit: u8 = 0b1000_0000;
		const ReservedFlagBits: u8 = !IssuerCriticalFlagBit;

		if unlikely!(flags & ReservedFlagBits != 0)
		{
			resource_record_visitor.CAA_ignored(resource_record_name, UseOfUnassignedFlagBits(flags));
			return Ok(resource_data_end_pointer)
		}

		static KnownTags: Map<&'static [u8], Option<CertificateAuthorityAuthorizationPropertyTag>> = phf_map!
		{
    		b"issue" => Some(AuthorizationEntryByDomain),
    		b"issuewild" => Some(AuthorizationEntryByWildcardDomain),
    		b"iodef" => Some(ReportIncidentByIodefReport),
    		b"contactemail" => Some(AuthorizedEMailContactForDomainValidation),
    		b"auth" => None,
    		b"path" => None,
    		b"policy" => None,
		};

		let property_tag_bytes = &resource_data[PropertyTagOffset .. property_value_offset];
		let property_tag = match KnownTags.get(property_tag_bytes)
		{
			Some(Some(property_tag)) => *property_tag,

			Some(None) =>
			{
				resource_record_visitor.CAA_ignored(resource_record_name, TagReservedByRfcErrata3547(property_tag_bytes));
				return Ok(resource_data_end_pointer)
			}

			None =>
			{
				resource_record_visitor.CAA_ignored(resource_record_name, TagUnassigned(property_tag_bytes));
				return Ok(resource_data_end_pointer)
			}
		};

		let record = CertificateAuthorityAuthorization
		{
			is_issuer_critical: flags & 0b0000_0001 != 0,
			property_tag,
			property_value: &resource_data[property_value_offset .. ],
		};

		resource_record_visitor.CAA(resource_record_name, time_to_live, record)?;

		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_tlsa_or_smimea<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, data_type: DataType, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<(usize, Either<(TimeToLiveInSeconds, DnsBasedAuthenticationOfNamedEntities<'message>), DnsBasedAuthenticationOfNamedEntitiesResourceRecordIgnoredBecauseReason>), DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, data_type, response_parsing_state)?;

		use self::DnsBasedAuthenticationOfNamedEntitiesResourceRecordIgnoredBecauseReason::*;

		const CertificateUsageSize: usize = 1;
		const SelectorSize: usize = 1;
		const MatchingTypeSize: usize = 1;
		const MinimumDigestSize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < CertificateUsageSize + SelectorSize + MatchingTypeSize + MinimumDigestSize)
		{
			return Err(ResourceDataForTypeTLSAOrSMIMEAHasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let raw_certificate_usage = resource_data.u8(0);
		let certificate_usage: CertificateUsage = match raw_certificate_usage
		{
			0 ..= 3 => unsafe { transmute(raw_certificate_usage) },

			4 ..= 254 => return Ok((resource_data_end_pointer, Right(UnassignedCertificateUsage(raw_certificate_usage)))),

			255 => return Ok((resource_data_end_pointer, Right(PrivateCertificateUsage))),
		};

		let raw_selector = resource_data.u8(CertificateUsageSize);
		let selector: Selector = match raw_selector
		{
			0 ..= 1 => unsafe { transmute(raw_selector) },

			2 ..= 254 => return Ok((resource_data_end_pointer, Right(UnassignedSelector(raw_selector)))),

			255 => return Ok((resource_data_end_pointer, Right(PrivateSelector))),
		};

		const DigestOffset: usize = CertificateUsageSize + SelectorSize + MatchingTypeSize;
		use self::MatchingType::*;
		let raw_matching_type = resource_data.u8(CertificateUsageSize + SelectorSize);
		let matching_type = match raw_matching_type
		{
			0 => NoHashUsed(&resource_data[DigestOffset .. ]),

			1 => guard_hash_digest_if_final_field!(resource_data, DigestOffset, 256, Sha2_256, ResourceDataForTypeTLSAOrSMIMEAHasADigestLengthThatIsIncorrectForTheMatchingType),

			2 => guard_hash_digest_if_final_field!(resource_data, DigestOffset, 512, Sha2_512, ResourceDataForTypeTLSAOrSMIMEAHasADigestLengthThatIsIncorrectForTheMatchingType),

			2 ..= 254 => return Ok((resource_data_end_pointer, Right(UnassignedMatchingType(raw_matching_type)))),

			255 => return Ok((resource_data_end_pointer, Right(PrivateMatchingType))),
		};

		Ok
		(
			(
				resource_data_end_pointer,
				Left
				(
					(
						time_to_live,
						DnsBasedAuthenticationOfNamedEntities
						{
							certificate_usage,
							selector,
							matching_type,
						}
					)
				)
			)
		)
	}

	#[inline(always)]
	fn parse_internet_protocol_address_only<'message, Address: Copy>(&mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, data_type: DataType, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<(TimeToLiveInSeconds, Address, usize), DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, data_type, response_parsing_state)?;

		let length = resource_data.len();
		if unlikely!(length != size_of::<Address>())
		{
			Err(ResourceDataForTypeAOrAAAAHasAnIncorrectLength(length))
		}
		else
		{
			let address = resource_data.value::<Address>(0);
			Ok((time_to_live, address, resource_data.end_pointer()))
		}
	}

	#[inline(always)]
	fn parse_name_only<'message>(&'message mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, parsed_labels: &mut ParsedLabels, data_type: DataType, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<(TimeToLiveInSeconds, WithCompressionParsedName<'message>, usize), DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer, data_type, response_parsing_state)?;

		let record = parsed_labels.parse_name_in_slice_with_nothing_left(resource_data)?;
		Ok((time_to_live, record, resource_data.end_pointer()))
	}

	#[inline(always)]
	fn validate_minimum_record_size_and_parse_name_and_resource_record_type<'message>(&'message mut self, end_of_message_pointer: usize, parsed_labels: &mut ParsedLabels) -> Result<(WithCompressionParsedName<'message>, usize, (u8, u8)), DnsProtocolError>
	{
		let start_of_resource_record_pointer = self.as_usize_pointer();
		if unlikely!(start_of_resource_record_pointer + Self::MinimumSize > end_of_message_pointer)
		{
			return Err(ResourceRecordIsShorterThanMinimumSize)
		}

		let (resource_record_name, end_of_resource_record_name_pointer) = self.name().parse_with_compression(parsed_labels, end_of_message_pointer)?;

		if unlikely!(end_of_resource_record_name_pointer + ResourceRecordFooter::MinimumSize > end_of_message_pointer)
		{
			return Err(ResourceRecordIsShorterThanMinimumSizeAfterParsingName)
		}

		let resource_record_type = self.resource_record_type(end_of_resource_record_name_pointer);

		Ok((resource_record_name, end_of_resource_record_name_pointer, resource_record_type.upper_and_lower()))
	}

	#[inline(always)]
	fn validate_class_is_internet_and_get_time_to_live_and_resource_data<'message>(&mut self, end_of_name_pointer: usize, end_of_message_pointer: usize, data_type: DataType, response_parsing_state: &mut ResponseParsingState<'message>) -> Result<(TimeToLiveInSeconds, &mut [u8]), DnsProtocolError>
	{
		let class = self.resource_record_class(end_of_name_pointer)?;
		debug_assert_eq!(class, ResourceRecordClass::Internet, "Why do we support classes other than Internet?");

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
		response_parsing_state.encountered(data_type, resource_record_name, resource_data)?;

		let time_to_live = self.time_to_live(end_of_name_pointer);

		Ok((time_to_live, resource_data))
	}

	#[inline(always)]
	fn safely_access_resource_data(&mut self, end_of_name_pointer: usize, end_of_message_pointer: usize) -> Result<&mut [u8], DnsProtocolError>
	{
		let resource_data_length = self.resource_data_length(end_of_name_pointer) as usize;
		if unlikely!(end_of_name_pointer + resource_data_length > end_of_message_pointer)
		{
			Err(ResourceDataLengthOverflows)
		}
		else
		{
			Ok(self.resource_data(end_of_name_pointer).unsafe_cast_slice_mut::<u8>(resource_data_length))
		}
	}

	/// `NAME` field.
	#[inline(always)]
	fn name(&mut self) -> &mut Name
	{
		self.unsafe_cast_mut::<Name>()
	}

	/// `TYPE` field.
	#[inline(always)]
	fn resource_record_type(&self, end_of_name_pointer: usize) -> DataType
	{
		self.footer(end_of_name_pointer).resource_record_type()
	}

	/// `CLASS` field.
	#[inline(always)]
	fn resource_record_class(&self, end_of_name_pointer: usize) -> Result<ResourceRecordClass, DnsProtocolError>
	{
		self.footer(end_of_name_pointer).resource_record_class()
	}

	/// `EDNS(0)` field.
	///
	/// RFC 6891.
	#[inline(always)]
	fn requestors_udp_payload_size(&self, end_of_name_pointer: usize) -> u16
	{
		self.footer(end_of_name_pointer).requestors_udp_payload_size()
	}

	/// `TTL` field.
	#[inline(always)]
	fn time_to_live(&self, end_of_name_pointer: usize) -> TimeToLiveInSeconds
	{
		self.footer(end_of_name_pointer).time_to_live()
	}

	/// `EDNS(0)` field.
	///
	/// RFC 6891.
	#[inline(always)]
	fn extended_response_code_and_flags(&self, end_of_name_pointer: usize) -> ExtendedResponseCodeAndFlags
	{
		self.footer(end_of_name_pointer).extended_response_code_and_flags()
	}

	/// `RDLEN` field.
	#[inline(always)]
	fn resource_data_length(&self, end_of_name_pointer: usize) -> u16
	{
		self.footer(end_of_name_pointer).resource_data_length()
	}

	/// `RDATA` field.
	#[inline(always)]
	fn resource_data(&mut self, end_of_name_pointer: usize) -> &mut ResourceData
	{
		self.footer_mut(end_of_name_pointer).resource_data()
	}

	#[inline(always)]
	fn footer(&self, end_of_name_pointer: usize) -> &ResourceRecordFooter
	{
		end_of_name_pointer.unsafe_cast::<ResourceRecordFooter>()
	}

	#[inline(always)]
	fn footer_mut(&self, end_of_name_pointer: usize) -> &mut ResourceRecordFooter
	{
		end_of_name_pointer.unsafe_cast_mut::<ResourceRecordFooter>()
	}
}

