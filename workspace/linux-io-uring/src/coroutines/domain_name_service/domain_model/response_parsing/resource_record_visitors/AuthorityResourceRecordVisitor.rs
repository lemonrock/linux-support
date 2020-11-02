// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct AuthorityResourceRecordVisitor<'message, 'cache: 'message>
{
	canonical_name_chain: CanonicalNameChain<'message, 'cache>,
	
	authority_name: RefCell<Option<CaseFoldedName<'cache>>>,
	
	/// *MUST* be for the parent of the final entry in the canonical name chain.
	/// It is valid to have no records.
	start_of_authority: RefCell<Option<PresentSolitary<StartOfAuthority<'cache, CaseFoldedName<'cache>>>>>,
	
	/// *MUST* be for the parent of the final entry in the canonical name chain.
	/// It is valid to have no records.
	/// However, all records will have the same name (the parent of the final entry in the canonical name chain).
	name_servers: RefCell<Option<PresentMultiple<CaseFoldedName<'cache>>>>,
}

impl<'message, 'cache: 'message> ResourceRecordVisitor<'message> for AuthorityResourceRecordVisitor<'message, 'cache>
{
	type Error = AuthorityError;
	
	type Finished = Self;
	
	#[inline(always)]
	fn finished(self) -> Self::Finished
	{
		self
	}
	
