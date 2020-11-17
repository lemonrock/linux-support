// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait QueryProcessor
{
	const DT: DataType;
	
	type PR<'message>: 'message + ParsedRecord;
	
	type RRV<'message>: ResourceRecordVisitor<'message, Error=(), Finished=OwnerNameToRecords<'message, Self::PR<'message>>>;
	
	fn new<'message>(query_name: &'message EfficientCaseFoldedName) -> Self::RRV<'message>;
	
	fn store_records_in_query_types_cache<'message>(query_types_cache: &mut QueryTypesCache, records: OwnerNameToRecordValue<Self::PR<'message>>);
	
	#[inline(always)]
	fn result<'message>(now: NanosecondsSinceUnixEpoch, query_name: &'message EfficientCaseFoldedName, domain_cache: &mut DomainCache, answer: Answer<Self::PR<'message>>, canonical_name_chain_records: CanonicalNameChainRecords, delegation_names: DelegationNames) -> Result<(), AnsweredError>
	{
		domain_cache.answered(now, query_name, domain_cache, answer, canonical_name_chain_records, delegation_names, Self::store_records_in_query_types_cache)
		
		
		
		// // TODO: This should update the no_domain_cache, as the domains now do exist!
		// domain_cache.cname_query_type_cache.put_present(canonical_name_chain_records);
		//
		// use self::Answer::*;
		// use self::NoDomainResponseType::*;
		// use self::NoDataResponseType::*;
		//
		// match answer
		// {
		// 	Answered =>
		// 	{
		// 		debug_assert!(!finished.is_empty());
		//
		// 		// TODO: This should update the no_domain_cache, as the domains now do exist!
		// 		Self::answered(finished, query_name, domain_cache);
		// 	}
		//
		// 	// Be careful here - all the records of the cname chain bar most_canonical_name do exist, and so domain presence needs to be updated for them!!!
		// 	NoDomain { response_type, most_canonical_name } =>
		// 	{
		// 		debug_assert!(finished.is_empty());
		//
		// 		match response_type
		// 		{
		// 			NoDomainResponseType1(authority_name_start_of_authority_name_servers) =>
		// 			{
		// 				domain_cache.no_domain_cache.put(most_canonical_name, authority_name_start_of_authority_name_servers.start_of_authority.0);
		//
		// 				// TODO: This should update the no_domain_cache, as the domains now do exist!
		// 				let mut present = PresentMultiple::default();
		// 				present.store_unprioritized_and_unweighted(authority_name_start_of_authority_name_servers.start_of_authority.0, authority_name_start_of_authority_name_servers.start_of_authority.1);
		// 				domain_cache.soa_query_type_cache.put_present_all_the_same_name(authority_name_start_of_authority_name_servers.authority_name.clone(), present);
		//
		// 				// TODO: This should update the no_domain_cache, as the domains now do exist!
		// 				domain_cache.ns_query_type_cache.put_present_all_the_same_name(authority_name_start_of_authority_name_servers.authority_name, authority_name_start_of_authority_name_servers.name_servers);
		//
		// 				// TODO: Referral.
		// 			}
		//
		// 			NoDomainResponseType2(authority_name_start_of_authority) =>
		// 			{
		// 				domain_cache.no_domain_cache.put(most_canonical_name, authority_name_start_of_authority_name_servers.start_of_authority.0);
		//
		// 				// TODO: This should update the no_domain_cache, as the domains now do exist!
		// 				let mut present = PresentMultiple::default();
		// 				present.store_unprioritized_and_unweighted(authority_name_start_of_authority_name_servers.start_of_authority.0, authority_name_start_of_authority_name_servers.start_of_authority.1);
		// 				domain_cache.soa_query_type_cache.put_present_all_the_same_name(authority_name_start_of_authority_name_servers.authority_name.clone(), present);
		// 			}
		//
		// 			NoDomainResponseType3 =>
		// 			{
		// 				// TODO: This should update the no_domain_cache, as the domains now do exist!
		// 				domain_cache.no_domain_cache.put_use_once_if_no_better_record(most_canonical_name, now)
		// 			}
		//
		// 			NoDomainResponseType4(authority_name_name_servers) =>
		// 			{
		// 				domain_cache.no_domain_cache.put_use_once_if_no_better_record(most_canonical_name, now);
		//
		// 				// TODO: This should update the no_domain_cache, as the domains now do exist!
		// 				domain_cache.ns_query_type_cache.put_present_all_the_same_name(authority_name_start_of_authority_name_servers.authority_name, authority_name_start_of_authority_name_servers.name_servers)
		//
		// 				// TODO: Referral.
		// 			}
		// 		}
		// 	}
		//
		// 	NoData { response_type, most_canonical_name } =>
		// 	{
		// 		debug_assert!(finished.is_empty());
		//
		// 		match response_type
		// 		{
		// 			NoDataResponseType1(authority_name_start_of_authority_name_servers) =>
		// 			{
		// 				// TODO: This should update the no_domain_cache, as the domains now do exist!
		// 				let mut present = PresentMultiple::default();
		// 				present.store_unprioritized_and_unweighted(authority_name_start_of_authority_name_servers.start_of_authority.0, authority_name_start_of_authority_name_servers.start_of_authority.1);
		// 				domain_cache.soa_query_type_cache.put_present_all_the_same_name(authority_name_start_of_authority_name_servers.authority_name.clone(), present);
		//
		// 				// TODO: This should update the no_domain_cache, as the domains now do exist!
		// 				domain_cache.ns_query_type_cache.put_present_all_the_same_name(authority_name_start_of_authority_name_servers.authority_name, authority_name_start_of_authority_name_servers.name_servers);
		//
		//
		//
		// 				// TODO: Referral.
		// 			}
		//
		// 			NoDataResponseType2(authority_name_start_of_authority) =>
		// 			{
		// 				// TODO: This should update the no_domain_cache, as the domains now do exist!
		// 				let mut present = PresentMultiple::default();
		// 				present.store_unprioritized_and_unweighted(authority_name_start_of_authority_name_servers.start_of_authority.0, authority_name_start_of_authority_name_servers.start_of_authority.1);
		// 				domain_cache.soa_query_type_cache.put_present_all_the_same_name(authority_name_start_of_authority_name_servers.authority_name.clone(), present);
		// 			}
		//
		// 			NoDataResponseType3 =>
		// 			{
		//
		// 			}
		// 		}
		// 	}
		//
		// 	Referral { referral, most_canonical_name } =>
		// 	{
		// 		debug_assert!(finished.is_empty());
		//
		// 		// TODO: Referral.
		// 	}
		// }
	}
}
