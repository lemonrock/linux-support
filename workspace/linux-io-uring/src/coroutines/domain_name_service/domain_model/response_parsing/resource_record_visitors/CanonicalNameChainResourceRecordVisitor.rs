// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct CanonicalNameChainAnswerSectionResourceRecordVisitor<'message, RRV: ResourceRecordVisitor<'message, Finished=OwnerNameToRecords<'message, R>>, R: ParsedRecord>
{
	answer_section_resource_record_visitor: RRV,
	
	canonical_name_chain: CanonicalNameChain<'message>,
}

impl<'message, RRV: ResourceRecordVisitor<'message, Finished=OwnerNameToRecords<'message, R>>, R: ParsedRecord> CanonicalNameChainAnswerSectionResourceRecordVisitor<'message, RRV, R>
{
	#[inline(always)]
	pub(crate) fn new(answer_section_resource_record_visitor: RRV, query_name: &'message EfficientCaseFoldedName) -> Self
	{
		Self
		{
			answer_section_resource_record_visitor,
		
			canonical_name_chain: CanonicalNameChain::new(query_name),
		}
	}
	
	/// eg:-
	///
	/// * `name` is `www.microsoft.com.`.
	/// * `time_to_live` is `1168`.
	/// * `record` is `www.microsoft.com-c-3.edgekey.net.`.
	///
	/// Assumes that CNAME records in the answer section are sorted in chain order.
	/// Whilst they don't have to be, only a poorly implemented server is likely to not do this.
	#[inline(always)]
	fn add_canonical_link(&mut self, from: &ParsedName<'message>, cache_until: CacheUntil, to: &ParsedName<'message>) -> Result<(), CanonicalChainError>
	{
		self.canonical_name_chain.insert_link(from, cache_until, to)
	}
	
	#[inline(always)]
	fn store_delegation_name(&mut self, from: &ParsedName<'message>, cache_until: CacheUntil, to: &ParsedName<'message>) -> Result<(), CanonicalChainError>
	{
		self.canonical_name_chain.store_delegation_name(from, cache_until, to)
	}
}

impl<'message, RRV: ResourceRecordVisitor<'message, Finished=OwnerNameToRecords<'message, R>>, R: ParsedRecord> ResourceRecordVisitor<'message> for CanonicalNameChainAnswerSectionResourceRecordVisitor<'message, RRV, R>
{
	type Error = WrappingCanonicalChainError<RRV::Error>;
	