	#[inline(always)]
	fn NS(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: ParsedName<'message>) -> Result<(), Self::Error>
	{
		use self::AuthorityError::*;
		
		if unlikely!(self.canonical_name_chain.validate_authority_section_name(&name))
		{
			return Err(NameServerRecordInAuthoritySectionIsNotForFinalNameInCanonicalNameChain)
		}
		self.store_authority_name(name);
		
		let mut name_server_records = self.name_servers.borrow_mut();
		let name_server_records = name_server_records.deref_mut();
		if unlikely!(name_server_records.is_none())
		{
			*name_server_records = Some(PresentMultiple::default());
		}
		
		let present = name_server_records.as_mut().unwrap();
		present.store_unprioritized_and_unweighted(cache_until, CaseFoldedName::from(record));

		Ok(())
	}

	#[inline(always)]
	fn SOA(&mut self, name: ParsedName<'message>, negative_cache_until: NegativeCacheUntil, record: StartOfAuthority<'message, ParsedName<'message>>) -> Result<(), Self::Error>
	{
		use self::AuthorityError::*;
		
		if unlikely!(self.canonical_name_chain.validate_authority_section_name(&name))
		{
			return Err(StartOfAuthorityRecordInAuthoritySectionIsNotForFinalNameInCanonicalNameChain)
		}
		self.store_authority_name(name);
		
		let mut start_of_authority_record = self.start_of_authority.borrow_mut();
		let start_of_authority_record = start_of_authority_record.deref_mut();
		if likely!(start_of_authority_record.is_none())
		{
			*start_of_authority_record = Some
			(
				{
					use self::PresentSolitary::*;
					let record = record.into();
					match negative_cache_until
					{
						None => UseOnce
						{
							record,
						},
						
						Some(cached_until) => Cached
						{
							cached_until,
							
							record,
						}
					}
				}
			);
			
			Ok(())
		}
		else
		{
			Err(MoreThanOneStartOfAuthorityRecordInAuthoritySection)
		}
	}
}

impl<'message, 'cache: 'message> AuthorityResourceRecordVisitor<'message, 'cache>
{
	#[inline(always)]
	pub(crate) fn new(canonical_name_chain: CanonicalNameChain<'message, 'cache>) -> Self
	{
		Self
		{
			canonical_name_chain,
			authority_name: RefCell::new(None),
			name_servers: RefCell::new(None),
			start_of_authority: RefCell::new(None)
		}
	}
	
	#[inline(always)]
	fn store_authority_name(&self, authority_name: ParsedName<'message>)
	{
		let mut authority_name_borrowed = self.authority_name.borrow_mut();
		let authority_name_borrowed = authority_name_borrowed.deref_mut();
		if likely!(authority_name_borrowed.is_none())
		{
			*authority_name_borrowed = Some(CaseFoldedName::from(authority_name));
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
	pub(crate) fn answer(self, answer_existence: AnswerExistence, answer_section_has_at_least_one_record_of_requested_data_type: bool) -> Result<(Answer<'cache>, Records<'cache, CaseFoldedName<'cache>>), AuthoritySectionError<AuthorityError>>
	{
		use self::AnswerExistence::*;
		use self::Answer::*;
		use self::NoDataResponseType::*;
		use self::NoDomainResponseType::*;
		
		let has_a_start_of_authority_record = self.start_of_authority.borrow().is_some();
		let has_name_server_records = !self.name_servers.borrow().is_some();
		
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
			(NoError(_), true) => Answer::Answered,
			
			// RFC 2308, Section 2.2 No Data, paragraph 1: "NODATA is indicated by an answer with the RCODE set to NOERROR and no relevant answers in the answer section".
			//
			// NODATA is really an Empty Non-Terminal Name (ENT; see RFC 7719), ie a domain name with no records but that exists.
			(NoError(authoritative_or_authenticated_or_neither), false) =>
			{
				let most_canonical_name = self.most_canonical_name();
				
				match (has_a_start_of_authority_record, has_name_server_records)
				{
					(true, true) => NoData
					{
						response_type: NoDataResponseType1
						(
							AuthorityNameStartOfAuthorityNameServers
							{
								authority_name_start_of_authority: AuthorityNameStartOfAuthority
								{
									authority_name: Self::authority_name(self.authority_name),
									
									start_of_authority: Self::start_of_authority(self.start_of_authority),
								},
								
								name_servers: Self::name_servers(self.name_servers)
							}
						),
						most_canonical_name
					},
					
					(true, false) => NoData
					{
						response_type: NoDataResponseType2
						(
							AuthorityNameStartOfAuthority
							{
								authority_name: Self::authority_name(self.authority_name),
								
								start_of_authority: Self::start_of_authority(self.start_of_authority)
							}
						),
						most_canonical_name
					},
					
					(false, false) =>
					{
						guard_against_authoritative_answer_without_start_of_authority_record(authoritative_or_authenticated_or_neither)?;
						NoData
						{
							response_type: NoDataResponseType3,
							
							most_canonical_name
						}
					},
					
					// RFC 2308, Section 2.2 No Data, paragraph 4: "It is possible to distinguish between a NODATA and a referral response by the presence of a SOA record in the authority section or the absence of NS records in the authority section".
					(false, true) => Referral
					{
						referral: AuthorityNameNameServers
						{
							authority_name: Self::authority_name(self.authority_name),
							
							name_servers: Self::name_servers(self.name_servers)
						},
						
						most_canonical_name
					},
				}
			}
			
			(NoSuchDomain(_), true) => return Err(AuthoritySectionError::ResponseHadNoSuchDomainErrorCodeButContainsAnAnswer),
			
			// NXDOMAIN means that child domains will not exist (eg if example.com. is NXDOMAIN, then www.example.com. is NXDOMAIN).
			// This reply is for any QTYPE (eg A, AAAA, etc) that may later be queried for.
			(NoSuchDomain(authoritative_or_authenticated_or_neither), false) =>
			{
				let most_canonical_name = self.most_canonical_name();
				
				// RFC 2308, Section 2.1 Name Error, paragraph 2: "It is possible to distinguish between a referral and a NXDOMAIN response by the presense of NXDOMAIN in the RCODE regardless of the presence of NS or SOA records in the authority section".
				match (has_a_start_of_authority_record, has_name_server_records)
				{
					(true, true) => NoDomain
					{
						response_type: NoDomainResponseType1
						(
							AuthorityNameStartOfAuthorityNameServers
							{
								authority_name_start_of_authority: AuthorityNameStartOfAuthority
								{
									authority_name: Self::authority_name(self.authority_name),
									
									start_of_authority: Self::start_of_authority(self.start_of_authority),
								},
								
								name_servers: Self::name_servers(self.name_servers)
							}
						),
						most_canonical_name
					},
					
					// Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 2.
					(true, false) => NoDomain
					{
						response_type: NoDomainResponseType2
						(
							AuthorityNameStartOfAuthority
							{
								authority_name: Self::authority_name(self.authority_name),
								start_of_authority: Self::start_of_authority(self.start_of_authority)
							}
						),
						most_canonical_name
					},
					
					// Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 3.
					(false, false) =>
					{
						guard_against_authoritative_answer_without_start_of_authority_record(authoritative_or_authenticated_or_neither)?;
						NoDomain
						{
							response_type: NoDomainResponseType3,
							most_canonical_name
						}
					},
					
					// Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 4.
					(false, true) =>
					{
						guard_against_authoritative_answer_without_start_of_authority_record(authoritative_or_authenticated_or_neither)?;
						NoDomain
						{
							response_type: NoDomainResponseType4
							(
								AuthorityNameNameServers
								{
									authority_name: Self::authority_name(self.authority_name),
									name_servers: Self::name_servers(self.name_servers)
								}
							),
							most_canonical_name
						}
					},
				}
			}
		};
		
		let canonical_records = self.canonical_name_chain.cname_records;
		Ok((answer, canonical_records))
	}
	
	#[inline(always)]
	fn most_canonical_name(&self) -> CaseFoldedName<'cache>
	{
		CaseFoldedName::from(self.canonical_name_chain.most_canonical_name())
	}
	
	#[inline(always)]
	fn authority_name(authority_name: RefCell<Option<CaseFoldedName<'cache>>>) -> CaseFoldedName<'cache>
	{
		authority_name.into_inner().unwrap()
	}
	
	#[inline(always)]
	fn name_servers(name_servers: RefCell<Option<PresentMultiple<CaseFoldedName<'cache>>>>) -> PresentMultiple<CaseFoldedName<'cache>>
	{
		name_servers.into_inner().unwrap()
	}
	
	#[inline(always)]
	fn start_of_authority(start_of_authority: RefCell<Option<PresentSolitary<StartOfAuthority<'cache, CaseFoldedName<'cache>>>>>) -> PresentSolitary<StartOfAuthority<'cache, CaseFoldedName<'cache>>>
	{
		start_of_authority.into_inner().unwrap()
	}
}
