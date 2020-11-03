// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct ResourceRecord;

impl ResourceRecord
{
	const MinimumSize: usize = ParsedNameParser::NameMinimumSize + ResourceRecordFooter::MinimumSize;

	const BitsInAByte: usize = 8;

	const MinimumCharacterStringSize: usize = 1;

	pub(crate) const ExtendedDns0OptRecordWithoutOptionsSize: usize = Self::MinimumSize;
	
	pub(crate) const UdpRequestorsPayloadSize: usize = (4096 - TcpDnsMessage::TcpBufferLengthSize);
	
	const UdpRequestorsPayloadSizeBigEndianU16: BigEndianU16 = (Self::UdpRequestorsPayloadSize as u16).to_be_bytes();

	#[inline(always)]
	pub(crate) fn write_extended_dns_0_opt_for_query(end_of_authority_section_pointer: usize) -> usize
	{
		let mut current_pointer = end_of_authority_section_pointer;
		
		const RootNameSize: usize = ParsedNameParser::NameMinimumSize;
		const RootName: u8 = 0x00;
		current_pointer.set_u8_byte(RootName);
		current_pointer += 1;
		
		const ExtendsDnsRequestorsUdpPayloadSize: usize = ResourceRecordFooter::ClassSize;
		current_pointer.set_u16_bytes(MetaType::OPT.0);
		current_pointer += ResourceRecordFooter::TypeSize;
		
		current_pointer.set_u16_bytes(Self::UdpRequestorsPayloadSizeBigEndianU16);
		current_pointer += ExtendsDnsRequestorsUdpPayloadSize;
		
		const ExtendedDnsExtendedRCodeAndFlagsSize: usize = ResourceRecordFooter::TimeToLiveSize;
		current_pointer.set_u32_bytes(ExtendedResponseCodeAndFlags::new_for_query());
		current_pointer += ExtendedDnsExtendedRCodeAndFlagsSize;
		
		const ExtendedDnsOptionsSize: usize = ResourceRecordFooter::ResourceDataLengthSize;
		const NoOptions: BigEndianU16 = [0, 0];
		current_pointer.set_u16_bytes(NoOptions);
		current_pointer += ExtendedDnsOptionsSize;
		
		current_pointer
	}

	#[inline(always)]
	pub(crate) fn parse_answer_section_resource_record_in_response<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, question_q_type: DataType, end_of_message_pointer: usize, parsed_names: &mut ParsedNames<'message>, resource_record_visitor: &mut RRV, response_parsing_state: &ResponseParsingState, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, answer_section_has_at_least_one_record_of_requested_data_type: &mut bool, query_name: &EfficientCaseFoldedName) -> Result<usize, AnswerSectionError<RRV::Error>>
	{
		use self::HandleRecordTypeError::ResourceTypeInWrongSection;
		use self::ResourceTypeInWrongSectionError::ResourceRecordTypeIsNotValidInAnswerSectionIfNotRequestedByQuery;
		
		let (owner_name, end_of_name_pointer, (resource_record_type_higher, resource_record_type_lower)) = self.validate_minimum_record_size_and_parse_name_and_resource_record_type(end_of_message_pointer, parsed_names)?;
		
		let resource_record_type_matches_query_type =
		{
			let (question_q_type_higher, question_q_type_lower) = question_q_type.upper_and_lower();
			question_q_type_higher == resource_record_type_higher && question_q_type_lower == resource_record_type_lower
		};
		
		if likely!(resource_record_type_matches_query_type)
		{
			if owner_name.ne(query_name)
			{
				return Err(AnswerSectionError::ResourceRecordOwnerNameThatDiffersToQueryNameButMatchesQueryType(EfficientCaseFoldedName::from(owner_name)))
			}
			*answer_section_has_at_least_one_record_of_requested_data_type = true;
			
			// This is a list of record types, which, when asked for in a query, are only allowed to occur once or never in the answer section.
			match (resource_record_type_higher, resource_record_type_lower)
			{
				(DataType::CNAME_higher, DataType::CNAME_lower) =>
				{
					response_parsing_state.validate_only_one_CNAME_record_in_answer_section_when_query_type_was_CNAME::<RRV::Error>()?;
					self.handle_CNAME(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing, Some(true))
				}
				
				(DataType::DNAME_higher, DataType::DNAME_lower) =>
				{
					response_parsing_state.validate_only_one_DNAME_record_in_answer_section_when_query_type_was_DNAME::<RRV::Error>()?;
					self.handle_DNAME(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing, Some(true))
				}
				
				_ => self.dispatch_resource_record_type(now, end_of_name_pointer, end_of_message_pointer, owner_name, parsed_names, resource_record_visitor, response_parsing_state, duplicate_resource_record_response_parsing, true, (resource_record_type_higher, resource_record_type_lower), Some(false))
			}
		}
		else
		{
			// This is a list of record types, which, although not asked for in a query, are permitted to be in the answer section.
			match (resource_record_type_higher, resource_record_type_lower)
			{
				(DataType::CNAME_higher, DataType::CNAME_lower) => self.handle_CNAME(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing, Some(false)),
				
				(DataType::DNAME_higher, DataType::DNAME_lower) => self.handle_DNAME(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing, Some(false)),
				
				(DataType::RRSIG_higher, DataType::RRSIG_lower) => self.handle_RRSIG(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing, Some(false)),
				
				_ => Err(ResourceTypeInWrongSection(ResourceRecordTypeIsNotValidInAnswerSectionIfNotRequestedByQuery(DataType([resource_record_type_higher, resource_record_type_lower])))),
			}
		}.map_err(AnswerSectionError::HandleRecordType)
	}

