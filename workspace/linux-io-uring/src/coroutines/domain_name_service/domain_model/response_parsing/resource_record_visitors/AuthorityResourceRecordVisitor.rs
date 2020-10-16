// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct AuthorityResourceRecordVisitor<'message>
{
	canonical_name_chain: CanonicalNameChain<'message>,
	have_seen_a_ns_record: bool,
	have_seen_a_soa_record: Option<NegativeCachingTimeToLiveInSeconds>,
}

impl<'message> ResourceRecordVisitor<'message> for AuthorityResourceRecordVisitor<'message>
{
	#[inline(always)]
	fn NS(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: WithCompressionParsedName<'message>) -> Result<(), DnsProtocolError>
	{
		if unlikely!(self.canonical_name_chain.validate_authority_section_name(name))
		{
			return Err(NameServerRecordInAuthoritySectionIsNotForFinalNameInCanonicalNameChain)
		}

		self.have_seen_a_ns_record = true;

		Ok(())
	}

	#[inline(always)]
	fn SOA(&mut self, name: WithCompressionParsedName<'message>, time_to_live: TimeToLiveInSeconds, record: StartOfAuthority<'message>) -> Result<(), DnsProtocolError>
	{
		if unlikely!(self.canonical_name_chain.validate_authority_section_name(name))
		{
			return Err(StartOfAuthorityRecordInAuthoritySectionIsNotForFinalNameInCanonicalNameChain)
		}

		// NOTE: We are supposed to only used `negative_caching_time_to_live`.
		// However, if the SOA record itself lives for less than `negative_caching_time_to_live`, then a future update to the SOA record may have changed the value of `negative_caching_time_to_live` such that it would expire before the current `negative_caching_time_to_live`.
		// For example:-
		//   SOA TTL = 1s, SOA MINIMUM = 5s;
		//   After 1s, a new SOA is published with SOA MINIMUM = 2s;
		//   If we used the original SOA MINIMUM = 5s, we would wait 4s before obtaining this, whereas our cached value was invalid after 3s.
		// Hence, using the lower of the two values allows for this change to be effective.
		// There is a slight cost of more frequent querying.
		let resource_record_time_to_live = time_to_live.into();
		let negative_caching_time_to_live = record.footer.negative_caching_time_to_live.into();
		let negative_caching_time_to_live = min(resource_record_time_to_live, negative_caching_time_to_live);
		self.have_seen_a_soa_record = Some(negative_caching_time_to_live);

		Ok(())
	}
}

impl<'message> AuthorityResourceRecordVisitor<'message>
{
	#[inline(always)]
	pub(crate) fn new(canonical_name_chain: CanonicalNameChain<'message>) -> Self
	{
		Self
		{
			canonical_name_chain,
			have_seen_a_ns_record: false,
			have_seen_a_soa_record: None
		}
	}
	
	/// Applies the rules in RFC 2308 to determine the outcome.
	///
	/// This logic does not work if the requested record type was `SOA`, `CNAME` or `DNAME`, and probably does not work also for `NS`.
	/// However, there is very little reason to request these record types for normal clients.
	pub(crate) fn answer_outcome(&self, is_authoritative_answer: bool, has_nxdomain_error_code: bool, answer_section_has_at_least_one_record_of_requested_data_type: bool) -> AnswerOutcome
	{
		use self::AnswerOutcome::*;

		// RFC 2308 Section 5: "Negative responses without SOA records SHOULD NOT be cached as there is no way to prevent the negative responses looping forever between a pair of servers even with a short TTL".
		const NoNegativeCachingTimeToLiveInSecondsIfNoStartOfAuthorityRecordReturned: NegativeCachingTimeToLiveInSeconds = 0;

		if unlikely!(is_authoritative_answer)
		{
			match (answer_section_has_at_least_one_record_of_requested_data_type, self.have_seen_a_soa_record, self.have_seen_a_ns_record, has_nxdomain_error_code)
			{
				// RFC 2308 Section 2.1 "NXDOMAIN RESPONSE: TYPE 1".
				(false, Some(negative_caching_time_to_live), true, true) => Ok(NameError(negative_caching_time_to_live)),

				// RFC 2308 Section 2.1 "NXDOMAIN RESPONSE: TYPE 2".
				(false, Some(negative_caching_time_to_live), false, true) => Ok(NameError(negative_caching_time_to_live)),

				// RFC 2308 Section 2.1 "NXDOMAIN RESPONSE: TYPE 3".
				// This response would seem to violate the requirements of RFC 2308 Section 3: "Name servers authoritative for a zone MUST include the SOA record of the zone in the authority section of the response when reporting an NXDOMAIN or indicating that no data of the requested type exists".
				(false, None, false, true) => Ok(NameError(NoNegativeCachingTimeToLiveInSecondsIfNoStartOfAuthorityRecordReturned)),

				// RFC 2308 Section 2.1 "NXDOMAIN RESPONSE: TYPE 4".
				// This response would seem to violate the requirements of RFC 2308 Section 3: "Name servers authoritative for a zone MUST include the SOA record of the zone in the authority section of the response when reporting an NXDOMAIN or indicating that no data of the requested type exists".
				(false, None, true, true) => Ok(NameError(NoNegativeCachingTimeToLiveInSecondsIfNoStartOfAuthorityRecordReturned)),

				// RFC 2308 Section 2.1 "REFERRAL RESPONSE".
				(false, None, true, false) => Ok(Referral),

				(true, _, _, true) => Err(ResponseWasAuthoritativeWithNoSuchDomainErrorCodeButContainsAnAnswer),

				// ? Should there be SOA / NS records in authority section if answered ?
				(true, _, _, false) => Ok(Answered),
			}
		}
		else
		{
			debug_assert_eq!(has_nxdomain_error_code, false, "The `NXDOMAIN` error code is invalid for non-authoritative answers; this should have been validated when checking the message_header");

			match (answer_section_has_at_least_one_record_of_requested_data_type, self.have_seen_a_soa_record, self.have_seen_a_ns_record)
			{
				// RFC 2308 Section 2.2 "NODATA RESPONSE: TYPE 1".
				(false, Some(negative_caching_time_to_live), true) => Ok(NoData(negative_caching_time_to_live)),

				// RFC 2308 Section 2.2 "NODATA RESPONSE: TYPE 2".
				(false, Some(negative_caching_time_to_live), false) => Ok(NoData(negative_caching_time_to_live)),

				// RFC 2308 Section 2.2 "NODATA RESPONSE: TYPE 3".
				(false, None, false) => Ok(NoData(NoNegativeCachingTimeToLiveInSecondsIfNoStartOfAuthorityRecordReturned)),

				// RFC 2308 Section 2.2 "REFERRAL RESPONSE".
				(false, None, true) => Ok(Referral),

				// ? Should there be SOA / NS records in authority section if answered ?
				(true, _, _) => Ok(Answered),
			}
		}
	}
}
