// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A naming authority pointer record.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamingAuthorityPointer<'label, N: Name<'label, TypeEquality=TE>, OOPU: OwnedOrParsedUri<TypeEquality=TE>, CS: CharacterString<TypeEquality=TE>, TE: OwnedOrParsedTypeEquality>
{
	/// Mutually exclusive flag, if any.
	pub mutually_exclusive_flag: Option<NamingAuthorityMutuallyExclusiveFlag>,

	/// Protocol.
	///
	/// This has been validated in accordance RFC 2915, Section 2 NAPTR RR Format, Service, paragraph 1: "A protocol MUST be specified if the flags field states that the NAPTR is terminal".
	///
	/// Consequently, if a terminal mutually exclusive flag is specified in `mutually_exclusive_flag` (viz, `mutually_exclusive_flag` is one of `Some(S)`, `Some(A)` or `Some(U)`), this will be `Some()`.
	pub protocol: Option<CaseFoldedNamingAuthorityProtocol>,

	/// Can be empty.
	pub resolution_services: HashSet<CaseFoldedNamingAuthorityResolutionService>,
	
	/// `domain_name` is `Either::Left`.
	///
	/// `regular_expression` is `Either::Right`:-
	///
	/// * is validated to not be empty.
	/// * is validated to be up to 255 bytes long.
	/// * does not have its syntax validated (although this may change in the future).
	pub replacement: Replacement<'label, N, OOPU, CS, TE>,
}

impl<'message> Into<NamingAuthorityPointer<'static, EfficientCaseFoldedName, OwnedUri, OwnedCharacterString>> for NamingAuthorityPointer<'message, ParsedName<'message>, ParsedUri<'message>, ParsedCharacterString<'message>>
{
	#[inline(always)]
	fn into(self) -> NamingAuthorityPointer<'static, EfficientCaseFoldedName, OwnedCharacterString>
	{
		NamingAuthorityPointer
		{
			mutually_exclusive_flag: self.mutually_exclusive_flag,
			protocol: self.protocol,
			resolution_services: self.resolution_services,
			replacement: replacement.into(),
		}
	}
}
