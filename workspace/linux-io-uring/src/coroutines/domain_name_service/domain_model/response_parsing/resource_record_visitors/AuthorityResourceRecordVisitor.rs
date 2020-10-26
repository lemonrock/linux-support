// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct AuthorityResourceRecordVisitor<'message>
{
	canonical_name_chain: CanonicalNameChain<'message>,
	
	/// *MUST* be for the parent of the final entry in the canonical name chain.
	/// It is valid to have no records.
	name_server_records: RefCell<Records<'message, ParsedName<'message>>>,
	
	/// *MUST* be for the parent of the final entry in the canonical name chain.
	/// It is valid to have no records.
	start_of_authority_record: RefCell<Option<(NegativeCacheUntil, StartOfAuthority<'message, ParsedName<'message>>)>>,
}

impl<'message> ResourceRecordVisitor<'message> for AuthorityResourceRecordVisitor<'message>
{
	type Error = AuthorityError;
	
	#[inline(always)]
	fn NS(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: ParsedName<'message>) -> Result<(), Self::Error>
	{
		if unlikely!(self.canonical_name_chain.validate_authority_section_name(name))
		{
			return Err(AuthorityError::NameServerRecordInAuthoritySectionIsNotForFinalNameInCanonicalNameChain)
		}
		
		let name_server_records = self.name_server_records.borrow_mut().deref_mut();
		name_server_records.store_unprioritized_and_unweighted(cache_until, record);

		Ok(())
	}

	#[inline(always)]
	fn SOA(&mut self, name: ParsedName<'message>, negative_cache_until: NegativeCacheUntil, record: StartOfAuthority<'message, ParsedName<'message>>) -> Result<(), Self::Error>
	{
		use self::AuthorityError::*;
		
		if unlikely!(self.canonical_name_chain.validate_authority_section_name(name))
		{
			return Err(StartOfAuthorityRecordInAuthoritySectionIsNotForFinalNameInCanonicalNameChain)
		}
		
		let start_of_authority_record = self.start_of_authority_record.borrow_mut().deref_mut();
		if likely!(start_of_authority_record.is_none())
		{
			*start_of_authority_record = Some((negative_cache_until, start_of_authority));
			Ok(())
		}
		else
		{
			Err(MoreThanOneStartOfAuthorityRecordInAuthoritySection)
		}
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
			name_server_records: RefCell::new(Records::with_capacity(4)),
			start_of_authority_record: RefCell::new(None)
		}
	}
	
	/// Scenarios as per RFC 2308 and RFC 8020.
	///
	/// # Section 2 Negative Responses
	///
	/// * Name Error
	/// 	* Query name: `an.example.` for a `QTYPE` of `A`.
	/// 	* The query name `an.example.` actually exists; there is a `CNAME` record (or records) returned pointing to `tripple.xx.`;
	/// 	* The `RDCODE` of `NXDOMAIN` refers to the non-existence of `tripple.xx.`, *not* `an.example.`!
	/// 	* The `RDCODE` is all that is needed to parse these responses, but it refers to the canonical name chain; if there is no canonical name chain, then it refers to the query name.
	/// 	* Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 1
	/// 		* `RDCODE`: `NXDOMAIN`.
	/// 		* Answer
	/// 			* `an.example. CNAME tripple.xx.`
	/// 		* Authority
	/// 			* `xx. SOA ns1.xx. …`
	/// 			* `xx. NS ns1.xx.`
	/// 			* `xx. NS ns2.xx.`
	/// 		* Additional
	/// 			* `ns1.xx. A 127.0.0.2`.
	/// 			* `ns2.xx. A 127.0.0.3`.
	/// 	* Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 2
	/// 		* `RDCODE`: `NXDOMAIN`.
	/// 		* Answer
	/// 			* `an.example. CNAME tripple.xx.`
	/// 		* Authority
	/// 			* `xx. SOA ns1.xx. …`
	/// 		* Additional
	/// 	* Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 3
	/// 		* `RDCODE`: `NXDOMAIN`.
	/// 		* Answer
	/// 			* `an.example. CNAME tripple.xx.`
	/// 		* Authority
	/// 		* Additional
	/// 	* Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 4
	/// 		* `RDCODE`: `NXDOMAIN`.
	/// 		* Answer
	/// 			* `an.example. CNAME tripple.xx.`
	/// 		* Authority
	/// 			* `xx. NS ns1.xx.`
	/// 			* `xx. NS ns2.xx.`
	/// 		* Additional
	/// 			* `ns1.xx. A 127.0.0.2`.
	/// 			* `ns2.xx. A 127.0.0.3`.
	/// * No Data
	/// 	* Query name: `another.example.` for a `QTYPE` of `A`.
	/// 	* These responses have to be determined using an algorithm.
	/// 	* The `RDCODE` of `NOERROR` is all that is needed to parse these responses, but it refers to the canonical name chain; if there is no canonical name chain, then it refers to the query name.
	/// 	* Thus if there is a canonical name chain then the concept of `No Data` refers to the final referral in the canonical name chain.
	/// 	* Section 2.2 No Data NODATA RESPONSE: TYPE 1
	/// 		* `RDCODE`: `NOERROR` (success).
	/// 		* Answer
	/// 		* Authority
	/// 			* `example. SOA ns1.xx. …`
	/// 			* `example. NS ns1.xx.`
	/// 			* `example. NS ns2.xx.`
	/// 		* Additional
	/// 			* `ns1.xx. A 127.0.0.2`.
	/// 			* `ns2.xx. A 127.0.0.3`.
	/// 	* Section 2.2 No Data NODATA RESPONSE: TYPE 2
	/// 		* `RDCODE`: `NOERROR` (success).
	/// 		* Answer
	/// 		* Authority
	/// 			* `example. SOA ns1.xx.`
	/// 		* Additional
	/// 	* Section 2.2 No Data NODATA RESPONSE: TYPE 3
	/// 		* `RDCODE`: `NOERROR` (success).
	/// 		* Answer
	/// 		* Authority
	/// 		* Additional
	/// * Referrals
	/// 	* Section 2.1 Name Error REFERRAL RESPONSE (the referral is for `tripple.xx.`).
	/// 		* Query name: `an.example.` for a `QTYPE` of `A`.
	/// 		* `RDCODE`: `NOERROR` (success)
	/// 		* Answer
	/// 			* `an.example. CNAME tripple.xx.`
	/// 		* Authority
	/// 			* `xx. NS ns1.xx.`
	/// 			* `xx. NS ns2.xx.`
	/// 		* Additional
	/// 			* `ns1.xx. A 127.0.0.2`.
	/// 			* `ns2.xx. A 127.0.0.3`.
	/// 	* Section 2.2 No Data REFERRAL RESPONSE (the referral is for `another.example.`).
	/// 		* Query name: `another.example.` for a `QTYPE` of `A`.
	/// 		* `RDCODE`: `NOERROR` (success)
	/// 		* Answer
	/// 		* Authority
	/// 			* `example. NS ns1.xx.`
	/// 			* `example. NS ns2.xx.`
	/// 		* Additional
	/// 			* `ns1.xx. A 127.0.0.2`.
	/// 			* `ns2.xx. A 127.0.0.3`.
	/// * A real world example (1)
	/// 	* Query name: `foobar.dnsknowledge.com.`, query type `A`.
	/// 	* `RDCODE`: `NXDOMAIN`.
	/// 	* Answer
	/// 	* Authority
	/// 		* `dnsknowledge.com.	3600	IN	SOA	ns07ci.stackpathdns.net. …`.
	/// 	* Additional
	/// * A real world example (2)
	/// 	* Query name: `foobar.`, query type `A`.
	/// 	* `RDCODE`: `NXDOMAIN`.
	/// 	* Answer
	/// 	* Authority
	/// 		* `.			86400	IN	SOA	a.root-servers.net. …`.
	/// 	* Additional
	/// * A real world example (3)
	/// 	* Query name: `.`, query type `A`.
	/// 	* `RDCODE`: `NOERROR`.
	/// 	* Answer
	/// 	* Authority
	/// 		* `.			86400	IN	SOA	a.root-servers.net. …`.
	/// 	* Additional
	///
	/// RFC 2308, Section 6 - Negative answers from the cache, Paragraph 3:-
	/// "An implicit referral is characterised by `NS` records in the authority section referring the resolver towards a authoritative source.
	/// `NXDOMAIN` types 1 and 4 responses contain implicit referrals as does `NODATA` type 1 response".
	///
	/// RFC 2308, Section 7.1 Server Failure (OPTIONAL):-
	/// "\[A\] resolver MAY cache a server failure response.
	/// If it does so it MUST NOT cache it for longer than five (5) minutes, and it MUST be cached against the specific query tuple `<query name, type, class, server IP address>`".
	///
	/// RFC 2308, Section 7.2 Dead / Unreachable Server (OPTIONAL):-
	/// "Dead / Unreachable servers are servers that fail to respond in any way to a query or where the transport layer has provided an indication that the server does not exist or is unreachable
	/// …
	/// A server MAY cache a dead server indication.
	/// If it does so it MUST NOT cache it for longer than five (5) minutes, and it MUST be cached against the specific query tuple `<query name, type, class, server IP address>` unless there was a transport layer indication that the server does not exist, in which case it applies to all queries to that specific IP address".
	///
	/// This logic *MUST NOT BE USED* for queries for anything that can occur in the Authority section, eg `CNAME`, `SOA`, `NS`, etc.
	pub(crate) fn answer(self, answer_existence: AnswerExistence, answer_section_has_at_least_one_record_of_requested_data_type: bool) -> Result<(Answer<'message, ParsedName<'message>>, Records<'message, ParsedName<'message>>), AuthoritySectionError<AuthorityError>>
	{
		use self::Answer::*;
		use self::NoDataResponseType::*;
		use self::NoDomainResponseType::*;
		use self::AnswerExistence::*;
		
		let has_a_start_of_authority_record = self.start_of_authority_record.borrow().is_some();
		let has_name_server_records = !self.name_server_records.borrow().has_records();
		
		// RFC 2308, Section 3, Negative Answers from Authoritative Servers: "Name servers authoritative for a zone MUST include the SOA record of the zone in the authority section of the response when reporting an NXDOMAIN or indicating that no data of the requested type exists".
		#[inline(always)]
		fn guard_against_authoritative_answer_without_start_of_authority_record(authoritative_or_authenticated_or_neither: AuthoritativeOrAuthenticatedOrNeither) -> Result<(), AuthoritySectionError<AuthorityError>>
		{
			if unlikely!(authoritative_or_authenticated_or_neither.is_authoritative_answer())
			{
				Err(AuthoritySectionError::AuthoritativeServersMustReturnAStartOfAuthorityRecord)
			}
			else
			{
				Ok(())
			}
		}
		
		let answer = match (answer_existence, answer_section_has_at_least_one_record_of_requested_data_type)
		{
			(AnswerExistence::NoError(_), true) => Answer::Answered,
			
			// RFC 2308, Section 2.2 No Data, paragraph 1: "NODATA is indicated by an answer with the RCODE set to NOERROR and no relevant answers in the answer section".
			//
			// NODATA is really an Empty Non-Terminal Name (ENT; see RFC 7719), ie a domain name with no records but that exists.
			(AnswerExistence::NoError(authoritative_or_authenticated_or_neither), false) =>
			{
				// RFC 2308, Section 2.2 No Data, paragraph 4: "It is possible to distinguish between a NODATA and a referral response by the presence of a SOA record in the authority section or the absence of NS records in the authority section".
				match (has_a_start_of_authority_record, has_name_server_records)
				{
					// Section 2.2 No Data NODATA RESPONSE: TYPE 1.
					(true, true) => Answer::NoData { response_type: NoDataResponseType1 { start_of_authority: self.start_of_authority_record.into_inner().unwrap(), name_servers: self.name_server_records.into_inner(), } },
					
					// Section 2.2 No Data NODATA RESPONSE: TYPE 2.
					(true, false) => Answer::NoData { response_type: NoDataResponseType2 { start_of_authority: self.start_of_authority_record.into_inner().unwrap() } },
					
					// Section 2.2 No Data NODATA RESPONSE: TYPE 3.
					(false, false) =>
					{
						guard_against_authoritative_answer_without_start_of_authority_record(authoritative_or_authenticated_or_neither);
						Answer::NoData { response_type: NoDataResponseType3 { name_servers: self.name_server_records.into_inner() } }
					},
					
					// Section 2.1 Name Error REFERRAL RESPONSE.
					// Section 2.2 No Data REFERRAL RESPONSE.
					(false, true) => Answer::Referral { name_servers: self.name_server_records.into_inner() },
				}
			}
			
			(AnswerExistence::NoDomain(_), true) => return Err(AuthoritySectionError::ResponseHadNoSuchDomainErrorCodeButContainsAnAnswer),
			
			// NXDOMAIN means that child domains will not exist (eg if example.com. is NXDOMAIN, then www.example.com. is NXDOMAIN).
			// This reply is for any QTYPE (eg A, AAAA, etc) that may later be queried for.
			(AnswerExistence::NoDomain(authoritative_or_authenticated_or_neither), false) =>
			{
				let most_canonical_name = self.canonical_name_chain.most_canonical_name().clone();
				
				// RFC 2308, Section 2.1 Name Error, paragraph 2: "It is possible to distinguish between a referral and a NXDOMAIN response by the presense of NXDOMAIN in the RCODE regardless of the presence of NS or SOA records in the authority section".
				match (has_a_start_of_authority_record, has_name_server_records)
				{
					// Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 1.
					(true, true) => Answer::NoDomain { response_type: NoDomainResponseType1 { start_of_authority: self.start_of_authority_record.into_inner().unwrap(), name_servers: self.name_server_records.into_inner(), }, most_canonical_name },
					
					// Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 2.
					(true, false) => Answer::NoDomain { response_type: NoDomainResponseType2 { start_of_authority: self.start_of_authority_record.into_inner().unwrap() }, most_canonical_name },
					
					// Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 3.
					(false, false) =>
					{
						guard_against_authoritative_answer_without_start_of_authority_record(authoritative_or_authenticated_or_neither);
						Answer::NoDomain { response_type: NoDomainResponseType3, most_canonical_name }
					},
					
					// Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 4.
					(false, true) =>
					{
						guard_against_authoritative_answer_without_start_of_authority_record(authoritative_or_authenticated_or_neither);
						Answer::NoDomain { response_type: NoDomainResponseType4 { name_servers: self.name_server_records.into_inner() }, most_canonical_name }
					},
				}
			}
		};
		
		Ok((answer, self.canonical_name_chain.into()))
	}
}
