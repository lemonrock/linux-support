// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This value will have been validated to be correct for the Service Field.
///
/// The service field value is emtpy (`""`) for this to occur.
///
/// Used by the following enum members of `ServiceField`:-
///
/// * `NonTerminalAndEmpty`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RegularExpressionResolvingToDomainNameOrQueryNaptrResourceRecord<'label, N: Name<'label, TypeEquality=TE>, CS: CharacterString<TypeEquality=TE>, TE: OwnedOrParsedTypeEquality>
{
	/// A regular expression that resolves to an domain name.
	///
	/// This is very unlikely; known examples are for `dig NAPTR *`:-
	///
	/// * `ftp.uri.arpa`.
	/// * `http.uri.arpa`.
	/// * `mailto.uri.arpa`.
	/// * `urn.uri.arpa`.
	UnvalidatedRegularExpression(CS),
	
	/// A domain name to query for a `NAPTR` record.
	///
	/// This is very likely.
	///
	/// This also includes the only known `urn.arpa` domain, `pin.urn.arpa` (which resolves to a non-existent domain).
	DomainName((N, PhantomData<&'label ()>)),
}

impl<'message> Into<RegularExpressionResolvingToDomainNameOrQueryNaptrResourceRecord<'static, EfficientCaseFoldedName, OwnedCharacterString>> for RegularExpressionResolvingToDomainNameOrQueryNaptrResourceRecord<'message, ParsedName<'message>, ParsedCharacterString<'message>>
{
	#[inline(always)]
	fn into(self) -> RegularExpressionResolvingToDomainNameOrQueryNaptrResourceRecord<'static, EfficientCaseFoldedName, OwnedCharacterString>
	{
		use self::RegularExpressionResolvingToDomainNameOrQueryNaptrResourceRecord::*;
		
		match self
		{
			UnvalidatedRegularExpression(regular_expression) => UnvalidatedRegularExpression(OwnedCharacterString::from(regular_expression)),
			
			DomainName(domain_name) => DomainName((EfficientCaseFoldedName::from(domain_name), PhantomData)),
		}
	}
}

impl<'message> RegularExpressionResolvingToDomainNameOrQueryNaptrResourceRecord<'message, ParsedName<'message>, ParsedCharacterString<'message>>
{
	#[inline(always)]
	fn parse(replacement_domain_name_or_raw_regular_expression: Either<ParsedName<'message>, ParsedCharacterString<'message>>, mutually_exclusive_flag: Option<NamingAuthorityMutuallyExclusiveFlag>) -> Result<Self, IgnoredServiceFieldReason>
	{
		use self::IgnoredServiceFieldReason::*;
		use self::NamingAuthorityMutuallyExclusiveFlag::*;
		use self::RegularExpressionResolvingToDomainNameOrQueryNaptrResourceRecord::*;
		use self::ServiceFieldKind::NonTerminalAndEmpty;
		
		match (replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)
		{
			(Left(domain_name), None) => Ok(DomainName((domain_name, PhantomData))),
			
			(Left(_), _) => Err(InvalidCombinationOfDomainNameAndFlag(NonTerminalAndEmpty, mutually_exclusive_flag)),
			
			(Right(unvalidated_regular_expression), None) => Ok(UnvalidatedRegularExpression(unvalidated_regular_expression)),
			
			(Right(_), _) => Err(InvalidCombinationOfRegularExpressionAndFlag(NonTerminalAndEmpty, mutually_exclusive_flag)),
		}
	}
}
