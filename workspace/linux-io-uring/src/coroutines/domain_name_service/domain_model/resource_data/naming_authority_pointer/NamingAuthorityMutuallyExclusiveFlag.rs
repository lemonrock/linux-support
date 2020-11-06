// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A mutually exclusive flag.
///
/// See RFC 2915, Section NAPTR RR Format, Flags, paragraph 2.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NamingAuthorityMutuallyExclusiveFlag
{
	/// The next look up should be for `SRV` records using a target domain name.
	/// The presence of this flag defines a `S-NAPTR`, properly known as "Straightforward NAPTR".
	///
	/// There should be a target domain name present.
	/// There should be no regular expression present (in RFC 3958, Section 2.2.3 Terminal and Non-terminal NAPTR Records, this means that the replacement field should be a target domain name and used to look up a `SRV` record set).
	/// The target name should be one suitable for a `SRV`, ie if the origin is `example.com`, then the target name should be, say, `_http._tcp.example.com`.
	///
	/// Most recently defined by RFC 3598, Section 6.4 Flags.
	/// Previously defined by RFC 2915 and RFC 3404, Section 4.3 Flags.
	S,
	
	/// The next look up should be for `A` or `AAAA` records.
	/// The presence of this flag defines a `S-NAPTR`, properly known as "Straightforward NAPTR".
	///
	/// There should be a target domain name present.
	/// There should be no regular expression present (in RFC 3958, Section 2.2.3 Terminal and Non-terminal NAPTR Records, this means that the replacement field should be a target domain name and used to look up a `A` or `AAAA` record set; the default port number for the service should be assumed).
	/// The target name should be one suitable for a `A`, ie if the origin is `example.com`, then the target name should be, say, `example.com`.
	///
	/// Most recently defined by RFC 3598, Section 6.4 Flags.
	/// Previously defined in RFC 2915 and RFC 3404, Section 4.3 Flags.
	A,
	
	/// The next look up should be for `URI` records using a target domain name.
	/// Inferred by example data in RFC 7553, Section 5.2.
	///
	/// There should be a target domain name present.
	/// There should be no regular expression present.
	/// The target name should be one suitable for a `URI`, ie if the origin is `example.com`, then the target name should be, say, `_http._tcp.example.com`.
	D,
	
	/// The next look up should process a URI
	/// The presence of this flags defines a `U-NAPTR`, properly known as "Straightforward URI-Enabled NAPTR".
	///
	/// There should be no target domain name present.
	/// There should be a limited form of regular expression present, which is effectively just a URI.
	///
	/// Most recently defined by RFC 4848, Section 2.1 Permitted Flags: "… the presence of the "U" Flag in the NAPTR record indicates the REGEXP field must be populated (and, consequently, the REPLACEMENT field is empty).
	/// The regular expression in the REGEXP field must be of the limited form \[in Section 2.2 Permitted Regular Expressions\], and the result of the regular expression evaluation will be a URI that is the result of the DDDS resolution".
	/// Previously defined in RFC 2915 and RFC 3404, Section 4.3 Flags.
	///
	/// This can be thought of as obsoleted by RFC 7553, which provides the much more useful `URI` resource record and can work with the `D` flag.
	U,
	
	/// The remainder of the application side algorithm shall be carried out in a Protocol-specific fashion.
	///
	/// Defined by RFC 2915.
	///
	/// Redefined by RFC 3404, Section 4.3 Flag, paragraph 2: "The "P" flag says that the remainder of the DDDS Algorithm is ignored and that the rest of the process is application specific and outside the scope of this document.
	/// An application can use the Protocol part found in the Services field to identify which Application specific set of rules that should be followed next.
	/// The record that contains the 'P' flag is the last record that is interpreted by the rules in this document".
	///
	/// The new set of rules is identified by the Protocol specified in the Services field
	P,
}

impl NamingAuthorityMutuallyExclusiveFlag
{
	/// Is this a terminal flag that halts the looping rewrite algorithm?
	#[inline(always)]
	pub fn is_terminal(self) -> bool
	{
		use self::NamingAuthorityMutuallyExclusiveFlag::*;
		
		match self
		{
			S | A | D | U => true,
			
			P => false,
		}
	}
	#[inline(always)]
	pub(crate) fn parse_flags<'message>(raw_flags: ParsedCharacterString<'message>) -> Result<Either<Option<Self>, NamingAuthorityResourceRecordIgnoredReason>, NAPTRHandleRecordTypeError>
	{
		use self::NAPTRHandleRecordTypeError::*;
		use self::NamingAuthorityResourceRecordIgnoredReason::*;
		
		use self::NamingAuthorityMutuallyExclusiveFlag::*;
		
		// Whilst RFC 3404, Section 4.3 Flags, paragraph 3 (and its predecessor,  RFC 2915, Section 2 NAPTR RR Format, Flags, paragraph 2) permits multiple flags and non-mutually exclusive flags, no such flags have ever come into being in 20 years.
		// Hence we optimize for zero or one mutually exclusive flags.
		match raw_flags.len()
		{
			0 => Ok(Left(None)),
			
			1 =>
			{
				let flag_byte = unsafe { *raw_flags.deref().get_unchecked(0) };
				match flag_byte
				{
					b'S' | b's' => Ok(Left(Some(S))),
					
					b'A' | b'a' => Ok(Left(Some(A))),
					
					b'D' | b'd' => Ok(Left(Some(D))),
					
					b'U' | b'u' => Ok(Left(Some(U))),
					
					b'P' | b'p' => Ok(Left(Some(P))),
					
					b'0' ..= b'9' => Ok(Right(NumericFlagBytesAreForLocalExpermination(case_folded_flag_byte))),
					
					0x00 ..= 0x2F => Err(FlagByteNotAlphanumeric(case_folded_flag_byte)),
					
					0x3A ..= 0x40 => Err(FlagByteNotAlphanumeric(case_folded_flag_byte)),
					
					0x5B ..= 0xFF => Err(FlagByteNotAlphanumeric(case_folded_flag_byte)),
					
					_ => Ok(Right(UndefinedAlphaFlagByte(case_folded_flag_byte))),
				}
			}
			
			_ => Ok(Right(MultipleFlags)),
		}
	}
}
