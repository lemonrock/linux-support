// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Validated, case-folded string that:-
///
/// * Can not be empty;
/// * Has a first ASCII byte of `a` to `z` inclusive.
/// * If 2 or more bytes long, has second and subsequent ASCII bytes of `a` to `z` inclusive or `0` to `9` inclusive.
/// * Is a maximum of 32 bytes.
///
/// A list of valid services is defined by RFC 2483, but not registered with IANA; these seem to only be applicable when using NAPTR records for URNs and URIs as per RFC 3404:-
///
/// * `I2L`.
/// * `I2Ls`.
/// * `I2R`.
/// * `I2Rs`.
/// * `I2C`.
/// * `I2CS`.
/// * `I2N`.
///
/// Also seen, in the obsolete Diameter Protocol RFC 3588, Section 11.6 NAPTR Service Fields, but not registered with IANA:-
///
/// * `D2T`.
/// * `D2S`.
///
/// * RFC 6116 The E.164 to Uniform Resource Identifiers (URI) Dynamic Delegation Discovery System (DDDS) Application (ENUM), Section 3.4.3 names "resolution service" as "enumservice", and permits a sub-type to be in it which is `:` delimited.
///
/// Lastly RFC 3263 defines an IANA registry for SIP: <https://www.iana.org/assignments/sip-table/sip-table.xhtml#sip-table-1> with values:-
///
/// * `D2T` (TCP).
/// * `D2U` (UDP).
/// * `D2S` (SCTP).
/// * `D2W` (WS, web socket).
///
/// Not all combinations are valid.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CaseFoldedNamingAuthorityResolutionService(Box<[u8]>);

impl Deref for CaseFoldedNamingAuthorityResolutionService
{
	type Target = [u8];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl CaseFoldedNamingAuthorityResolutionService
{
	#[inline(always)]
	fn unchecked_from(value: &[u8]) -> Self
	{
		Self(value.to_vec().into_boxed_slice())
	}
}