	/// Returns `Ok(end_of_resource_data_pointer)` unless there is an error.
	#[inline(always)]
	pub(crate) fn parse_authority_section_resource_record_in_response<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_message_pointer: usize, parsed_names: &mut ParsedNames<'message>, resource_record_visitor: &mut RRV, response_parsing_state: &ResponseParsingState, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, AuthoritySectionError<RRV::Error>>
	{
		use self::HandleRecordTypeError::ResourceTypeInWrongSection;
		use self::ResourceTypeInWrongSectionError::ResourceRecordTypeIsNotValidInAuthoritySection;
		
		let (owner_name, end_of_name_pointer, (type_upper, type_lower)) = self.validate_minimum_record_size_and_parse_name_and_resource_record_type(end_of_message_pointer, parsed_names)?;

		// List of type codes based on https://serverfault.com/questions/899714/can-authority-rrs-contain-anything-but-ns-and-soa-additional-rrs-anything-but-a .
		if likely!(type_upper == 0x00)
		{
			match type_lower
			{
				// Referral.
				DataType::NS_lower => self.handle_NS(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing),

				// Negative Response.
				DataType::SOA_lower => self.handle_SOA(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, response_parsing_state, duplicate_resource_record_response_parsing),

				// Referral.
				DataType::DS_lower => self.handle_DS(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				// Signing negative response or referral.
				DataType::RRSIG_lower => self.handle_RRSIG(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing, None),

				// Signing negative response.
				DataType::NSEC_lower => self.handle_NSEC(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing),

				// Signing negative response.
				DataType::NSEC3_lower => self.handle_NSEC3(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				_ => Err(ResourceTypeInWrongSection(ResourceRecordTypeIsNotValidInAuthoritySection(DataType([type_upper, type_lower])))),
			}
		}
		else
		{
			Err(ResourceTypeInWrongSection(ResourceRecordTypeIsNotValidInAuthoritySection(DataType([type_upper, type_lower]))))
		}.map_err(AuthoritySectionError::HandleRecordType)
	}

	/// Returns `Ok(end_of_resource_data_pointer)` unless there is an error.
	#[inline(always)]
	pub(crate) fn parse_additional_section_resource_record_in_response<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_message_pointer: usize, parsed_names: &mut ParsedNames<'message>, resource_record_visitor: &mut RRV, response_parsing_state: &ResponseParsingState, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, authoritative_or_authenticated_or_neither: AuthoritativeOrAuthenticatedOrNeither, rcode_lower_4_bits: RCodeLower4Bits) -> Result<usize, AdditionalSectionError<RRV::Error>>
	{
		let (parsed_name_iterator, end_of_name_pointer, (type_upper, type_lower)) = self.validate_minimum_record_size_and_parse_name_and_resource_record_type(end_of_message_pointer, parsed_names)?;
		
		let end_of_resource_data_pointer = if type_upper == 0x00 && type_lower == MetaType::OPT_lower
		{
			self.handle_OPT(now, end_of_name_pointer, end_of_message_pointer, response_parsing_state, authoritative_or_authenticated_or_neither, rcode_lower_4_bits)?
		}
		else
		{
			self.dispatch_resource_record_type(now, end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, parsed_names, resource_record_visitor, response_parsing_state, duplicate_resource_record_response_parsing, false, (type_upper, type_lower), None)?
		};
		Ok(end_of_resource_data_pointer)
	}
	
	#[inline(always)]
	fn dispatch_resource_record_type<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, parsed_names: &mut ParsedNames<'message>, resource_record_visitor: &mut RRV, response_parsing_state: &ResponseParsingState, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, soa_is_permitted: bool, (type_upper, type_lower): (u8, u8), is_some_if_present_in_answer_section_and_true_if_was_queried_for: Option<bool>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::HandleRecordTypeError::*;
		use self::ResourceTypeInWrongSectionError::*;
		use self::QueryTypeOutsideOfAQuestionSectionEntryError::*;
		
		match type_upper
		{
			0x00 => match type_lower
			{
				MetaType::SIG0_lower => self.handle_obsolete_meta_type(end_of_name_pointer, end_of_message_pointer, owner_name, duplicate_resource_record_response_parsing, MetaType::SIG0, "Only really useful for updates, which, frankly, are probably better done out-of-band than using DNS; regardless, when using DNS over TLS a client certificate is much more useful"),

				DataType::A_lower => self.handle_A(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::NS_lower => self.handle_NS(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing),

				DataType::MD_lower => self.handle_very_obsolete_record_type(DataType::MD),

				DataType::MF_lower => self.handle_very_obsolete_record_type(DataType::MF),

				DataType::CNAME_lower => self.handle_CNAME(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing, is_some_if_present_in_answer_section_and_true_if_was_queried_for),

				DataType::SOA_lower => if likely!(soa_is_permitted)
				{
					self.handle_SOA(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, response_parsing_state, duplicate_resource_record_response_parsing)
				}
				else
				{
					Err(ResourceTypeInWrongSection(StartOfAuthorityResourceRecordTypeIsNotPermittedInThisSection))
				},

				DataType::MB_lower => self.handle_very_obsolete_record_type(DataType::MB),

				DataType::MG_lower => self.handle_very_obsolete_record_type(DataType::MG),

				DataType::MR_lower => self.handle_very_obsolete_record_type(DataType::MR),

				DataType::NULL_lower => self.handle_very_obsolete_record_type(DataType::NULL),

				DataType::WKS_lower => self.handle_very_obsolete_record_type(DataType::WKS),

				DataType::PTR_lower => self.handle_PTR(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing),

				DataType::HINFO_lower => self.handle_HINFO(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::MINFO_lower => self.handle_very_obsolete_record_type(DataType::MINFO),

				DataType::MX_lower => self.handle_MX(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing),

				DataType::TXT_lower => self.handle_TXT(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::RP_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, owner_name, duplicate_resource_record_response_parsing, DataType::RP, "Used in some rare circumstances; some legacy records may remain"),

				DataType::AFSDB_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, owner_name, duplicate_resource_record_response_parsing, DataType::AFSDB, "Replaced by use of SRV records; some legacy records may remain"),

				DataType::X25_lower => self.handle_very_obsolete_record_type(DataType::X25),

				DataType::ISDN_lower => self.handle_very_obsolete_record_type(DataType::ISDN),

				DataType::RT_lower => self.handle_very_obsolete_record_type(DataType::RT),

				DataType::NSAP_lower => self.handle_very_obsolete_record_type(DataType::NSAP),

				DataType::NSAP_PTR_lower => self.handle_very_obsolete_record_type(DataType::NSAP_PTR),

				DataType::SIG_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, owner_name, duplicate_resource_record_response_parsing, DataType::SIG, "Not used now SIG(0) is available; some legacy records may remain"),

				DataType::KEY_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, owner_name, duplicate_resource_record_response_parsing, DataType::KEY, "Replaced by IPSECKEY and various DNSSEC records; some legacy records may remain"),

				DataType::PX_lower => self.handle_very_obsolete_record_type(DataType::PX),

				DataType::GPOS_lower => self.handle_very_obsolete_record_type(DataType::GPOS),

				DataType::AAAA_lower => self.handle_AAAA(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::LOC_lower => self.handle_LOC(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::NXT_lower => self.handle_very_obsolete_record_type(DataType::NXT),

				DataType::EID_lower => self.handle_very_obsolete_record_type(DataType::EID),

				DataType::NIMLOC_lower => self.handle_very_obsolete_record_type(DataType::NIMLOC),

				DataType::SRV_lower => self.handle_SRV(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing),

				DataType::ATMA_lower => self.handle_very_obsolete_record_type(DataType::ATMA),

				DataType::NAPTR_lower => self.handle_NAPTR(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing),

				DataType::KX_lower => self.handle_KX(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing),

				DataType::CERT_lower => self.handle_cert(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::A6_lower => self.handle_very_obsolete_record_type(DataType::A6),

				DataType::DNAME_lower => self.handle_DNAME(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing, is_some_if_present_in_answer_section_and_true_if_was_queried_for),

				DataType::SINK_lower => self.handle_very_obsolete_record_type(DataType::SINK),

				MetaType::OPT_lower => Err(ResourceTypeInWrongSection(ExtendedDnsOptResourceRecordTypeIsNotPermittedOutsideOfAnAdditionalSection)),

				DataType::APL_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, owner_name, duplicate_resource_record_response_parsing, DataType::APL, "Some legacy records may remain"),

				DataType::DS_lower => self.handle_DS(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::SSHFP_lower => self.handle_SSHFP(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::IPSECKEY_lower => self.handle_IPSECKEY(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing),

				DataType::NSEC_lower => self.handle_NSEC(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing),

				DataType::RRSIG_lower => self.handle_RRSIG(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing, is_some_if_present_in_answer_section_and_true_if_was_queried_for),

				DataType::DNSKEY_lower => self.handle_DNSKEY(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::DHCID_lower => self.handle_DHCID(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::NSEC3_lower => self.handle_NSEC3(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::NSEC3PARAM_lower => self.handle_NSEC3PARAM(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::TLSA_lower => self.handle_TLSA(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::SMIMEA_lower => self.handle_SMIMEA(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				54 => self.handle_unassigned(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing, 0x00, 54),

				DataType::HIP_lower => self.handle_HIP(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing),

				DataType::NINFO_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, owner_name, duplicate_resource_record_response_parsing, DataType::NINFO, "No RFC or RFC draft and probably not deployed"),

				DataType::RKEY_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, owner_name, duplicate_resource_record_response_parsing, DataType::RKEY, "No RFC or RFC draft and probably not deployed"),

				DataType::TALINK_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, owner_name, duplicate_resource_record_response_parsing, DataType::TALINK, "No RFC or RFC draft and probably not deployed"),

				DataType::CDS_lower => self.handle_CDS(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::CDNSKEY_lower => self.handle_CDNSKEY(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::OPENPGPKEY_lower => self.handle_OPENPGPKEY(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::CSYNC_lower => self.handle_CSYNC(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::ZONEMD_lower => self.handle_possible_future_standard(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing, DataType::ZONEMD),

				64 ..= 98 => self.handle_unassigned(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing, 0x00, type_lower),

				DataType::SPF_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, owner_name, duplicate_resource_record_response_parsing, DataType::SPF, "RFC 7208 deprecated this record type; some legacy records may remain"),

				DataType::UINFO_lower => self.handle_very_obsolete_record_type(DataType::UINFO),

				DataType::UID_lower => self.handle_very_obsolete_record_type(DataType::UID),

				DataType::GID_lower => self.handle_very_obsolete_record_type(DataType::GID),

				DataType::UNSPEC_lower => self.handle_very_obsolete_record_type(DataType::UNSPEC),

				DataType::NID_lower => self.handle_NID(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::L32_lower => self.handle_L32(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::L64_lower => self.handle_L64(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::LP_lower => self.handle_LP(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, parsed_names, duplicate_resource_record_response_parsing),

				DataType::EUI48_lower => self.handle_EUI48(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::EUI64_lower => self.handle_EUI64(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				110 ..= 127 => self.handle_unassigned(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing, 0x00, type_lower),

				128 ..= 248 => Err(UnknownQueryTypeOrMetaType(0x00, type_lower)),

				MetaType::TKEY_lower => self.handle_obsolete_meta_type(end_of_name_pointer, end_of_message_pointer, owner_name, duplicate_resource_record_response_parsing, MetaType::TKEY, "Only really useful for updates, which, frankly, are probably better done out-of-band than using DNS; regardless, when using DNS over TLS a client certificate is much more useful"),

				MetaType::TSIG_lower => self.handle_obsolete_meta_type(end_of_name_pointer, end_of_message_pointer, owner_name, duplicate_resource_record_response_parsing, MetaType::TSIG, "Only really useful for updates, which, frankly, are probably better done out-of-band than using DNS; regardless, when using DNS over TLS a client certificate is much more useful"),

				QueryType::IXFR_lower => Err(QueryTypeOutsideOfAQuestionSectionEntry(IXFR)),

				QueryType::AXFR_lower => Err(QueryTypeOutsideOfAQuestionSectionEntry(AXFR)),

				QueryType::MAILB_lower => Err(QueryTypeOutsideOfAQuestionSectionEntry(MAILB)),

				QueryType::MAILA_lower => Err(QueryTypeOutsideOfAQuestionSectionEntry(MAILA)),

				QueryType::Asterisk_lower => Err(QueryTypeOutsideOfAQuestionSectionEntry(Asterisk)),
			},

			0x01 => match type_lower
			{
				DataType::URI_lower => self.handle_uri(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::CAA_lower => self.handle_CAA(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing),

				DataType::DOA_lower => self.handle_possible_future_standard(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing, DataType::DOA),

				DataType::AMTRELAY_lower => self.handle_possible_future_standard(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing, DataType::AMTRELAY),
				
				_ => self.handle_unassigned(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing, 0x01, type_lower),
			},

			0x02 ..= 0x7F => self.handle_unassigned(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing, type_upper, type_lower),

			0x80 => match type_lower
			{
				DataType::TA_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, owner_name, duplicate_resource_record_response_parsing, DataType::TA, "DNSSEC Trust Anchors were never widely deployed; some legacy records may remain"),

				DataType::DLV_lower => self.handle_obsolete_or_very_obscure_record_type(end_of_name_pointer, end_of_message_pointer, owner_name, duplicate_resource_record_response_parsing, DataType::DLV, "DNSSEC Lookaside Validation is not longer supported now that all root nameservers support DNSSEC; some legacy records may remain"),

				_ => self.handle_unassigned(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing, 0x80, type_lower),
			},

			0x81 ..= 0xEF => self.handle_unassigned(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, duplicate_resource_record_response_parsing,  type_upper, type_lower),

			_ => Err(ReservedRecordType(type_upper, type_lower))
		}
	}

	/// Record types that died a very long time ago.
	#[inline(always)]
	fn handle_very_obsolete_record_type<'message, E: error::Error>(&'message self, data_type: DataType) -> Result<usize, HandleRecordTypeError<E>>
	{
		Err(HandleRecordTypeError::VeryObsoleteResourceRecordType(data_type))
	}

	/// Record types that died, never became popular or widespread or never proceeded even to a RFC draft.
	#[inline(always)]
	fn handle_obsolete_or_very_obscure_record_type<'message, E: error::Error>(&'message self, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, data_type: DataType, _reason: &'static str) -> Result<usize, HandleRecordTypeError<E>>
	{
		let (_time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(&owner_name, end_of_name_pointer, end_of_message_pointer, data_type, duplicate_resource_record_response_parsing)?;

		Ok(resource_data.end_pointer())
	}

	/// Meta types, that, with the coming of DNS over TLS, are obsolete.
	#[inline(always)]
	fn handle_obsolete_meta_type<'message, E: error::Error>(&'message self, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, meta_type: MetaType, _reason: &'static str) -> Result<usize, HandleRecordTypeError<E>>
	{
		let (_time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(&owner_name, end_of_name_pointer, end_of_message_pointer, meta_type, duplicate_resource_record_response_parsing)?;

		Ok(resource_data.end_pointer())
	}

	/// Data types that are draft RFCs or similar and may need to be supported by clients of this library.
	#[inline(always)]
	fn handle_possible_future_standard<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, data_type: DataType) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, data_type, duplicate_resource_record_response_parsing)?;

		resource_record_visitor.handle_possible_future_standard(owner_name, cache_until, resource_data, data_type).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(data_type, error))?;
		Ok(resource_data.end_pointer())
	}

	/// Data types that aren't officially registered with IANA.
	#[inline(always)]
	fn handle_unassigned<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, type_upper: u8, type_lower: u8) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		let data_type = DataType([type_upper, type_lower]);
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, data_type, duplicate_resource_record_response_parsing)?;

		resource_record_visitor.unassigned(owner_name, cache_until, resource_data, DataType([type_upper, type_lower])).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(data_type, error))?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_A<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::HandleRecordTypeError::*;
		
		let (cache_until, record, end_of_resource_data_pointer) = self.parse_internet_protocol_address_only(now, end_of_name_pointer, end_of_message_pointer, &owner_name, DataType::A, duplicate_resource_record_response_parsing, AHasAnIncorrectLength)?;
		resource_record_visitor.A(owner_name, cache_until, record).map_err(|error| ResourceRecordVisitor(DataType::A, error))?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_NS<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, parsed_names: &mut ParsedNames<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::HandleRecordTypeError::*;
		
		let (cache_until, record, end_of_resource_data_pointer) = self.parse_name_only(now, end_of_name_pointer, end_of_message_pointer, &owner_name, parsed_names, DataType::NS, duplicate_resource_record_response_parsing, NS)?;
		resource_record_visitor.NS(owner_name, cache_until, record).map_err(|error| ResourceRecordVisitor(DataType::NS, error))?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_CNAME<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, parsed_names: &mut ParsedNames<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, is_some_if_present_in_answer_section_and_true_if_was_queried_for: Option<bool>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::HandleRecordTypeError::*;
		
		let (cache_until, record, end_of_resource_data_pointer) = self.parse_name_only(now, end_of_name_pointer, end_of_message_pointer, &owner_name, parsed_names, DataType::CNAME, duplicate_resource_record_response_parsing, CNAME)?;
		resource_record_visitor.CNAME(owner_name, cache_until, record, is_some_if_present_in_answer_section_and_true_if_was_queried_for).map_err(|error| ResourceRecordVisitor(DataType::CNAME, error))?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_SOA<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, parsed_names: &mut ParsedNames<'message>, response_parsing_state: &ResponseParsingState, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::SOAHandleRecordTypeError::*;
		
		response_parsing_state.parsing_a_soa_record()?;

		let (resource_record_time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(&owner_name, end_of_name_pointer, end_of_message_pointer, DataType::SOA, duplicate_resource_record_response_parsing)?;

		let start_of_resource_data = resource_data.start_pointer();

		let (primary_name_server, end_of_mname_pointer) = ParsedNameParser::parse_name_in_slice(DataType::SOA, parsed_names, resource_data).map_err(ParseStartOfAuthorityMName)?;
		
		let (responsible_person_email_address, end_of_rname_pointer) = ParsedNameParser::parse_name_in_slice(DataType::SOA, parsed_names, &resource_data[(end_of_mname_pointer - start_of_resource_data) .. ]).map_err(ParseStartOfAuthorityRName)?;

		if unlikely!(responsible_person_email_address.is_root())
		{
			Err(ResponsiblePersonMailBoxIsRoot)?
		}
		
		if unlikely!(responsible_person_email_address.is_top_level())
		{
			Err(ResponsiblePersonMailBoxIsTopLevelDomain)?
		}
		
		let end_of_resource_data = start_of_resource_data + resource_data.len();
		if likely!((end_of_resource_data - end_of_rname_pointer) == size_of::<StartOfAuthorityFooter>())
		{
			let footer: &StartOfAuthorityFooter = end_of_rname_pointer.unsafe_cast::<StartOfAuthorityFooter>();
			
			let negative_cache_until =
			{
				// NOTE: We are supposed to only use `negative_caching_time_to_live`.
				// However, if the SOA record itself lives for less than `negative_caching_time_to_live`, then a future update to the SOA record may have changed the value of `negative_caching_time_to_live` such that it would expire before the current `negative_caching_time_to_live`.
				// For example:-
				//   SOA TTL = 1s, SOA MINIMUM = 5s;
				//   After 1s, a new SOA is published with SOA MINIMUM = 2s;
				//   If we used the original SOA MINIMUM = 5s, we would wait 4s before obtaining this, whereas our cached value was invalid after 3s.
				// Hence, using the lower of the two values allows for this change to be effective.
				// There is a slight cost of more frequent querying.
				let negative_caching_time_to_live = min(resource_record_time_to_live, footer.negative_caching_time_to_live);
				negative_caching_time_to_live.cache_until(now)
			};
			
			let record = StartOfAuthority
			{
				primary_name_server,
				responsible_person_email_address,
				zone_file_serial_number: footer.serial,
				referesh_interval: footer.refresh_interval.into(),
				retry_interval: footer.retry_interval.into(),
				expire_interval: footer.expire_interval.into(),
				marker: PhantomData,
			};
			
			resource_record_visitor.SOA(owner_name, negative_cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::SOA, error))?;

			Ok(resource_data.end_pointer())
		}
		else
		{
			Err(StartOfAuthorityIsIncorrectSizeAfterParsingMNameAndRName)?
		}
	}

	#[inline(always)]
	fn handle_PTR<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, parsed_names: &mut ParsedNames<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::HandleRecordTypeError::*;
		
		let (cache_until, record, end_of_resource_data_pointer) = self.parse_name_only(now, end_of_name_pointer, end_of_message_pointer, &owner_name, parsed_names, DataType::PTR, duplicate_resource_record_response_parsing, PTR)?;
		resource_record_visitor.PTR(owner_name, cache_until, record).map_err(|error| ResourceRecordVisitor(DataType::PTR, error))?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_HINFO<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::HINFOHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::HINFO, duplicate_resource_record_response_parsing)?;

		let length = resource_data.len();

		const MinimumCpuSize: usize = ResourceRecord::MinimumCharacterStringSize;
		const MinimumOsSize: usize = ResourceRecord::MinimumCharacterStringSize;

		if unlikely!(length < (MinimumCpuSize + MinimumOsSize))
		{
			Err(HasTooShortALength(length))?
		}

		let mut character_strings_iterator = CharacterStringsIterator::new(resource_data).map_err(CharacterStrings)?;

		let cpu = character_strings_iterator.next().ok_or(CpuDataMissing(length))?.map_err(CpuDataCharacterString)?;

		let os = character_strings_iterator.next().ok_or(OsDataMissing(length))?.map_err(OsDataCharacterString)?;

		if likely!(character_strings_iterator.is_empty())
		{
			let record = HostInformation
			{
				cpu,
				os,
			};

			resource_record_visitor.HINFO(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::HINFO, error))?;
			Ok(resource_data.end_pointer())
		}
		else
		{
			Err(WouldHaveUnusuedDataRemaining)?
		}
	}

	#[inline(always)]
	fn handle_MX<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, parsed_names: &mut ParsedNames<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::MXHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::MX, duplicate_resource_record_response_parsing)?;

		const PreferenceSize: usize = 2;
		const MinimumMailServerNameSize: usize = ParsedNameParser::NameMinimumSize;

		let length = resource_data.len();

		if unlikely!(length < PreferenceSize + MinimumMailServerNameSize)
		{
			Err(HasTooShortALength(length))?
		}

		let record = MailExchange
		{
			preference: Priority(resource_data.u16(0)),
			mail_server_name: ParsedNameParser::parse_name_in_slice_with_nothing_left(DataType::MX, parsed_names, &resource_data[PreferenceSize .. ]).map_err(MailServerName)?,
		};

		resource_record_visitor.MX(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::MX, error))?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_TXT<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::TXTHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::TXT, duplicate_resource_record_response_parsing)?;

		let mut character_strings = Vec::with_capacity(8);
		for (character_string_index, character_string) in CharacterStringsIterator::new(resource_data).map_err(CharacterStrings)?.enumerate()
		{
			let character_string = character_string.map_err(|error| CharacterStringLengthIncorrect(character_string_index, error))?;
			character_strings.push(character_string)
		}
		
		resource_record_visitor.TXT(owner_name, cache_until, character_strings).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::TXT, error))?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_AAAA<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::HandleRecordTypeError::*;
		
