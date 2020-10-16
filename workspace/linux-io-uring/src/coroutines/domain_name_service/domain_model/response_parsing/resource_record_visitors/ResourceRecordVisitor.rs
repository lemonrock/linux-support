// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Visits different kinds of records.
pub(crate) trait ResourceRecordVisitor<'message>
{
	/// Visits a record of type `A`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn A(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: Ipv4Addr) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `NS`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn NS(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: WithCompressionParsedName<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `SOA`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn SOA(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: StartOfAuthority<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `CNAME`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn CNAME(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: WithCompressionParsedName<'message>, is_some_if_present_in_answer_section_and_true_if_was_queried_for: Option<bool>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `PTR`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn PTR(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: WithCompressionParsedName<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `MX`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn MX(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: MailExchange<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `HINFO`.
	///
	/// Default implementation does nothing.
	///
	/// `HINFO` had been brought back into use by RFC 8482.
	#[inline(always)]
	fn HINFO(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: HostInformation<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `TXT`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn TXT(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: CharacterStringsIterator) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `AAAA`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn AAAA(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: Ipv6Addr) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `LOC`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn LOC(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: &Location) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `SRV`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn SRV(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: ServiceLocation<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `NAPTR`, with a domain name.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn NAPTR_domain_name(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: NamingAuthorityPointerWithDomainName<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `NAPTR`, with a regular expression.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn NAPTR_regular_expression(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: NamingAuthorityPointerWithRegularExpression<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `KX`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn KX(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: KeyExchange<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `CERT`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn CERT(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: Certificate<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `CERT` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn CERT_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: CertificateResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `DNAME`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn DNAME(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: WithoutCompressionParsedName<'message>, is_some_if_present_in_answer_section_and_true_if_was_queried_for: Option<bool>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `DS`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn DS(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: DelegationSigner<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `DS` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn DS_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: DelegationSignerResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `SSHFP`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn SSHFP(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: PublicKeyFingerprint<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `SSHFP` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn SSHFP_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: SshFingerprintResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `IPSECKEY`.
	///
	/// Default implementation does nothing.
	///
	/// Note that the leading bytes of the exponent and modulus are unchecked for a RSA public key.
	#[inline(always)]
	fn IPSECKEY(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: IpsecPublicKey<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `IPSECKEY` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn IPSECKEY_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: IpsecKeyResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `NSEC`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn NSEC(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: NextSecure<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `RRSIG`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn RRSIG(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: ResourceRecordSetSignature<'message>, is_some_if_present_in_answer_section_and_true_if_was_queried_for: Option<bool>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `RRSIG` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn RRSIG_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: ResourceRecordSetSignatureResourceRecordIgnoredBecauseReason, is_some_if_present_in_answer_section_and_true_if_was_queried_for: Option<bool>)
	{
	}

	/// Visits a record of type `DNSKEY`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn DNSKEY(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: DnsKey<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `DNSKEY` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn DNSKEY_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: DnsKeyResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `DHCID`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn DHCID(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: Dhcid<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `DHCID` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn DHCID_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: DhcidResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `NSEC3`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn NSEC3(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: NextSecureVersion3<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `NSEC3` which was ignored.
	///
	/// Default implementation does nothing.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn NSEC3_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: NextSecureVersion3ResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `NSEC3PARAM`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn NSEC3PARAM(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: NextSecureVersion3Parameters<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `NSEC3PARAM` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn NSEC3PARAM_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: NextSecureVersion3ParametersResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `TLSA`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn TLSA(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: DnsBasedAuthenticationOfNamedEntities<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `TLSA` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn TLSA_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: DnsBasedAuthenticationOfNamedEntitiesResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `SMIMEA`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn SMIMEA(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: DnsBasedAuthenticationOfNamedEntities<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `SMIMEA` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn SMIMEA_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: DnsBasedAuthenticationOfNamedEntitiesResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `HIP`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn HIP(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: HostIdentityProtocol<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `HIP` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn HIP_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: HostIdentityProtocolResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `CDNSKEY`.
	///
	/// Default implementation does nothing.
	///
	/// Note that the algorithm `DS-Delete` is NOT validated.
	#[inline(always)]
	fn CDNSKEY(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: DnsKey<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `CDNSKEY` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn CDNSKEY_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: DnsKeyResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `CDS`.
	///
	/// Default implementation does nothing.
	///
	/// Note that the algorithm `DS-Delete` is NOT validated.
	#[inline(always)]
	fn CDS(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: DelegationSigner<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `CDS` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn CDS_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: DelegationSignerResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `OPENPGPKEY`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn OPENPGPKEY(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: OpenPgpRfc4880TransferablePublicKey<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `CSYNC`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn CSYNC(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: ChildSynchronize) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `CSYNC` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn CSYNC_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: ChildSynchronizeResourceRecordIgnoredBecauseReason)
	{
	}

	/// Visits a record of type `NID`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn NID(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: NodeIdentifier) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `L32`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn L32(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: Locator32) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `L64`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn L64(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: Locator64) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `LP`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn LP(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: LocatorPointer<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `EUI48`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn EUI48(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: [u8; 6]) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `EUI64`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn EUI64(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: [u8; 8]) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `URI`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn URI(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: Uri<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `CAA`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn CAA(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: CertificateAuthorityAuthorization<'message>) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits a record of type `CAA` which was ignored.
	///
	/// Default implementation does nothing.
	#[allow(unused)]
	#[inline(always)]
	fn CAA_ignored(&mut self, name: WithCompressionParsedName<'message>, resource_record_ignored_because_reason: CertificateAuthorityAuthorizationResourceRecordIgnoredBecauseReason<'message>)
	{
	}

	/// Visits an unsupported record type that might become a future standard.
	///
	/// Default implementation ignores it.
	#[allow(unused)]
	#[inline(always)]
	fn handle_possible_future_standard(&mut self, name: WithCompressionParsedName<'message>, _time_to_live: TimeToLiveInSeconds, _record: &'message [u8], _unsupported_resource_record_type: DataType) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}

	/// Visits an unassigned record type.
	///
	/// Default implementation ignores it.
	#[allow(unused)]
	#[inline(always)]
	fn unassigned(&mut self, name: WithCompressionParsedName<'message>, _time_to_live: TimeToLiveInSeconds, _record: &'message [u8], _unassigned_resource_record_type: DataType) -> Result<(), DnsProtocolError>
	{
		Ok(())
	}
}
