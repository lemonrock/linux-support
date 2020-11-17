// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This value will have been validated to be correct for the Service Field.
///
/// Used by the following enum members of `ServiceField`:-
///
/// * `SessionInitiationProtocol`.
/// * `Diameter`.
/// * `Radius`.
/// * `InternetRegistryInformationService`.
/// * `TraversalUsingRelaysAroundNetworkAddressTranslation`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryResourceRecord<N: Name>
{
	/// Domain name.
	pub domain_name: N,
	
	/// What to query for next at `domain_name`.
	pub query_for_next: QueryForNext,
}

impl<'message> Into<QueryResourceRecord<EfficientCaseFoldedName>> for QueryResourceRecord<ParsedName<'message>>
{
	#[inline(always)]
	fn into(self) -> QueryResourceRecord<EfficientCaseFoldedName>
	{
		QueryResourceRecord
		{
			domain_name: EfficientCaseFoldedName::from(self.domain_name),
			
			query_for_next: self.query_for_next,
		}
	}
}

impl<'message> QueryResourceRecord<ParsedName<'message>>
{
	#[inline(always)]
	fn new(domain_name: ParsedName<'message>, query_for_next: QueryForNext) -> Result<Self, IgnoredServiceFieldReason>
	{
		Ok
		(
			Self
			{
				domain_name,
				query_for_next,
			}
		)
	}
	
	#[inline(always)]
	fn parse(service_field_kind: ServiceFieldKind, replacement_domain_name_or_raw_regular_expression: Either<ParsedName<'message>, ParsedCharacterString<'message>>, mutually_exclusive_flag: Option<NamingAuthorityMutuallyExclusiveFlag>) -> Result<Self, IgnoredServiceFieldReason>
	{
		use self::IgnoredServiceFieldReason::*;
		use self::NamingAuthorityMutuallyExclusiveFlag::*;
		
		match (replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)
		{
			(Left(domain_name), Some(S)) => Self::new(domain_name, QueryForNext::SRV),
			
			(Left(domain_name), Some(A)) => Self::new(domain_name, QueryForNext::A),
			
			(Left(domain_name), Some(D)) => Self::new(domain_name, QueryForNext::URI),
			
			(Left(domain_name), _) => Err(InvalidCombinationOfDomainNameAndFlag(service_field_kind, mutually_exclusive_flag)),
			
			(Right(_), _) => Err(InvalidCombinationOfRegularExpressionAndFlag(service_field_kind, mutually_exclusive_flag)),
		}
	}
}