		let (cache_until, record, end_of_resource_data_pointer) = self.parse_internet_protocol_address_only(now, end_of_name_pointer, end_of_message_pointer, &owner_name, DataType::AAAA, duplicate_resource_record_response_parsing, AAAAHasAnIncorrectLength)?;
		resource_record_visitor.AAAA(owner_name, cache_until, record).map_err(|error| ResourceRecordVisitor(DataType::AAAA, error))?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_LOC<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::LOCHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::LOC, duplicate_resource_record_response_parsing)?;

		let length = resource_data.len();
		if unlikely!(length == 0)
		{
			Err(CanNotBeEmpty)?
		}
		
		const SizeOfVersion: usize = 1;
		
		let location_version = resource_data.u8(0);
		if unlikely!(location_version != 0)
		{
			Err(InvalidVersion(location_version))?
		}
		
		if unlikely!(length != SizeOfVersion + size_of::<LocationBodyVersion0>())
		{
			Err(HasAnIncorrectLengthForVersion0(length))?
		}
		
		let location_body_version_0 = resource_data.cast::<LocationBodyVersion0>(SizeOfVersion);
		
		resource_record_visitor.LOC_version_0(owner_name, cache_until, location_body_version_0).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::LOC, error))?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_SRV<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, parsed_names: &mut ParsedNames<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::SRVHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::SRV, duplicate_resource_record_response_parsing)?;

		const PrioritySize: usize = 2;
		const WeightSize: usize = 2;
		const PortSize: usize = 2;
		const MinimumTargetNameSize: usize = ParsedNameParser::NameMinimumSize;

		let length = resource_data.len();
		if unlikely!(length < PrioritySize + WeightSize + PortSize + MinimumTargetNameSize)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let record = ServiceLocation
		{
			priority: Priority(resource_data.u16(0)),
			weight: Weight(resource_data.u16(PrioritySize)),
			port: resource_data.u16(PrioritySize + WeightSize),
			target: ParsedNameParser::parse_name_in_slice_with_nothing_left(DataType::SRV, parsed_names, &resource_data[(PrioritySize + WeightSize + PortSize) .. ]).map_err(ServiceName)?,
		};

		resource_record_visitor.SRV(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::SRV, error))?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_NAPTR<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, parsed_names: &mut ParsedNames<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::NAPTRHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::NAPTR, duplicate_resource_record_response_parsing)?;

		const OrderSize: usize = 2;
		const PreferenceSize: usize = 2;
		const MinimumFlagsSize: usize = ResourceRecord::MinimumCharacterStringSize;
		const MinimumServicesSize: usize = ResourceRecord::MinimumCharacterStringSize;
		const MinimumRegularExpressionSize: usize = ResourceRecord::MinimumCharacterStringSize;
		const MinimumDomainNameSize: usize = ParsedNameParser::NameMinimumSize;

		let length = resource_data.len();
		if unlikely!(length < OrderSize + PreferenceSize + MinimumFlagsSize + MinimumServicesSize + MinimumRegularExpressionSize + MinimumDomainNameSize)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let order = resource_data.u16(0);
		let preference = resource_data.u16(OrderSize);

		let mut character_strings_iterator = CharacterStringsIterator::new(&resource_data[(OrderSize + PreferenceSize) .. ]).map_err(CharacterStrings)?;

		let flags = character_strings_iterator.next().ok_or(IsMissingFlags)?.map_err(FlagsCharacterString)?;

		let services = character_strings_iterator.next().ok_or(IsMissingServices)?.map_err(ServicesCharacterString)?;

		let regular_expression = character_strings_iterator.next().ok_or(IsMissingRegularExpression)?.map_err(RegularExpressionCharacterString)?;

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

		let result = if regular_expression.is_empty()
		{
			let (domain_name, end_of_name_pointer) = ParsedNameParser::parse_name_uncompressed(parsed_names, start_of_name_pointer, resource_data_end_pointer, DataType::NAPTR).map_err(DomainName)?;
			if unlikely!(end_of_name_pointer != resource_data_end_pointer)
			{
				Err(HasDataLeftOver)?
			}

			let record = NamingAuthorityPointerWithDomainName
			{
				header,
				domain_name
			};

			resource_record_visitor.NAPTR_domain_name(owner_name, cache_until, record)
		}
		else
		{
			let end_of_name_pointer = start_of_name_pointer + 1;

			if unlikely!(end_of_name_pointer != resource_data_end_pointer)
			{
				Err(HasDataLeftOver)?
			}

			let domain_name_byte = start_of_name_pointer.dereference_u8();
			if unlikely!(domain_name_byte != 0)
			{
				Err(HasBothARegularExpressionAndADomainName)?
			}

			let record = NamingAuthorityPointerWithRegularExpression
			{
				header,
				regular_expression
			};

			resource_record_visitor.NAPTR_regular_expression(owner_name, cache_until, record)
		};
		
		result.map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::NAPTR, error))?;
		
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_KX<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, parsed_names: &mut ParsedNames<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::KXHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::KX, duplicate_resource_record_response_parsing)?;

		let length = resource_data.len();

		const PreferenceSize: usize = 2;
		const MinimumKeyExchangeServerNameSize: usize = ParsedNameParser::NameMinimumSize;

		if unlikely!(length < PreferenceSize + MinimumKeyExchangeServerNameSize)
		{
			Err(HasTooShortALength(length))?
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let (key_exchange_server_name, end_of_key_exchange_server_name) = ParsedNameParser::parse_name_uncompressed(parsed_names, resource_data.start_pointer() + PreferenceSize, resource_data_end_pointer, DataType::KX).map_err(KeyExchangeServerName)?;

		if unlikely!(end_of_key_exchange_server_name != resource_data_end_pointer)
		{
			Err(DataRemainingAfterKeyExchangeServerName)?
		}

		let record = KeyExchange
		{
			preference: resource_data.u16(0),
			key_exchange_server_name,
		};

		resource_record_visitor.KX(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::KX, error))?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_cert<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::CERTHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::CERT, duplicate_resource_record_response_parsing)?;

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
			Err(HasTooShortALength(length))?
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let certificate_type_value_upper = resource_data.u8(0);
		let certificate_type_value_lower = resource_data.u8(1);
		let certificate_type = match certificate_type_value_upper
		{
			0x00 => match certificate_type_value_lower
			{
				0 => Err(UsesAReservedCertificateTypeValue(0))?,

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
					resource_record_visitor.CERT_ignored(owner_name, CertificateTypeUnassigned(certificate_type_value_lower as u16));
					return Ok(resource_data_end_pointer)
				}

				253 =>
				{
					resource_record_visitor.CERT_ignored(owner_name, CertificateTypeUriPrivate);
					return Ok(resource_data_end_pointer)
				}

				254 =>
				{
					resource_record_visitor.CERT_ignored(owner_name, CertificateTypeOidPrivate);
					return Ok(resource_data_end_pointer)
				}

				255 => Err(UsesAReservedCertificateTypeValue(255))?,
			},

			0x01 ..= 0xFE =>
			{
				resource_record_visitor.CERT_ignored(owner_name, CertificateTypeUnassigned((certificate_type_value_upper as u16) << 8 | (certificate_type_value_lower as u16)));
				return Ok(resource_data_end_pointer)
			}

			0xFF => if unlikely!(certificate_type_value_lower == 0xFF)
			{
				Err(UsesAReservedCertificateTypeValue(0xFF << 8 | (certificate_type_value_lower as u16)))?
			}
			else
			{
				Err(UsesAnExperimentalCertificateTypeValue(0xFF << 8 | (certificate_type_value_lower as u16)))?
			},
		};

		let security_algorithm_type = resource_data.u8(CertificateTypeSize + KeyTagSize);
		let security_algorithm = match SecurityAlgorithm::parse_security_algorithm(security_algorithm_type, false, false, DataType::CERT).map_err(SecurityAlgorithmFailed)?
		{
			Left(security_algorithm) => security_algorithm,

			Right(security_algorithm_rejected_because_reason) =>
			{
				resource_record_visitor.CERT_ignored(owner_name, SecurityAlgorithmRejected(security_algorithm_rejected_because_reason));
				return Ok(resource_data_end_pointer)
			}
		};

		let record = Certificate
		{
			key_tag: resource_data.value::<KeyTag>(CertificateTypeSize),
			security_algorithm,
			certificate_type,
		};

		resource_record_visitor.CERT(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::CERT, error))?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_OPT<'message>(&self, _now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, response_parsing_state: &ResponseParsingState, authoritative_or_authenticated_or_neither: AuthoritativeOrAuthenticatedOrNeither, rcode_lower_4_bits: RCodeLower4Bits) -> Result<usize, ExtendedDnsError>
	{
		use self::ExtendedDnsError::*;
		
		let start_of_name_pointer = self.start_of_name_pointer();
		if unlikely!(end_of_name_pointer - start_of_name_pointer != 1)
		{
			return Err(ExtendedDnsOptRecordNameTooLong)
		}

		let name_length_or_offset = start_of_name_pointer.dereference_u8();
		if unlikely!(name_length_or_offset != 0x00)
		{
			return Err(ExtendedDnsOptRecordNameNotRoot)
		}

		let extended_response_code_and_flags = self.extended_response_code_and_flags(end_of_name_pointer);
		extended_response_code_and_flags.validate_is_version_0()?;
		
		// NOTE: This behaviour violates RFC 6840, Section 5.6, Setting the DO Bit on Replies.
		if unlikely!(!extended_response_code_and_flags.dnssec_ok())
		{
			return Err(ExtendedDnsError::ResponseIgnoredDnsSec)
		}
		
		response_parsing_state.parsed_an_edns_opt_record(ExtendedDnsResponseCodeError::parse_error_code(extended_response_code_and_flags.extended_response_code_upper_8_bits(), authoritative_or_authenticated_or_neither, rcode_lower_4_bits)?)?;
		
		let _requestors_udp_payload_size = max(512, self.requestors_udp_payload_size(end_of_name_pointer));
		
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

		Ok(options.end_pointer())
	}

	#[inline(always)]
	fn handle_DNAME<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, parsed_names: &mut ParsedNames<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, is_some_if_present_in_answer_section_and_true_if_was_queried_for: Option<bool>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::DNAMEHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::DNAME, duplicate_resource_record_response_parsing)?;

		let length = resource_data.len();

		const MinimumDNameSize: usize = ParsedNameParser::NameMinimumSize;

		if unlikely!(length < MinimumDNameSize)
		{
			Err(HasTooShortALength(length))?
		}

		let end_of_resource_data_pointer = resource_data.end_pointer();

		let (record, end_of_dname_pointer) = ParsedNameParser::parse_name_uncompressed(parsed_names, resource_data.start_pointer(), end_of_resource_data_pointer, DataType::DNAME).map_err(DomainName)?;

		if unlikely!(end_of_dname_pointer != end_of_dname_pointer)
		{
			Err(DataRemainingAfterDName)?
		}

		// ^ the trait `From<HandleRecordTypeError<<ResourceRecordVisitor<'message>>::Error>>` is not implemented for `:HandleRecordTypeError<<RRV::Error>`
		//      |
		resource_record_visitor.DNAME(owner_name, cache_until, record, is_some_if_present_in_answer_section_and_true_if_was_queried_for).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::DNAME, error))?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_DS<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		self.handle_delegation_signer(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, RRV::DS_ignored, RRV::DS, false, DataType::DS, duplicate_resource_record_response_parsing, HandleRecordTypeError::DS)
	}

	#[inline(always)]
	fn handle_SSHFP<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::HandleRecordTypeError::*;
		use self::SSHFPHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::SSHFP, duplicate_resource_record_response_parsing)?;

		use self::SshFingerprintDigest::*;
		use self::SshFingerprintResourceRecordIgnoredBecauseReason::*;
		use self::SshPublicKeyAlgorithm::*;

		const PublicKeyAlgorithmSize: usize = 1;
		const DigestAlgorithmSize: usize = 1;
		const MinimumDigestSize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < PublicKeyAlgorithmSize + DigestAlgorithmSize + MinimumDigestSize)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let raw_public_key_algorithm = resource_data.u8(0);
		let public_key_algorithm: SshPublicKeyAlgorithm = match raw_public_key_algorithm
		{
			0 => Err(HasReservedPublicKeyAlgorithm)?,

			1 => RSA,

			2 =>
			{
				resource_record_visitor.SSHFP_ignored(owner_name, PublicKeyAlgorithmDsaIsEffectivelyObsolete);
				return Ok(resource_data_end_pointer)
			}

			3 => ECDSA,

			4 => Ed25519,

			_ =>
			{
				resource_record_visitor.SSHFP_ignored(owner_name, PublicKeyAlgorithmUnassigned(raw_public_key_algorithm));
				return Ok(resource_data_end_pointer)
			}
		};

		const DigestOffset: usize = PublicKeyAlgorithmSize + DigestAlgorithmSize;

		let raw_digest_algorithm = resource_data.u8(1);
		let public_key_digest = match raw_digest_algorithm
		{
			0 => Err(HasReservedDigestAlgorithm)?,

			1 =>
			{
				resource_record_visitor.SSHFP_ignored(owner_name, DigestAlgorithmSha1IsBroken);
				return Ok(resource_data_end_pointer)
			}

			2 => Self::guard_hash_digest_if_final_field(resource_data, DigestOffset, Sha2_256, |length| SSHFP(HasADigestLengthThatIsIncorrectForTheMatchingType(length)))?,

			_ =>
			{
				resource_record_visitor.SSHFP_ignored(owner_name, DigestAlgorithmUnassigned(raw_digest_algorithm));
				return Ok(resource_data_end_pointer)
			}
		};

		let record = PublicKeyFingerprint
		{
			public_key_algorithm,
			public_key_digest,
		};

		resource_record_visitor.SSHFP(owner_name, cache_until, record).map_err(|error| ResourceRecordVisitor(DataType::SSHFP, error))?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_IPSECKEY<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, parsed_names: &mut ParsedNames<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::HandleRecordTypeError::*;
		use self::IPSECKEYHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::IPSECKEY, duplicate_resource_record_response_parsing)?;

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
			Err(HasTooShortALength(length))?
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
					Err(HasTooShortALengthForAnInternetProtocolVersion4Gateway(length))?
				}
				let gateway = resource_data.value::<Ipv4Addr>(GatewayFieldStartsAtOffset);

				(GatewayFieldStartsAtOffset + size_of::<Ipv4Addr>(), Some(InternetProtocolVersion4(gateway)))
			}

			2 =>
			{
				if unlikely!(length < GatewayFieldStartsAtOffset + size_of::<Ipv6Addr>())
				{
					Err(HasTooShortALengthForAnInternetProtocolVersion6Gateway(length))?
				}
				let gateway = resource_data.value::<Ipv6Addr>(GatewayFieldStartsAtOffset);

				(GatewayFieldStartsAtOffset + size_of::<Ipv6Addr>(), Some(InternetProtocolVersion6(gateway)))
			}

			3 =>
			{
				if unlikely!(length < GatewayFieldStartsAtOffset + 1)
				{
					Err(HasTooShortALengthForDomainNameGateway(length))?
				}

				let resource_data_starts_at_pointer = resource_data.start_pointer();
				let start_of_name_pointer = resource_data_starts_at_pointer + GatewayFieldStartsAtOffset;
				let (domain_name, end_of_domain_name_pointer) = ParsedNameParser::parse_name_uncompressed(parsed_names, start_of_name_pointer, start_of_name_pointer + length - GatewayFieldStartsAtOffset, DataType::IPSECKEY).map_err(DomainNameGateway)?;

				(end_of_domain_name_pointer - resource_data_starts_at_pointer, Some(DomainName(domain_name)))
			}

			_ =>
			{
				resource_record_visitor.IPSECKEY_ignored(owner_name, GatewayTypeUnassigned(gateway_type));
				return Ok(resource_data_end_pointer)
			}
		};

		let public_key_algorithm_type = resource_data.u8(PrecedenceSize + GatewayTypeSize);
		let public_key_length = length - public_key_starts_at_offset;
		let public_key = match Self::ipsec_like_public_key
		(
			DataType::IPSECKEY,
			resource_record_visitor,
			public_key_algorithm_type,
			resource_data,
			public_key_starts_at_offset,
			public_key_length,
			|resource_record_visitor| resource_record_visitor.IPSECKEY_ignored(owner_name.clone(), PublicKeyAlgorithmDSAIsProbablyBroken),
			|resource_record_visitor| resource_record_visitor.IPSECKEY_ignored(owner_name.clone(), PublicKeyAlgorithmUnassigned(public_key_algorithm_type))
		).map_err(|error| IPSECKEY(IpsecLikePublicKeyHandleRecordType(error)))?
		{
			Left(public_key) => public_key,
			Right(()) => return Ok(resource_data_end_pointer)
		};

		let record = IpsecPublicKey
		{
			precedence: resource_data.u8(0),
			gateway,
			public_key,
		};

		resource_record_visitor.IPSECKEY(owner_name, cache_until, record).map_err(|error| ResourceRecordVisitor(DataType::IPSECKEY, error))?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_NSEC<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, parsed_names: &mut ParsedNames<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::NSECHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::NSEC, duplicate_resource_record_response_parsing)?;

		const MinimumNextSecureNameSize: usize = ParsedNameParser::NameMinimumSize;

		let length = resource_data.len();
		if unlikely!(length < MinimumNextSecureNameSize + TypeBitmaps::MinimumTypeBitmapsSize)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let resource_data_pointer = resource_data.start_pointer();
		let resource_data_end_pointer = resource_data.end_pointer();

		let (next_domain_name, end_of_next_domain_name_pointer) = ParsedNameParser::parse_name_uncompressed(parsed_names, resource_data_pointer, resource_data_end_pointer, DataType::NSEC).map_err(NextDomainName)?;

		let record = NextSecure
		{
			next_domain_name,
			type_bitmaps: TypeBitmaps::parse_type_bitmaps(DataType::NSEC, &resource_data[(end_of_next_domain_name_pointer - resource_data_pointer) .. ]).map_err(TypeBitmapsParse)?,
		};

		resource_record_visitor.NSEC(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::NSEC, error))?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_RRSIG<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, parsed_names: &mut ParsedNames<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, is_some_if_present_in_answer_section_and_true_if_was_queried_for: Option<bool>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::RRSIGHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::RRSIG, duplicate_resource_record_response_parsing)?;

		use self::ResourceRecordSetSignatureResourceRecordIgnoredBecauseReason::*;

		const TypeCoveredSize: usize = 2;
		const AlgorithmSize: usize = 1;
		const LabelsSize: usize = 1;
		const OriginalTimeToLiveSize: usize = 4;
		const SignatureExpirationSize: usize = 4;
		const SignatureInceptionSize: usize = 4;
		const KeyTagSize: usize = 2;
		const MinimumSignersNameSize: usize = ParsedNameParser::NameMinimumSize;
		const MinimumSignatureSize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize + SignatureInceptionSize + KeyTagSize + MinimumSignersNameSize + MinimumSignatureSize)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let labels = resource_data.u8(TypeCoveredSize + AlgorithmSize);
		if unlikely!(labels > 126)
		{
			Err(HasMoreThan126Labels(labels))?
		}

		let security_algorithm_type = resource_data.u8(TypeCoveredSize);
		let security_algorithm = match SecurityAlgorithm::parse_security_algorithm(security_algorithm_type, false, false, DataType::RRSIG).map_err(SecurityAlgorithmFailed)?
		{
			Left(security_algorithm) => security_algorithm,
			
			Right(security_algorithm_rejected_because_reason) =>
			{
				resource_record_visitor.RRSIG_ignored(owner_name, SecurityAlgorithmRejected(security_algorithm_rejected_because_reason), is_some_if_present_in_answer_section_and_true_if_was_queried_for);
				return Ok(resource_data_end_pointer)
			}
		};

		let signature_expiration_timestamp = resource_data.value::<SignatureTimestamp>(TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize);
		let signature_inception_timestamp = resource_data.value::<SignatureTimestamp>(TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize);
		
		match signature_expiration_timestamp.inception_and_expiration(signature_inception_timestamp)
		{
			None =>
			{
				resource_record_visitor.RRSIG_ignored(owner_name, DifferenceInSignatureExpirationAndInceptionIsTooGreatForWrappingSerialNumberMathematics { signature_inception_timestamp, signature_expiration_timestamp }, is_some_if_present_in_answer_section_and_true_if_was_queried_for);
				return Ok(resource_data_end_pointer)
			}
			
			Some(None) =>
			{
				resource_record_visitor.RRSIG_ignored(owner_name, DifferenceInSignatureInceptionAndExpirationWasNegativeOrZero { signature_inception_timestamp, signature_expiration_timestamp }, is_some_if_present_in_answer_section_and_true_if_was_queried_for);
				return Ok(resource_data_end_pointer)
			}
			
			Some(Some((signature_inception_time, signature_expiration_time))) =>
			{
				if unlikely!(signature_inception_time > now)
				{
					resource_record_visitor.RRSIG_ignored(owner_name, InceptionIsInTheFuture { signature_inception_timestamp, signature_expiration_timestamp }, is_some_if_present_in_answer_section_and_true_if_was_queried_for);
					return Ok(resource_data_end_pointer)
				}
				
				if unlikely!(signature_expiration_time <= now)
				{
					resource_record_visitor.RRSIG_ignored(owner_name, Expired { signature_inception_timestamp, signature_expiration_timestamp }, is_some_if_present_in_answer_section_and_true_if_was_queried_for);
					return Ok(resource_data_end_pointer)
				}
			}
		};
		
		let remaining_data = &resource_data[(TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize + SignatureInceptionSize + KeyTagSize) .. ];
		let remaining_data_pointer = remaining_data.start_pointer();

		let (signers_name, end_of_name_pointer) = ParsedNameParser::parse_name_uncompressed(parsed_names, remaining_data.start_pointer(), resource_data_end_pointer, DataType::RRSIG).map_err(SignersName)?;

		let signature_offset = TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize + SignatureInceptionSize + KeyTagSize + (end_of_name_pointer - remaining_data_pointer);
		let signature = &resource_data[signature_offset .. ];
		
		let record = ResourceRecordSetSignature
		{
			type_covered: resource_data.value::<DataType>(0),
			security_algorithm,
			labels,
			original_time_to_live: resource_data.value::<TimeInSeconds>(TypeCoveredSize + AlgorithmSize + LabelsSize),
			key_tag: resource_data.value::<KeyTag>(TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize + SignatureInceptionSize),
			signers_name,
			signature,
			rrsig_rdata_excluding_signature_field: &resource_data[ .. signature_offset],
		};

		resource_record_visitor.RRSIG(owner_name, cache_until, record, is_some_if_present_in_answer_section_and_true_if_was_queried_for).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::RRSIG, error))?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_DNSKEY<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		self.guard_dns_key(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, RRV::DNSKEY_ignored, RRV::DNSKEY, false, DataType::DNSKEY, duplicate_resource_record_response_parsing, HandleRecordTypeError::DNSKEY)
	}

	#[inline(always)]
	fn handle_DHCID<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::DHCIDHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::DHCID, duplicate_resource_record_response_parsing)?;

		use self::DhcidResourceRecordIgnoredBecauseReason::*;
		use self::HandleRecordTypeError::*;

		const IdentifierTypeCodeSize: usize = 2;
		const DigestTypeCodeSize: usize = 1;
		const MinimumDigestSize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < IdentifierTypeCodeSize + DigestTypeCodeSize + MinimumDigestSize)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let identifier_type_code = resource_data.u16(0);
		let identifier_type: IdentifierType = match identifier_type_code
		{
			0x0000 ..= 0x0002 => unsafe { transmute(identifier_type_code) },

			0x0003 ..= 0xFFFE =>
			{
				resource_record_visitor.DHCID_ignored(owner_name, IdentifierTypeUnassigned(identifier_type_code));
				return Ok(resource_data_end_pointer)
			}

			reserved @ _ => Err(HasAReservedIdentifierTypeCode(reserved))?
		};

		const DigestOffset: usize = IdentifierTypeCodeSize + DigestTypeCodeSize;
		let digest = match resource_data.u8(IdentifierTypeCodeSize)
		{
			0 => Err(HasReservedDigestTypeCode)?,

			1 => Self::guard_hash_digest_if_final_field(resource_data, DigestOffset, DhcidDigest::Sha2_256, |length| DHCID(HasADigestLengthThatIsIncorrectForTheMatchingType(length)))?,

			digest_type_code @ _ =>
			{
				resource_record_visitor.DHCID_ignored(owner_name, DigestAlgorithmUnassigned(digest_type_code));
				return Ok(resource_data_end_pointer)
			}
		};

		let record = Dhcid
		{
			identifier_type,
			digest,
		};

		resource_record_visitor.DHCID(owner_name, cache_until, record).map_err(|error| ResourceRecordVisitor(DataType::DHCID, error))?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_NSEC3<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::NSEC3HandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::NSEC3, duplicate_resource_record_response_parsing)?;

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
			Err(HasAnIncorrectLength(length))?
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let salt_length = resource_data.u8_as_usize(HashAlgorithmSize + FlagsSize + IterationsSize);
		let salt_end_offset = HashAlgorithmSize + FlagsSize + IterationsSize + SaltLengthSize + salt_length;

		if unlikely!(salt_end_offset > length)
		{
			Err(HasAnOverflowingSaltLength(salt_length))?
		}
		let salt = &resource_data[SaltStartOffset .. salt_end_offset];

		let hash_algorithm_number = resource_data.u8(0);
		let (next_hashed_owner_name, hash_end_offset) = match hash_algorithm_number
		{
			0 => Err(HasAReservedHashAlgorithm)?,

			1 =>
			{
				let hash_length = resource_data.u8_as_usize(salt_end_offset);

				const DigestSizeInBits: usize = 160;
				const DigestSize: usize = DigestSizeInBits / ResourceRecord::BitsInAByte;

				if unlikely!(hash_length != DigestSize)
				{
					Err(HasAnIncorrectHashLengthForASha1Hash(length))?
				}

				let hash_start_offset = salt_end_offset + HashLengthSize;

				let hash_end_offset = hash_start_offset + hash_length;

				if unlikely!(hash_end_offset > length)
				{
					Err(HasAnOverflowingHashLength(hash_length))?
				}

				let hash = NextSecureVersion3Hash::Sha_1(resource_data.cast::<[u8; DigestSize]>(hash_start_offset));

				(hash, hash_end_offset)
			}

			_ =>
			{
				resource_record_visitor.NSEC3_ignored(owner_name, UnassignedHashAlgorithm(hash_algorithm_number));
				return Ok(resource_data_end_pointer)
			}
		};

		let flags = resource_data.u8(HashAlgorithmSize);

		const OptOut: u8 = 7;
		const OptOutFlag: u8 = 1 << (7 - OptOut);
		const KnownFlags: u8 = OptOutFlag;

		if unlikely!(flags & !KnownFlags != 0)
		{
			resource_record_visitor.NSEC3_ignored(owner_name, UnassignedFlags(flags));
			return Ok(resource_data_end_pointer)
		}

		let record = NextSecureVersion3
		{
			opt_out: flags & OptOutFlag != 0,

			iterations: resource_data.u16(HashAlgorithmSize + FlagsSize),

			salt,

			next_hashed_owner_name,

			type_bitmaps: TypeBitmaps::parse_type_bitmaps(DataType::NSEC3, &resource_data[hash_end_offset .. ]).map_err(TypeBitmapsParse)?,
		};

		resource_record_visitor.NSEC3(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::NSEC3, error))?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_NSEC3PARAM<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::NSEC3PARAMHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::NSEC3PARAM, duplicate_resource_record_response_parsing)?;

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
			Err(HasAnIncorrectLength(length))?
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let salt_length = resource_data.u8_as_usize(HashAlgorithmSize + FlagsSize + IterationsSize);
		let salt_end_offset = HashAlgorithmSize + FlagsSize + IterationsSize + SaltLengthSize + salt_length;

		if unlikely!(salt_end_offset > length)
		{
			Err(HasAnOverflowingSaltLength(salt_length))?
		}
		let salt = &resource_data[SaltStartOffset .. salt_end_offset];

		let hash_algorithm_number = match resource_data.u8(0)
		{
			0 => Err(HasAReservedHashAlgorithm)?,

			1 => NextSecureVersion3Parameters::Sha1HashAlgorithmNumber,

			hash_algorithm_number @ _ =>
			{
				resource_record_visitor.NSEC3PARAM_ignored(owner_name, UnassignedHashAlgorithm(hash_algorithm_number));
				return Ok(resource_data_end_pointer)
			}
		};

		// We are meant to ignore the reserved flags.
		let flags = resource_data.u8(HashAlgorithmSize);
		if unlikely!(flags != 0)
		{
			resource_record_visitor.NSEC3PARAM_ignored(owner_name, UnassignedFlags(flags));
			return Ok(resource_data_end_pointer)
		}

		let record = NextSecureVersion3Parameters
		{
			hash_algorithm_number,

			iterations: resource_data.u16(HashAlgorithmSize + FlagsSize),

			salt,
		};

		resource_record_visitor.NSEC3PARAM(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::NSEC3PARAM, error))?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_TLSA<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::HandleRecordTypeError::*;
		
		let (resource_data_end_pointer, either) = self.handle_TLSA_or_SMIMEA(now, end_of_name_pointer, end_of_message_pointer, &owner_name, DataType::TLSA, duplicate_resource_record_response_parsing, TLSA)?;

		match either
		{
			Left((cache_until, record)) => resource_record_visitor.TLSA(owner_name, cache_until, record).map_err(|error| ResourceRecordVisitor(DataType::TLSA, error))?,

			Right(resource_record_ignored_because_reason) => resource_record_visitor.TLSA_ignored(owner_name, resource_record_ignored_because_reason),
		}

		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_SMIMEA<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::HandleRecordTypeError::*;
		
		let (resource_data_end_pointer, either) = self.handle_TLSA_or_SMIMEA(now, end_of_name_pointer, end_of_message_pointer, &owner_name, DataType::SMIMEA, duplicate_resource_record_response_parsing, SMIMEA)?;

		match either
		{
			Left((cache_until, record)) => resource_record_visitor.SMIMEA(owner_name, cache_until, record).map_err(|error| ResourceRecordVisitor(DataType::SMIMEA, error))?,

			Right(resource_record_ignored_because_reason) => resource_record_visitor.SMIMEA_ignored(owner_name, resource_record_ignored_because_reason),
		}

		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_HIP<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, parsed_names: &mut ParsedNames<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::HandleRecordTypeError::*;
		use self::HIPHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::HIP, duplicate_resource_record_response_parsing)?;

		use self::HostIdentityProtocolResourceRecordIgnoredBecauseReason::*;

		const HostIdentityTagLengthSize: usize = 1;
		const PublicKeyAlgorithmTypeSize: usize = 1;
		const PublicKeyLengthSize: usize = 2;
		const MinimumHostIdentityTagLength: usize = 0;
		const MinimumPublicKeyLength: usize = 0;
		const MinimumNumberOfRendezvousServersIsOneSoMinimumNameSizeIsOne: usize = ParsedNameParser::NameMinimumSize;
		const HostIdentityTagOffset: usize = HostIdentityTagLengthSize + PublicKeyAlgorithmTypeSize + PublicKeyLengthSize;

		let length = resource_data.len();
		if unlikely!(length < HostIdentityTagOffset + MinimumHostIdentityTagLength + MinimumPublicKeyLength + MinimumNumberOfRendezvousServersIsOneSoMinimumNameSizeIsOne)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let host_identity_tag_length = resource_data.u8_as_usize(0);
		if unlikely!(length < HostIdentityTagOffset + host_identity_tag_length + MinimumPublicKeyLength + MinimumNumberOfRendezvousServersIsOneSoMinimumNameSizeIsOne)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let public_key_algorithm_type = resource_data.u8(HostIdentityTagLengthSize);
		let public_key_starts_at_offset = HostIdentityTagOffset + host_identity_tag_length;
		let public_key_length = resource_data.u16_as_usize(HostIdentityTagLengthSize + PublicKeyAlgorithmTypeSize);
		let public_key = match Self::ipsec_like_public_key
		(
			DataType::HIP,
			resource_record_visitor,
			public_key_algorithm_type,
			resource_data,
			public_key_starts_at_offset,
			public_key_length,
			|resource_record_visitor| resource_record_visitor.HIP_ignored(owner_name.clone(), PublicKeyAlgorithmDSAIsProbablyBroken),
			|resource_record_visitor| resource_record_visitor.HIP_ignored(owner_name.clone(), PublicKeyAlgorithmUnassigned(public_key_algorithm_type))
		).map_err(|error| HIP(IpsecLikePublicKeyHandleRecordType(error)))?
		{
			Left(public_key) => public_key,
			Right(()) => return Ok(resource_data_end_pointer),
		};

		let start_of_name_pointer = resource_data.start_pointer() + HostIdentityTagOffset + host_identity_tag_length + public_key_length;
		let (first_rendezvous_server_domain_name, true_end_of_name_pointer) = ParsedNameParser::parse_name_uncompressed(parsed_names, start_of_name_pointer, resource_data_end_pointer, DataType::HIP).map_err(FirstRendezvousServerDomainName)?;

		let remaining_rendezvous_servers_length = resource_data_end_pointer - true_end_of_name_pointer;
		let remaining_rendezvous_server_domain_names = true_end_of_name_pointer.unsafe_cast_slice::<u8>(remaining_rendezvous_servers_length);

		let record = HostIdentityProtocol
		{
			host_identity_tag: &resource_data[HostIdentityTagOffset .. public_key_starts_at_offset],

			public_key,

			first_rendezvous_server_domain_name,

			remaining_rendezvous_server_domain_names,
		};

		resource_record_visitor.HIP(owner_name, cache_until, record).map_err(|error| ResourceRecordVisitor(DataType::HIP, error))?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_CDS<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		self.handle_delegation_signer(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, RRV::CDS_ignored, RRV::CDS, true, DataType::CDS, duplicate_resource_record_response_parsing, HandleRecordTypeError::CDS)
	}

	#[inline(always)]
	fn handle_CDNSKEY<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		self.guard_dns_key(now, end_of_name_pointer, end_of_message_pointer, owner_name, resource_record_visitor, RRV::CDNSKEY_ignored, RRV::CDNSKEY, true, DataType::CDNSKEY, duplicate_resource_record_response_parsing, HandleRecordTypeError::CDNSKEY)
	}

	#[inline(always)]
	fn handle_OPENPGPKEY<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::OPENPGPKEY, duplicate_resource_record_response_parsing)?;

		resource_record_visitor.OPENPGPKEY(owner_name, cache_until, resource_data).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::OPENPGPKEY, error))?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_CSYNC<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::CSYNCHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::CSYNC, duplicate_resource_record_response_parsing)?;

		use self::ChildSynchronizeResourceRecordIgnoredBecauseReason::*;

		const StartOfAuthoritySize: usize = 4;
		const FlagsSize: usize = 2;

		let length = resource_data.len();
		if unlikely!(length < StartOfAuthoritySize + FlagsSize + TypeBitmaps::MinimumTypeBitmapsSize)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let flags = resource_data.u16(StartOfAuthoritySize);

		const immediate: u16 = 0x0001;
		const soaminimum: u16 = 0x0002;
		const KnownFlags: u16 = immediate | soaminimum;

		if unlikely!(flags & !KnownFlags != 0)
		{
			resource_record_visitor.CSYNC_ignored(owner_name, UnassignedFlags(flags));
			return Ok(resource_data_end_pointer)
		}

		let record = ChildSynchronize
		{
			start_of_authority_serial: resource_data.value::<SerialNumber>(0),

			immediate: flags & immediate != 0,

			start_of_authority_minimum: flags & soaminimum != 0,

			type_bitmaps: TypeBitmaps::parse_type_bitmaps(DataType::CSYNC, &resource_data[(StartOfAuthoritySize + FlagsSize) .. ]).map_err(TypeBitmapsParse)?
		};

		resource_record_visitor.CSYNC(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::CSYNC, error))?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_NID<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::NIDHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::NID, duplicate_resource_record_response_parsing)?;

		const PreferenceSize: usize = 2;
		const NodeIdentifierSize: usize = 8;

		let length = resource_data.len();
		if unlikely!(length != PreferenceSize + NodeIdentifierSize)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let record = NodeIdentifier
		{
			preference: resource_data.u16(0),
			node_identifier: resource_data.u64(PreferenceSize),
		};

		resource_record_visitor.NID(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::NID, error))?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_L32<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::L32HandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::L32, duplicate_resource_record_response_parsing)?;

		const PreferenceSize: usize = 2;
		const LocatorSize: usize = 4;

		let length = resource_data.len();
		if unlikely!(length != PreferenceSize + LocatorSize)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let record = Locator32
		{
			preference: resource_data.u16(0),
			locator: resource_data.value::<Ipv4Addr>(PreferenceSize),
		};

		resource_record_visitor.L32(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::L32, error))?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_L64<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::L64HandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::L64, duplicate_resource_record_response_parsing)?;

		const PreferenceSize: usize = 2;
		const LocatorSize: usize = 8;

		let length = resource_data.len();
		if unlikely!(length != PreferenceSize + LocatorSize)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let record = Locator64
		{
			preference: resource_data.u16(0),
			locator: resource_data.value::<[u8; LocatorSize]>(PreferenceSize),
		};

		resource_record_visitor.L64(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::L64, error))?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_LP<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, parsed_names: &mut ParsedNames<'message>, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::LPHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::LP, duplicate_resource_record_response_parsing)?;

		const PreferenceSize: usize = 2;
		const MinimumNameSize: usize = ParsedNameParser::NameMinimumSize;

		let length = resource_data.len();
		if unlikely!(length < PreferenceSize + MinimumNameSize)
		{
			Err(HasTooShortALength(length))?
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let domain_name_data = &resource_data[PreferenceSize .. ];
		let (domain_name, end_of_name_pointer) = ParsedNameParser::parse_name_uncompressed(parsed_names, domain_name_data.start_pointer(), resource_data_end_pointer, DataType::LP).map_err(DomainName)?;
		if unlikely!(end_of_name_pointer != resource_data_end_pointer)
		{
			Err(HasDataLeftOver)?
		}
		
		if unlikely!(owner_name == domain_name)
		{
			Err(HasDomainNameSameAsRecordName)?
		}

		let record = LocatorPointer
		{
			preference: resource_data.u16(0),
			domain_name,
		};

		resource_record_visitor.LP(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::LP, error))?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_EUI48<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::EUI48HandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::EUI48, duplicate_resource_record_response_parsing)?;

		const Eui48Size: usize = 48 / ResourceRecord::BitsInAByte;

		let length = resource_data.len();
		if unlikely!(length != Eui48Size)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let record = resource_data.value::<[u8; Eui48Size]>(0);

		resource_record_visitor.EUI48(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::EUI48, error))?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_EUI64<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::EUI64HandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::EUI64, duplicate_resource_record_response_parsing)?;

		const Eui64Size: usize = 64 / ResourceRecord::BitsInAByte;

		let length = resource_data.len();
		if unlikely!(length != Eui64Size)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let record = resource_data.value::<[u8; Eui64Size]>(0);

		resource_record_visitor.EUI64(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::EUI64, error))?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_uri<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::URIHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::URI, duplicate_resource_record_response_parsing)?;

		const PrioritySize: usize = 2;
		const WeightSize: usize = 2;
		const MinimumTargetSize: usize = 1;

		let length = resource_data.len();
		if unlikely!(length < PrioritySize + WeightSize + MinimumTargetSize)
		{
			Err(HasAnIncorrectLength(length))?
		}
		
		let target_uri_bytes = &resource_data[(PrioritySize + WeightSize)..];
		let target_uri = URI::try_from(target_uri_bytes).map_err(InvalidTargetUri)?;
		
		let record = Uri
		{
			priority: Priority(resource_data.u16(0)),
			weight: Weight(resource_data.u16(PrioritySize)),
			target_uri,
		};

		resource_record_visitor.URI(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::URI, error))?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_CAA<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::CAAHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, DataType::CAA, duplicate_resource_record_response_parsing)?;

		use self::CertificateAuthorityAuthorizationResourceRecordIgnoredBecauseReason::*;

		const FlagsSize: usize = 1;
		const TagLengthSize: usize = 1;
		const MinimumTagSize: usize = 1;
		const MinimumValueSize: usize = 0;
		const PropertyTagOffset: usize = FlagsSize + TagLengthSize;

		let length = resource_data.len();
		if unlikely!(length < FlagsSize + TagLengthSize + MinimumTagSize + MinimumValueSize)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let tag_length = resource_data.u8_as_usize(FlagsSize);

		if unlikely!(tag_length == 0)
		{
			Err(HasAZeroTagLength)?
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		if unlikely!(tag_length > 15)
		{
			resource_record_visitor.CAA_ignored(owner_name, TagLengthExceeded15(tag_length));
			return Ok(resource_data_end_pointer)
		}

		let property_value_offset = PropertyTagOffset + tag_length;

		if unlikely!(property_value_offset > length)
		{
			Err(HasAnIncorrectLength(length))?
		}

		let flags = resource_data.u8(0);

		// // See <https://www.iana.org/assignments/pkix-parameters/pkix-parameters.xhtml>; note that bit 0 is MSB, ie bits are numbered from left-to-right.
		const IssuerCriticalFlagBit: u8 = 0b1000_0000;
		const ReservedFlagBits: u8 = !IssuerCriticalFlagBit;

		if unlikely!(flags & ReservedFlagBits != 0)
		{
			resource_record_visitor.CAA_ignored(owner_name, UseOfUnassignedFlagBits(flags));
			return Ok(resource_data_end_pointer)
		}
		
		let property_tag_bytes = &resource_data[PropertyTagOffset .. property_value_offset];
		let property_tag = match CertificateAuthorityAuthorizationPropertyTag::known_tag(property_tag_bytes)
		{
			Some(&Some(property_tag)) => property_tag,

			Some(&None) =>
			{
				resource_record_visitor.CAA_ignored(owner_name, TagReservedByRfcErrata3547(property_tag_bytes));
				return Ok(resource_data_end_pointer)
			}

			None =>
			{
				resource_record_visitor.CAA_ignored(owner_name, TagUnassigned(property_tag_bytes));
				return Ok(resource_data_end_pointer)
			}
		};

		let record = CertificateAuthorityAuthorization
		{
			is_issuer_critical: flags & 0b0000_0001 != 0,
			property_tag,
			property_value: &resource_data[property_value_offset .. ],
		};

		resource_record_visitor.CAA(owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(DataType::CAA, error))?;

		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_TLSA_or_SMIMEA<'message, E: error::Error>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: &ParsedName<'message>, data_type: DataType, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, map_error: impl FnOnce(X509CertificateHandleRecordTypeError) -> HandleRecordTypeError<E>) -> Result<(usize, Either<(CacheUntil, DnsBasedAuthenticationOfNamedEntities<'message>), DnsBasedAuthenticationOfNamedEntitiesResourceRecordIgnoredBecauseReason>), HandleRecordTypeError<E>>
	{
		use self::X509CertificateHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, owner_name, end_of_name_pointer, end_of_message_pointer, data_type, duplicate_resource_record_response_parsing)?;

		use self::DnsBasedAuthenticationOfNamedEntitiesResourceRecordIgnoredBecauseReason::*;

		const CertificateUsageSize: usize = 1;
		const SelectorSize: usize = 1;
		const MatchingTypeSize: usize = 1;
		const MinimumDigestSize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < CertificateUsageSize + SelectorSize + MatchingTypeSize + MinimumDigestSize)
		{
			return Err(map_error(HasAnIncorrectLength(length)))
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
		let raw_matching_type = resource_data.u8(CertificateUsageSize + SelectorSize);
		let matching_type = match raw_matching_type
		{
			0 => MatchingType::NoHashUsed(&resource_data[DigestOffset .. ]),

			1 => Self::guard_hash_digest_if_final_field(resource_data, DigestOffset, MatchingType::Sha2_256, |length| map_error(HasADigestLengthThatIsIncorrectForTheMatchingType(length)))?,

			2 => Self::guard_hash_digest_if_final_field(resource_data, DigestOffset, MatchingType::Sha2_512, |length| map_error(HasADigestLengthThatIsIncorrectForTheMatchingType(length)))?,

			3 ..= 254 => return Ok((resource_data_end_pointer, Right(UnassignedMatchingType(raw_matching_type)))),

			255 => return Ok((resource_data_end_pointer, Right(PrivateMatchingType))),
		};

		Ok
		(
			(
				resource_data_end_pointer,
				Left
				(
					(
						cache_until,
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
	fn parse_internet_protocol_address_only<'message, E: error::Error, Address: Copy>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: &ParsedName<'message>, data_type: DataType, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, error: impl FnOnce(usize) -> HandleRecordTypeError<E>) -> Result<(CacheUntil, Address, usize), HandleRecordTypeError<E>>
	{
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, owner_name, end_of_name_pointer, end_of_message_pointer, data_type, duplicate_resource_record_response_parsing)?;

		let length = resource_data.len();
		if unlikely!(length != size_of::<Address>())
		{
			Err(error(length))
		}
		else
		{
			let address = resource_data.value::<Address>(0);
			Ok((cache_until, address, resource_data.end_pointer()))
		}
	}

	#[inline(always)]
	fn parse_name_only<'message, E: error::Error>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: &ParsedName<'message>, parsed_names: &mut ParsedNames<'message>, data_type: DataType, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, error: impl FnOnce(ParsedNameParserError) -> HandleRecordTypeError<E>) -> Result<(CacheUntil, ParsedName<'message>, usize), HandleRecordTypeError<E>>
	{
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, owner_name, end_of_name_pointer, end_of_message_pointer, data_type, duplicate_resource_record_response_parsing)?;

		let record = ParsedNameParser::parse_name_in_slice_with_nothing_left(data_type, parsed_names, resource_data).map_err(error)?;
		Ok((cache_until, record, resource_data.end_pointer()))
	}

	#[inline(always)]
	fn validate_minimum_record_size_and_parse_name_and_resource_record_type<'message>(&'message self, end_of_message_pointer: usize, parsed_names: &mut ParsedNames<'message>) -> Result<(ParsedName<'message>, usize, (u8, u8)), ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError>
	{
		use self::ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError::*;
		
		let start_of_resource_record_pointer = self.as_usize_pointer();
		if unlikely!(start_of_resource_record_pointer + Self::MinimumSize > end_of_message_pointer)
		{
			return Err(ResourceRecordIsShorterThanMinimumSize)
		}
		
		let (owner_name, end_of_owner_name_pointer) = ParsedNameParser::parse_name(None, parsed_names, self.start_of_name_pointer(), end_of_message_pointer)?;

		if unlikely!(end_of_owner_name_pointer + ResourceRecordFooter::MinimumSize > end_of_message_pointer)
		{
			return Err(ResourceRecordIsShorterThanMinimumSizeAfterParsingName)
		}

		let resource_record_type = self.resource_record_type(end_of_owner_name_pointer);

		Ok((owner_name, end_of_owner_name_pointer, resource_record_type.upper_and_lower()))
	}

	#[inline(always)]
	fn validate_class_is_internet_and_get_cache_until_and_resource_data<'message>(&'message self, now: NanosecondsSinceUnixEpoch, owner_name: &ParsedName<'message>, end_of_name_pointer: usize, end_of_message_pointer: usize, data_type: DataType, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<(CacheUntil, &'message [u8]), ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(&owner_name, end_of_name_pointer, end_of_message_pointer, data_type, duplicate_resource_record_response_parsing)?;
		
		Ok((time_to_live.cache_until(now), resource_data))
	}

	#[inline(always)]
	fn validate_class_is_internet_and_get_time_to_live_and_resource_data<'message>(&'message self, owner_name: &ParsedName<'message>, end_of_name_pointer: usize, end_of_message_pointer: usize, data_type_or_meta_type: impl DataTypeOrMetaType, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>) -> Result<(TimeInSeconds, &'message [u8]), ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError>
	{
		self.resource_record_class_is_internet(end_of_name_pointer)?;

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer).map_err(|error| ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError::ResourceDataLengthOverflows(data_type_or_meta_type.into_big_endian_u16(), error))?;
		
		duplicate_resource_record_response_parsing.encountered(data_type_or_meta_type, owner_name, resource_data)?;

		let time_to_live = self.time_to_live(end_of_name_pointer);
		
		Ok((time_to_live, resource_data))
	}

	#[inline(always)]
	fn safely_access_resource_data<'message>(&'message self, end_of_name_pointer: usize, end_of_message_pointer: usize) -> Result<&'message [u8], ResourceDataLengthOverflowsError>
	{
		let resource_data_length = self.resource_data_length(end_of_name_pointer) as usize;
		if unlikely!(end_of_name_pointer + resource_data_length > end_of_message_pointer)
		{
			Err(ResourceDataLengthOverflowsError)
		}
		else
		{
			Ok(self.resource_data(end_of_name_pointer).unsafe_cast_slice::<u8>(resource_data_length))
		}
	}

	/// `NAME` field.
	#[inline(always)]
	fn start_of_name_pointer(&self) -> usize
	{
		self.as_usize_pointer()
	}

	/// `TYPE` field.
	#[inline(always)]
	fn resource_record_type(&self, end_of_name_pointer: usize) -> DataType
	{
		self.footer(end_of_name_pointer).resource_record_type()
	}

	/// `CLASS` field.
	#[inline(always)]
	fn resource_record_class_is_internet(&self, end_of_name_pointer: usize) -> Result<(), ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError>
	{
		self.footer(end_of_name_pointer).resource_record_class_is_internet()
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
	fn time_to_live(&self, end_of_name_pointer: usize) -> TimeInSeconds
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
	fn resource_data(&self, end_of_name_pointer: usize) -> &ResourceData
	{
		self.footer(end_of_name_pointer).resource_data()
	}

	#[inline(always)]
	fn footer(&self, end_of_name_pointer: usize) -> &ResourceRecordFooter
	{
		end_of_name_pointer.unsafe_cast::<ResourceRecordFooter>()
	}

	#[inline(always)]
	fn handle_delegation_signer<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, ignored_callback: impl FnOnce(&mut RRV, ParsedName<'message>, DelegationSignerResourceRecordIgnoredBecauseReason), visit_callback: impl Fn(&mut RRV, ParsedName<'message>, CacheUntil, DelegationSigner<'message>) -> Result<(), RRV::Error>, permit_delete: bool, data_type: DataType, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, map_error: impl Fn(DelegationSignerHandleRecordTypeError) -> HandleRecordTypeError<RRV::Error>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		use self::DelegationSignerHandleRecordTypeError::*;
		
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, data_type, duplicate_resource_record_response_parsing)?;
	
		use self::DelegationSignerResourceRecordIgnoredBecauseReason::*;
		use self::DigestAlgorithmRejectedBecauseReason::*;
	
		const KeyTagSize: usize = 2;
		const SecurityAlgorithmTypeSize: usize = 1;
		const DigestTypeSize: usize = 1;
		const MinimumDigestSize: usize = 0;
	
		let length = resource_data.len();
		if unlikely!(length < KeyTagSize + SecurityAlgorithmTypeSize + DigestTypeSize + MinimumDigestSize)
		{
			return Err(map_error(HasAnIncorrectLength(data_type, length)))
		}
	
		let resource_data_end_pointer = resource_data.end_pointer();
	
		let security_algorithm_type = resource_data.u8(KeyTagSize);
		let security_algorithm = match SecurityAlgorithm::parse_security_algorithm(security_algorithm_type, permit_delete, false, data_type).map_err(|error| map_error(SecurityAlgorithmFailed(error)))?
		{
			Left(security_algorithm) => security_algorithm,
			
			Right(security_algorithm_rejected_because_reason) =>
			{
				ignored_callback(resource_record_visitor, owner_name, SecurityAlgorithmRejected(security_algorithm_rejected_because_reason));
				return Ok(resource_data_end_pointer)
			}
		};
	
		const DigestOffset: usize = KeyTagSize + SecurityAlgorithmTypeSize + DigestTypeSize;
	
		let digest_type = resource_data.u8(KeyTagSize + SecurityAlgorithmTypeSize);
		let digest = match digest_type
		{
			0 => return Err(map_error(DigestAlgorithmTypeIsReservedByRfc3658(data_type))),
	
			1 =>
			{
				ignored_callback(resource_record_visitor, owner_name, DigestAlgorithmRejected(Sha1IsBroken));
				return Ok(resource_data_end_pointer)
			}
	
			2 => Self::guard_hash_digest_if_final_field(resource_data, DigestOffset, DnsSecDigest::Sha2_256, |length| map_error(HasADigestLengthThatIsIncorrectForTheDigestType(data_type, length)))?,
	
			3 =>
			{
				ignored_callback(resource_record_visitor, owner_name, DigestAlgorithmRejected(Gost94MayBeBroken));
				return Ok(resource_data_end_pointer)
			}
	
			4 => Self::guard_hash_digest_if_final_field(resource_data, DigestOffset, DnsSecDigest::Sha2_384, |length| map_error(HasADigestLengthThatIsIncorrectForTheDigestType(data_type, length)))?,
	
			_ =>
			{
				ignored_callback(resource_record_visitor, owner_name, DigestAlgorithmRejected(Unassigned(digest_type)));
				return Ok(resource_data_end_pointer)
			}
		};
	
		let record = DelegationSigner
		{
			key_tag: resource_data.value::<KeyTag>(0),
			security_algorithm,
			digest,
		};
	
		visit_callback(resource_record_visitor, owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(data_type, error))?;
		Ok(resource_data_end_pointer)
	}
	
	#[inline(always)]
	fn guard_dns_key<'message, RRV: ResourceRecordVisitor<'message>>(&'message self, now: NanosecondsSinceUnixEpoch, end_of_name_pointer: usize, end_of_message_pointer: usize, owner_name: ParsedName<'message>, resource_record_visitor: &mut RRV, ignored_callback: impl FnOnce(&mut RRV, ParsedName<'message>, DnsKeyResourceRecordIgnoredBecauseReason), visit_callback: impl FnOnce(&mut RRV, ParsedName<'message>, CacheUntil, DnsKey<'message>) -> Result<(), RRV::Error>, permit_delete: bool, data_type: DataType, duplicate_resource_record_response_parsing: &DuplicateResourceRecordResponseParsing<'message>, map_error: impl FnOnce(DnsKeyHandleRecordTypeError) -> HandleRecordTypeError<RRV::Error>) -> Result<usize, HandleRecordTypeError<RRV::Error>>
	{
		let (cache_until, resource_data) = self.validate_class_is_internet_and_get_cache_until_and_resource_data(now, &owner_name, end_of_name_pointer, end_of_message_pointer, data_type, duplicate_resource_record_response_parsing)?;

		use self::DnsKeyPurpose::*;
		use self::DnsKeyResourceRecordIgnoredBecauseReason::*;

		const FlagsSize: usize = 2;
		const ProtocolSize: usize = 1;
		const AlgorithmSize: usize = 1;
		const MinimumPublicKeySize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < FlagsSize + ProtocolSize)
		{
			return Err(map_error(DnsKeyHandleRecordTypeError::HasAnIncorrectLength(data_type, length)))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let protocol = resource_data.u8(FlagsSize);
		if unlikely!(protocol != 3)
		{
			ignored_callback(resource_record_visitor, owner_name, ProtocolWasNot3(protocol));
			return Ok(resource_data_end_pointer)
		}

		if unlikely!(length < FlagsSize + ProtocolSize + AlgorithmSize + MinimumPublicKeySize)
		{
			return Err(map_error(DnsKeyHandleRecordTypeError::HasAnIncorrectLengthForProtocol3(data_type, length)))
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
			ignored_callback(resource_record_visitor, owner_name, UnassignedFlags(flags));
			return Ok(resource_data_end_pointer)
		}

		let is_revoked = flags & RevokedFlag != 0;
		if unlikely!(is_revoked)
		{
			ignored_callback(resource_record_visitor, owner_name, Revoked);
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
				ignored_callback(resource_record_visitor, owner_name, SecureEntryPointFlagSetButNotZoneKeyFlag);
				return Ok(resource_data_end_pointer)
			}
			KeySigningKey
		};

		let security_algorithm_type = resource_data.u8(FlagsSize + ProtocolSize);
		let security_algorithm = match SecurityAlgorithm::parse_security_algorithm(security_algorithm_type, permit_delete, false, data_type).map_err(|error| map_error(DnsKeyHandleRecordTypeError::SecurityAlgorithm(error)))?
		{
			Left(security_algorithm) => security_algorithm,
			
			Right(security_algorithm_rejected_because_reason) =>
			{
				ignored_callback(resource_record_visitor, owner_name, SecurityAlgorithmRejected(security_algorithm_rejected_because_reason));
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
		
		visit_callback(resource_record_visitor, owner_name, cache_until, record).map_err(|error| HandleRecordTypeError::ResourceRecordVisitor(data_type, error))?;
		Ok(resource_data_end_pointer)
	}
	
	#[inline(always)]
	fn guard_hash_digest_if_final_field<'message, D: Digest<'message>, N: Sized, E: error::Error>(resource_data: &'message [u8], digest_offset: usize, name: impl FnOnce(D) -> N, error: impl FnOnce(usize) -> HandleRecordTypeError<E>) -> Result<N, HandleRecordTypeError<E>>
	{
		let digest_data = &resource_data[digest_offset .. ];
		
		let length = digest_data.len();
		
		if length == D::DigestSizeInBytes
		{
			let d = unsafe { D::new_unchecked(digest_data.as_ptr()) };
			Ok(name(d))
		}
		else
		{
			Err(error(length))
		}
	}
	
	#[inline(always)]
	fn ipsec_like_public_key<'message, RRV: ResourceRecordVisitor<'message>>(data_type: DataType, resource_record_visitor: &mut RRV, public_key_algorithm_type: u8, resource_data: &'message [u8], public_key_starts_at_offset: usize, public_key_length: usize, dsa_ignored_callback: impl FnOnce(&mut RRV), unassigned_ignored_callback: impl FnOnce(&mut RRV)) -> Result<Either<Option<PublicKey<'message>>, ()>, IpsecLikePublicKeyHandleRecordTypeError>
	{
		use self::IpsecLikePublicKeyHandleRecordTypeError::*;
		use self::PublicKey::*;
	
		let public_key = match public_key_algorithm_type
		{
			0 => if unlikely!(public_key_length != 0)
			{
				return Err(HasWrongLengthForNoPublicKey(data_type, public_key_length))
			}
			else
			{
				None
			},
	
			1 =>
			{
				dsa_ignored_callback(resource_record_visitor);
				return Ok(Right(()))
			}
	
			2 =>
			{
				if unlikely!(public_key_length == 0)
				{
					return Err(HasTooShortALengthForRSAPublicKey(data_type, public_key_length))
				}
	
				let public_key_data = &resource_data[public_key_starts_at_offset .. public_key_starts_at_offset + public_key_length];
	
				const FirstByteSize: usize = 1;
	
				let first_byte_of_exponent_length = public_key_data.u8(0);
				let (exponent_and_modulus, exponent_length) = if first_byte_of_exponent_length == 0
				{
					const SecondAndThirdBytesSize: usize = 2;
	
					const SizeSize: usize = FirstByteSize + SecondAndThirdBytesSize;
	
					if unlikely!(public_key_data.len() < SizeSize)
					{
						return Err(HasTooShortALengthForRSAPublicKeyForAThreeByteExponentLength(data_type, public_key_length))
					}
	
					(&public_key_data[SizeSize .. ], public_key_data.u16_as_usize(FirstByteSize))
				}
				else
				{
					(&public_key_data[FirstByteSize .. ], first_byte_of_exponent_length as usize)
				};
	
				if unlikely!(exponent_length == 0)
				{
					return Err(HasAZeroExponentForARSAPublicKey(data_type))
				}
	
				if unlikely!(exponent_and_modulus.len() < exponent_length)
				{
					return Err(HasTooShortALengthForARSAPublicKeyForExponentLength(data_type))
				}
	
				let modulus_length = exponent_and_modulus.len() - exponent_length;
				if unlikely!(modulus_length == 0)
				{
					return Err(HasAZeroModulusForARSAPublicKey(data_type))
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
	
				if unlikely!(public_key_length != 256 / BitsInAByte || public_key_length != 384 / BitsInAByte)
				{
					return Err(HasAUnusualLengthForAnECDSAPublicKey(data_type, public_key_length))
				}
	
				let public_key_data = &resource_data[public_key_starts_at_offset .. public_key_starts_at_offset + public_key_length];
	
				let ec_dsa_public_key = EcDsaPublicKey
				{
					Q: public_key_data,
				};
	
				Some(ECDSA(ec_dsa_public_key))
			}
			
			_ =>
			{
				unassigned_ignored_callback(resource_record_visitor);
				return Ok(Right(()))
			}
		};
		
		Ok(Left(public_key))
	}
}