	type Finished = (RRV::Finished, CanonicalNameChain<'message>);
	
	fn finished(self) -> Self::Finished
	{
		(self.answer_section_resource_record_visitor.finished(), self.canonical_name_chain)
	}
	
	#[inline(always)]
	fn A(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: Ipv4Addr) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.A(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn NS(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: NameServerName<ParsedName<'message>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.NS(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn SOA(&mut self, name: ParsedName<'message>, negative_cache_until: NegativeCacheUntil, record: StartOfAuthority<ParsedName<'message>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.SOA(name, negative_cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn CNAME(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: ParsedName<'message>, is_some_if_present_in_answer_section_and_true_if_was_queried_for: Option<bool>) -> Result<(), Self::Error>
	{
		debug_assert!(is_some_if_present_in_answer_section_and_true_if_was_queried_for.is_some());
		
		if is_some_if_present_in_answer_section_and_true_if_was_queried_for == Some(false)
		{
			self.add_canonical_link(&name, cache_until, &record)?;
		}
		
		self.answer_section_resource_record_visitor.CNAME(name, cache_until, record, is_some_if_present_in_answer_section_and_true_if_was_queried_for).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn PTR(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, alias_or_domain_target: ParsedName<'message>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.PTR(name, cache_until, alias_or_domain_target).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn MX(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, preference: Priority, mail_server_name: MailServerName<ParsedName<'message>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.MX(name, cache_until, preference, mail_server_name).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn HINFO(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: HostInformation<ParsedCharacterString<'message>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.HINFO(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn TXT(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: Vec<ParsedCharacterString<'message>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.TXT(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn AAAA(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: Ipv6Addr) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.AAAA(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn LOC_version_0(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: &'message LocationBodyVersion0) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.LOC_version_0(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn SRV(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, priority: Priority, weight: Weight, record: ServiceLocation<'message>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.SRV(name, cache_until, priority, weight, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn NAPTR(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, order: Order, preference: Priority, record: NamingAuthorityPointer<ParsedName<'message>, ParsedUri<'message>, ParsedCharacterString<'message>, ParsedTypeEquality>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.NAPTR(name, cache_until, order, preference, record).map_err(WrappingCanonicalChainError::Wrapped)
	}
	
	#[inline(always)]
	fn NAPTR_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: NamingAuthorityResourceRecordIgnoredReason)
	{
		self.answer_section_resource_record_visitor.NAPTR_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn KX(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, preference: Priority, key_exchange_server_name: ParsedName<'message>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.KX(name, cache_until, preference, key_exchange_server_name).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn CERT(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: Certificate<ParsedBytes<'message>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.CERT(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn CERT_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: CertificateResourceRecordIgnoredBecauseReason)
	{
		self.answer_section_resource_record_visitor.CERT_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn DNAME(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: ParsedName<'message>, is_some_if_present_in_answer_section_and_true_if_was_queried_for: Option<bool>) -> Result<(), Self::Error>
	{
		debug_assert!(is_some_if_present_in_answer_section_and_true_if_was_queried_for.is_some());
		
		if is_some_if_present_in_answer_section_and_true_if_was_queried_for == Some(false)
		{
			self.store_delegation_name(&name, cache_until, &record)?;
		}
		
		self.answer_section_resource_record_visitor.DNAME(name, cache_until, record, is_some_if_present_in_answer_section_and_true_if_was_queried_for).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn DS(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: DelegationSigner<Parsed<'message, Sha2_256>, Parsed<'message, Sha2_384>, ParsedTypeEquality>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.DS(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn DS_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: DelegationSignerResourceRecordIgnoredBecauseReason)
	{
		self.answer_section_resource_record_visitor.DS_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn SSHFP(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: PublicKeyFingerprint<Parsed<'message, Sha2_256>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.SSHFP(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn SSHFP_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: SshFingerprintResourceRecordIgnoredBecauseReason)
	{
		self.answer_section_resource_record_visitor.SSHFP_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn IPSECKEY(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, precedence: Priority, record: IpsecPublicKey<ParsedName<'message>, ParsedBytes<'message>, ParsedTypeEquality>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.IPSECKEY(name, cache_until, precedence, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn IPSECKEY_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: IpsecKeyResourceRecordIgnoredBecauseReason)
	{
		self.answer_section_resource_record_visitor.IPSECKEY_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn NSEC(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: NextSecure<ParsedName<'message>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.NSEC(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn RRSIG(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: ResourceRecordSetSignature<ParsedName<'message>, ParsedBytes<'message>, ParsedTypeEquality>, is_some_if_present_in_answer_section_and_true_if_was_queried_for: Option<bool>) -> Result<(), Self::Error>
	{
		debug_assert!(is_some_if_present_in_answer_section_and_true_if_was_queried_for.is_some());
		
		self.answer_section_resource_record_visitor.RRSIG(name, cache_until, record, is_some_if_present_in_answer_section_and_true_if_was_queried_for).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn RRSIG_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: ResourceRecordSetSignatureResourceRecordIgnoredBecauseReason, is_some_if_present_in_answer_section_and_true_if_was_queried_for: Option<bool>)
	{
		debug_assert!(is_some_if_present_in_answer_section_and_true_if_was_queried_for.is_some());
		
		self.answer_section_resource_record_visitor.RRSIG_ignored(name, resource_record_ignored_because_reason, is_some_if_present_in_answer_section_and_true_if_was_queried_for)
	}

	#[inline(always)]
	fn DNSKEY(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: DnsKey<ParsedBytes<'message>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.DNSKEY(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn DNSKEY_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: DnsKeyResourceRecordIgnoredBecauseReason)
	{
		self.answer_section_resource_record_visitor.DNSKEY_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn DHCID(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: Dhcid<Parsed<'message, Sha2_256>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.DHCID(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn DHCID_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: DhcidResourceRecordIgnoredBecauseReason)
	{
		self.answer_section_resource_record_visitor.DHCID_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn NSEC3(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: NextSecureVersion3<Parsed<'message, Sha1>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.NSEC3(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn NSEC3_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: NextSecureVersion3ResourceRecordIgnoredBecauseReason)
	{
		self.answer_section_resource_record_visitor.NSEC3_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn NSEC3PARAM(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: NextSecureVersion3Parameters<ParsedBytes<'message>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.NSEC3PARAM(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn NSEC3PARAM_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: NextSecureVersion3ParametersResourceRecordIgnoredBecauseReason)
	{
		self.answer_section_resource_record_visitor.NSEC3PARAM_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn TLSA(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: DnsBasedAuthenticationOfNamedEntities<ParsedBytes<'message>, Parsed<'message, Sha2_256>, Parsed<'message, Sha2_512>, ParsedTypeEquality>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.TLSA(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn TLSA_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: DnsBasedAuthenticationOfNamedEntitiesResourceRecordIgnoredBecauseReason)
	{
		self.answer_section_resource_record_visitor.TLSA_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn SMIMEA(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: DnsBasedAuthenticationOfNamedEntities<ParsedBytes<'message>, Parsed<'message, Sha2_256>, Parsed<'message, Sha2_512>, ParsedTypeEquality>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.SMIMEA(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn SMIMEA_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: DnsBasedAuthenticationOfNamedEntitiesResourceRecordIgnoredBecauseReason)
	{
		self.answer_section_resource_record_visitor.SMIMEA_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn HIP(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: HostIdentityProtocol<ParsedName<'message>, ParsedBytes<'message>, ParsedTypeEquality>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.HIP(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn HIP_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: HostIdentityProtocolResourceRecordIgnoredBecauseReason)
	{
		self.answer_section_resource_record_visitor.HIP_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn CDNSKEY(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: DnsKey<ParsedBytes<'message>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.CDNSKEY(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn CDNSKEY_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: DnsKeyResourceRecordIgnoredBecauseReason)
	{
		self.answer_section_resource_record_visitor.CDNSKEY_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn CDS(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: DelegationSigner<Parsed<'message, Sha2_256>, Parsed<'message, Sha2_384>, ParsedTypeEquality>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.CDS(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn CDS_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: DelegationSignerResourceRecordIgnoredBecauseReason)
	{
		self.answer_section_resource_record_visitor.CDS_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn OPENPGPKEY(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: OpenPgpRfc4880TransferablePublicKey<ParsedBytes<'message>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.OPENPGPKEY(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn CSYNC(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: ChildSynchronize) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.CSYNC(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn CSYNC_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: ChildSynchronizeResourceRecordIgnoredBecauseReason)
	{
		self.answer_section_resource_record_visitor.CSYNC_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn NID(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, preference: Priority, record: NodeIdentifier) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.NID(name, cache_until, preference, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn L32(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, preference: Priority, record: Locator32) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.L32(name, cache_until, preference, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn L64(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, preference: Priority, record: Locator64) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.L64(name, cache_until, preference, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn LP(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, preference: Priority, record: LocatorPointer<ParsedName<'message>>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.LP(name, cache_until, preference, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn EUI48(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: [u8; 6]) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.EUI48(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn EUI64(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: [u8; 8]) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.EUI64(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn URI(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, priority: Priority, weight: Weight, target_uri: URI<'message>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.URI(name, cache_until, priority, weight, target_uri).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn CAA(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: CertificateAuthorityAuthorization<'message>) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.CAA(name, cache_until, record).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn CAA_ignored(&mut self, name: ParsedName<'message>, resource_record_ignored_because_reason: CertificateAuthorityAuthorizationResourceRecordIgnoredBecauseReason<'message>)
	{
		self.answer_section_resource_record_visitor.CAA_ignored(name, resource_record_ignored_because_reason)
	}

	#[inline(always)]
	fn handle_possible_future_standard(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: &'message [u8], unsupported_resource_record_type: DataType) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.handle_possible_future_standard(name, cache_until, record, unsupported_resource_record_type).map_err(WrappingCanonicalChainError::Wrapped)
	}

	#[inline(always)]
	fn unassigned(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: &'message [u8], unassigned_resource_record_type: DataType) -> Result<(), Self::Error>
	{
		self.answer_section_resource_record_visitor.unassigned(name, cache_until, record, unassigned_resource_record_type).map_err(WrappingCanonicalChainError::Wrapped)
	}
}
