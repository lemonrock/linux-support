// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This value will have been validated to be correct for the Service Field.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Replacement<'label, N: Name<'label, TypeEquality=TE>, OOPU: OwnedOrParsedUri<TypeEquality=TE>, CS: CharacterString<TypeEquality=TE>, TE: OwnedOrParsedTypeEquality>
{
	/// A domain name.
	DomainName(N),

	/// An URI.
	///
	/// The scheme will have been validated to match the transport protocol or restrictions of the service field.
	UniformResourceIdentifier(OOPU),

	/// This means that the service field should expect a regular expression, but the regular expression pattern has not been validated.
	UnvalidatedRegularExpression(CS),
}

impl<'message> Into<Replacement<'static, EfficientCaseFoldedName, OwnedUri, OwnedCharacterString>> for Replacement<'message, ParsedName<'message>, ParsedUri<'message>, ParsedCharacterString<'message>>
{
	#[inline(always)]
	fn into(self) -> Replacement<'static, EfficientCaseFoldedName, OwnedUri, OwnedCharacterString>
	{
		use self::Replacement::*;
		
		match self
		{
			DomainName(domain_name) => DomainName(EfficientCaseFoldedName::from(domain_name)),
			
			UniformResourceIdentifier(uri) => UniformResourceIdentifier(OwnedUri::from(uri)),
			
			UnvalidatedRegularExpression(regular_expression) => UnvalidatedRegularExpression(OwnedCharacterString::from(regular_expression)),
		}
	}
}

impl<'message> Replacement<'message, ParsedName<'message>, ParsedUri<'message>, ParsedCharacterString<'message>, ParsedTypeEquality>
{
	#[inline(always)]
	pub(crate) fn from_replacement_domain_name_or_raw_regular_expression(replacement_domain_name_or_raw_regular_expression: Either<ParsedName<'message>, ParsedCharacterString<'message>>) -> Self
	{
		use self::Replacement::*;
		
		match replacement_domain_name_or_raw_regular_expression
		{
			Left(domain_name) => DomainName(domain_name),
			
			Right(regular_expression) => UnvalidatedRegularExpression(regular_expression),
		}
	}
}
