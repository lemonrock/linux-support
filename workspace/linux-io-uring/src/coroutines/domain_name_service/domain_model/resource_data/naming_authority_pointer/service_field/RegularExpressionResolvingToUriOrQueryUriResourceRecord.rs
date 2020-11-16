// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This value will have been validated to be correct for the Service Field.
///
/// Used by the following enum members of `ServiceField`:-
///
/// * `Enum`.
/// * `BusinessDocumentMetadataServiceLocation`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RegularExpressionResolvingToUriOrQueryUriResourceRecord<'label, N: Name<'label, TypeEquality=TE>, CS: CharacterString<TypeEquality=TE>, TE: OwnedOrParsedTypeEquality>
{
	/// A regular expression that resolves to an URI.
	UnvalidatedRegularExpression(CS),
	
	/// A domain name to query for a `URI` record.
	///
	/// This will be the case if a `D` flag was present.
	///
	/// This is very unlikely.
	DomainName((N, PhantomData<&'label ()>)),
}

impl<'message> Into<RegularExpressionResolvingToUriOrQueryUriResourceRecord<'static, EfficientCaseFoldedName, OwnedCharacterString>> for RegularExpressionResolvingToUriOrQueryUriResourceRecord<'message, ParsedName<'message>, ParsedCharacterString<'message>>
{
	#[inline(always)]
	fn into(self) -> RegularExpressionResolvingToUriOrQueryUriResourceRecord<'static, EfficientCaseFoldedName, OwnedCharacterString>
	{
		use self::RegularExpressionResolvingToUriOrQueryUriResourceRecord::*;
		
		match self
		{
			UnvalidatedRegularExpression(regular_expression) => UnvalidatedRegularExpression(OwnedCharacterString::from(regular_expression)),
			
			DomainName(domain_name) => DomainName((EfficientCaseFoldedName::from(domain_name), PhantomData)),
		}
	}
}

impl<'message> RegularExpressionResolvingToUriOrQueryUriResourceRecord<'message, ParsedName<'message>, ParsedCharacterString<'message>>
{
	#[inline(always)]
	fn parse(service_field_kind: ServiceFieldKind, replacement_domain_name_or_raw_regular_expression: Either<ParsedName<'message>, ParsedCharacterString<'message>>, mutually_exclusive_flag: Option<NamingAuthorityMutuallyExclusiveFlag>) -> Result<Self, IgnoredServiceFieldReason>
	{
		use self::IgnoredServiceFieldReason::*;
		use self::NamingAuthorityMutuallyExclusiveFlag::*;
		use self::RegularExpressionResolvingToUriOrQueryUriResourceRecord::*;
		
		match (replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)
		{
			(Left(domain_name), Some(D)) => Ok(DomainName((domain_name, PhantomData))),
			
			(Left(_), _) => Err(InvalidCombinationOfDomainNameAndFlag(service_field_kind, mutually_exclusive_flag)),
			
			(Right(unvalidated_regular_expression), Some(U)) => Ok(UnvalidatedRegularExpression(unvalidated_regular_expression)),
			
			(Right(_), _) => Err(InvalidCombinationOfRegularExpressionAndFlag(service_field_kind, mutually_exclusive_flag)),
		}
	}
}
