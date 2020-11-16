// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This value will have been validated to be correct for the Service Field.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UriOrQueryUriResourceRecord<'label, N: Name<'label, TypeEquality=TE>, OOPU: OwnedOrParsedUri<TypeEquality=TE>, TE: OwnedOrParsedTypeEquality>
{
	/// An URI.
	///
	/// This will be the case if a `U` flag was present.
	///
	/// The scheme will have been validated to match the transport protocol or restrictions of the service field.
	UniformResourceIdentifier(OOPU),
	
	/// A domain name to query for a `URI` record.
	///
	/// This will be the case if a `D` flag was present.
	///
	/// This is very unlikely.
	DomainName((N, PhantomData<&'label ()>)),
}

impl<'message> Into<Replacement<'static, EfficientCaseFoldedName, OwnedUri>> for UriOrQueryUriResourceRecord<'message, ParsedName<'message>, ParsedUri<'message>>
{
	#[inline(always)]
	fn into(self) -> UriOrQueryUriResourceRecord<'static, EfficientCaseFoldedName, OwnedUri>
	{
		use self::UriOrQueryUriResourceRecord::*;
		
		match self
		{
			UniformResourceIdentifier(uri) => UniformResourceIdentifier(OwnedUri::from(uri)),
			
			DomainName(domain_name) => DomainName((EfficientCaseFoldedName::from(domain_name), PhantomData)),
		}
	}
}

impl<'message> UriOrQueryUriResourceRecord<'message, ParsedName<'message>, ParsedUri<'message>>
{
	#[inline(always)]
	fn parse_SessionInitiationProtocolUserAgentConfiguration(replacement_domain_name_or_raw_regular_expression: Either<ParsedName<'message>, ParsedCharacterString<'message>>, mutually_exclusive_flag: Option<NamingAuthorityMutuallyExclusiveFlag>) -> Result<Self, IgnoredServiceFieldReason>
	{
		Self::parse(ServiceFieldKind::SessionInitiationProtocolUserAgentConfiguration, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, |service_field_kind, unvalidated_regular_expression| Self::u_naptr_regular_expression_uri(service_field_kind, unvalidated_regular_expression, Some(HypertextTransportProtocol::https)))
	}
	
	#[inline(always)]
	fn parse_NoSolicit(replacement_domain_name_or_raw_regular_expression: Either<ParsedName<'message>, ParsedCharacterString<'message>>, mutually_exclusive_flag: Option<NamingAuthorityMutuallyExclusiveFlag>) -> Result<Self, IgnoredServiceFieldReason>
	{
		Self::parse(ServiceFieldKind::NoSolicit, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, |service_field_kind, unvalidated_regular_expression| Self::no_solicit_regular_expression_uri(service_field_kind, unvalidated_regular_expression))
	}
	
	#[inline(always)]
	fn parse_CentralizedConferencing(replacement_domain_name_or_raw_regular_expression: Either<ParsedName<'message>, ParsedCharacterString<'message>>, mutually_exclusive_flag: Option<NamingAuthorityMutuallyExclusiveFlag>) -> Result<Self, IgnoredServiceFieldReason>
	{
		Self::parse(ServiceFieldKind::CentralizedConferencing, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, |service_field_kind, unvalidated_regular_expression| Self::u_naptr_regular_expression_uri(service_field_kind, unvalidated_regular_expression, None))
	}
	
	#[inline(always)]
	fn parse_LocalLocationInformationServer(replacement_domain_name_or_raw_regular_expression: Either<ParsedName<'message>, ParsedCharacterString<'message>>, mutually_exclusive_flag: Option<NamingAuthorityMutuallyExclusiveFlag>) -> Result<Self, IgnoredServiceFieldReason>
	{
		Self::parse(ServiceFieldKind::LocalLocationInformationServer, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, |service_field_kind, unvalidated_regular_expression| Self::u_naptr_regular_expression_uri(service_field_kind, unvalidated_regular_expression, None))
	}
	
	#[inline(always)]
	fn parse_LocationToServiceTranslationProtocol(replacement_domain_name_or_raw_regular_expression: Either<ParsedName<'message>, ParsedCharacterString<'message>>, mutually_exclusive_flag: Option<NamingAuthorityMutuallyExclusiveFlag>, transport_protocol: Option<HypertextTransportProtocol>) -> Result<Self, IgnoredServiceFieldReason>
	{
		Self::parse(ServiceFieldKind::LocationToServiceTranslationProtocol, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, |service_field_kind, unvalidated_regular_expression| Self::u_naptr_regular_expression_uri(service_field_kind, unvalidated_regular_expression, transport_protocol))
	}
	
	#[inline(always)]
	fn parse_ApplicationLayerTrafficOptimization(replacement_domain_name_or_raw_regular_expression: Either<ParsedName<'message>, ParsedCharacterString<'message>>, mutually_exclusive_flag: Option<NamingAuthorityMutuallyExclusiveFlag>, transport_protocol: Option<HypertextTransportProtocol>) -> Result<Self, IgnoredServiceFieldReason>
	{
		Self::parse(ServiceFieldKind::ApplicationLayerTrafficOptimization, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, |service_field_kind, unvalidated_regular_expression| Self::u_naptr_regular_expression_uri(service_field_kind, unvalidated_regular_expression, transport_protocol))
	}
	
	#[inline(always)]
	fn parse(service_field_kind: ServiceFieldKind, replacement_domain_name_or_raw_regular_expression: Either<ParsedName<'message>, ParsedCharacterString<'message>>, mutually_exclusive_flag: Option<NamingAuthorityMutuallyExclusiveFlag>, regular_expression_parser: impl FnOnce(ServiceFieldKind, ParsedCharacterString<'message>) -> Result<URI<'message>, IgnoredServiceFieldReason>) -> Result<Self, IgnoredServiceFieldReason>
	{
		use self::IgnoredServiceFieldReason::*;
		use self::NamingAuthorityMutuallyExclusiveFlag::*;
		use self::UriOrQueryUriResourceRecord::*;
		
		debug_assert_ne!(service_field_kind, ServiceFieldKind::NonEmptyTerminal);
		
		match (replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)
		{
			(Left(domain_name), Some(D)) => Ok(DomainName((domain_name, PhantomData))),
			
			(Left(_), _) => Err(InvalidCombinationOfDomainNameAndFlag(service_field_kind, mutually_exclusive_flag)),
			
			(Right(unvalidated_regular_expression), Some(U)) => regular_expression_parser(service_field_kind, unvalidated_regular_expression).map(|| UriOrQueryUriResourceRecord::UniformResourceIdentifier(ParsedUri::from(target_uri))),
			
			(Right(_), _) => Err(InvalidCombinationOfRegularExpressionAndFlag(service_field_kind, mutually_exclusive_flag)),
		}
	}
	
	/// See RFC 4095, Section 2 The No-Solicit NAPTR Application, page 4.
	/// If `valid_protocol` is `None`, then both `HTTP` and `HTTPS` are considered valid.
	fn no_solicit_regular_expression_uri(service_field_kind: ServiceFieldKind, unvalidated_regular_expression: ParsedCharacterString<'message>) -> Result<URI<'message>, IgnoredServiceFieldReason>
	{
		use self::IgnoredServiceFieldReason::*;
		
		// At least 3 bytes long for 3 delimiter characters.
		const DelimiterCharacterLength: usize = 1;
		const MinimumRegularExpressionLength: usize = DelimiterCharacterLength + DelimiterCharacterLength + DelimiterCharacterLength;
		
		let regular_expression_length = unvalidated_regular_expression.len();
		
		if unlikely!(regular_expression_length < MinimumRegularExpressionLength)
		{
			return Err(ExpectedANoSolicitRegularExpression)
		}
		
		let first_delimiter_character = unvalidated_regular_expression.get_unchecked_value_safe(0);
		let second_delimiter_character = unvalidated_regular_expression.get_unchecked_value_safe(1);
		let last_delimiter_character = unvalidated_regular_expression.get_unchecked_value_safe(regular_expression_length - 1);
		
		if unlikely!(first_delimiter_character != second_delimiter_character || second_delimiter_character != last_delimiter_character)
		{
			return Err(ExpectedANoSolicitRegularExpressionToHaveTheSameDelimiterCharacter { first_delimiter_character, second_delimiter_character, last_delimiter_character })
		}
		
		match first_delimiter_character
		{
			b'/' | b'!' => (),
			b'1' ..= b'9' | b'i' => return Err(NoSolicitRegularExpressionHasAnInvalidDelimiterCharacter(first_delimiter_character)),
			_ => (),
		};
		
		let target_uri = URI::try_from(&unvalidated_regular_expression[(DelimiterCharacterLength + DelimiterCharacterLength) .. (regular_expression_length - DelimiterCharacterLength)]).map_err(|error| InvalidTargetUri(service_field_kind, error))?;
		
		use self::Scheme::*;
		match target_uri.scheme()
		{
			&HTTP => (),
			&HTTPS => (),
			&FTP => (),
			other @ _ => return Err(NoSolicitRegularExpressionUriIsNotHttpOrHttpsOrFtp(other.into_owned())),
		};
		
		Ok(target_uri)
	}
	
	/// See RFC 4848, Section 2.2 Permitted Regular Expressions.
	/// If `valid_protocol` is `None`, then both `HTTP` and `HTTPS` are considered valid.
	fn u_naptr_regular_expression_uri(service_field_kind: ServiceFieldKind, unvalidated_regular_expression: ParsedCharacterString<'message>, transport_protocol: Option<HypertextTransportProtocol>) -> Result<URI<'message>, IgnoredServiceFieldReason>
	{
		use self::IgnoredServiceFieldReason::*;
		
		// `u-naptr-regexp = "!.*!"<URI>"!"`.
		const Prefix: &'static [u8] = b"!.*!";
		const Suffix: &'static [u8] = b"!";
		const PrefixLength: usize = Prefix.len();
		const SuffixLength: usize = Suffix.len();
		
		const MinimumRegularExpressionLength: usize = PrefixLength + SuffixLength;
		
		let regular_expression_length = unvalidated_regular_expression.len();
		
		if unlikely!(regular_expression_length < MinimumRegularExpressionLength)
		{
			return Err(ExpectedAnUNaptrRegularExpression(service_field_kind))
		}
		
		if unlikely!(&unvalidated_regular_expression[0 .. PrefixLength] != Prefix)
		{
			return Err(UNaptrRegularExpressionDoesNotStartWithCorrectPrefix(service_field_kind))
		}
		
		let suffix_starts_at_index = (regular_expression_length - SuffixLength);
		if unlikely!(&unvalidated_regular_expression[suffix_starts_at_index ..] != Suffix)
		{
			return Err(UNaptrRegularExpressionDoesNotEndWithCorrectSuffix(service_field_kind))
		}
		
		let target_uri = URI::try_from(&unvalidated_regular_expression[PrefixLength .. suffix_starts_at_index]).map_err(|error| InvalidTargetUri(service_field_kind, error))?;
		
		use self::Scheme::*;
		use self::HypertextTransportProtocol::*;
		match (transport_protocol, target_uri.scheme())
		{
			(None, &HTTP) => (),
			(None, &HTTPS) => (),
			(None, other @ _) => return Err(UNaptrRegularExpressionUriIsNotHttpOrHttps(service_field_kind, other.into_owned())),
			
			(Some(http), &HTTP) => (),
			(Some(http), other @ _) => return Err(UNaptrRegularExpressionUriIsNotHttp(service_field_kind, other.into_owned())),
			
			(Some(https), &HTTPS) => (),
			(Some(https), other @ _) => return Err(UNaptrRegularExpressionUriIsNotHttps(service_field_kind, other.into_owned())),
		};
		
		Ok(target_uri)
	}
}
