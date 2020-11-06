// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Validated, case-folded string that:-
///
/// * Can not be empty;
/// * Has a first ASCII byte of `a` to `z` inclusive.
/// * If 2 or more bytes long, has second and subsequent ASCII bytes of `a` to `z` inclusive or `0` to `9` inclusive.
/// * Is a maximum of 32 bytes.
///
/// To see a (rare) example of a SIP NAPTR record, do `dig NAPTR columbia.edu`, which as of Nov 2020 was returning `columbia.edu.		3600	IN	NAPTR	1 0 "s" "SIP+D2U" "" _sip._udp.columbia.edu.`.
///
/// Legacy RFCs referred to:-
///
/// * `THTTP` (as defined by RFC 3404 Section 4.4.2 Protocols), a protocol using HTTP to resolve URNs, with one or more of:-
/// 	* `I2L`.
/// 	* `I2Ls`.
/// 	* `I2R`.
/// 	* `I2Rs`.
/// 	* `I2C`.
/// 	* `I2CS`.
/// 	* `I2N`.
///
/// Also seen, in the obsolete Diameter Protocol RFC 3588, Section 11.6 NAPTR Service Fields, but not registered with IANA:-
///
/// * `AAA` as:-
/// 	* `AAA+D2T` (TCP).
/// 	* `AAA+D2S` (SCTP).
/// 	* with generic form `AAA+D2n`, where `n` is a code letter for the transport protocol.
///
/// ***NOTE*** that Diameter now uses S-NAPTR records.
///
/// RFC 6116, Section 3.4.3 Service Parameters defines:-
///
/// * `E2U` with one or more 'enumservice'.
/// 	* This should not be present in the SERVICE field for non-terminal records.
/// 	* 'enumservice':-
/// 		* These can contain a hyphen `-`, but
/// 			* these are not registered with IANA.
/// 		* 'enumservice' is case-insensitive
///				*  can start `x-` for experimental and `p-` for private, but these are not registered with IANA.
/// 		* Sadly these have a subtype
/// 			* that is demarked by a leading colon `:`.
/// 			* the sub type can then use a hyphen `-`.
/// 		* whilst 'enumservice' can
///
/// RFC 3263 defines an IANA registry for SIP: <https://www.iana.org/assignments/sip-table/sip-table.xhtml#sip-table-1> with values:-
///
/// * `SIP` as:-
/// 	* `SIP+D2T` (TCP).
/// 	* `SIP+D2U` (UDP).
/// 	* `SIP+D2S` (SCTP).
/// 	* `SIP+D2W` (WS, web socket).
/// * `SIPS` (implies SIP over TLS / DTLS) as:-
/// 	* `SIPS+D2T` (TCP).
/// 	* `SIPS+D2S` (SCTP).
/// 	* `SIPS+D2W` (WS, web socket).
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CaseFoldedNamingAuthorityProtocol(Box<[u8]>);

impl Deref for CaseFoldedNamingAuthorityProtocol
{
	type Target = [u8];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl CaseFoldedNamingAuthorityProtocol
{
	// Implementation based on RFC 2915, Section 2 NAPTR RR Format, Service, paragraph 3 (page 6).
	// See also redefinitions by RFCs 3401 to 3404 inclusive.
	#[inline(always)]
	pub(crate) fn parse_services<'message>(service: ParsedCharacterString<'message>) -> Result<(Option<Self>, HashSet<CaseFoldedNamingAuthorityResolutionService>), ProtocolOrResolutionServiceParseError>
	{
		let service = service.deref();
		if service.is_empty()
		{
			return Ok((None, HashSet::default()))
		}
		
		const ResolutionServicePrefix: u8 = b'+';
		
		let (protocol, resolution_services_iterator) = if (unsafe { *service.get_unchecked(0) }) == ResolutionServicePrefix
		{
			(None, (&service[1 .. ]).split_bytes(ResolutionServicePrefix))
		}
		else
		{
			let mut iterator = resolution_service_bytes.split_bytes(b'+');
			let protocol = Self(Self::parse_protocol_or_resolution_service(iterator.next().unwrap())?);
			(Some(protocol), iterator)
		};
		
		let mut resolution_services = HashSet::new();
		for resolution_service_bytes in resolution_services_iterator
		{
			let new = resolution_services.insert(CaseFoldedNamingAuthorityResolutionService(Self::parse_protocol_or_resolution_service(resolution_service_bytes)?));
			let is_duplicate = !new;
			if unlikely!(is_duplicate)
			{
				return Err(ProtocolOrResolutionServiceParseError::DuplicateResolutionService)
			}
		}
		
		Ok((protocol, resolution_services))
	}
	
	
	#[inline(always)]
	fn parse_protocol_or_resolution_service(bytes: &[u8]) -> Result<Box<[u8]>, ProtocolOrResolutionServiceParseError>
	{
		use self::ProtocolOrResolutionServiceParseError::*;
		
		let length = bytes.len();
		if unlikely!(length == 0)
		{
			return Err(CanNotBeEmpty)
		}
		if unlikely!(length > 32)
		{
			return Err(CanNotExceed32Bytes(length))
		}
		
		let mut vec = Vec::with_capacity(length);
		unsafe { vec.set_len(length) };
		
		let case_folded_byte = match unsafe { * bytes.get_unchecked(0) }
		{
			byte @ b'A' ..= b'Z' => case_fold_upper_case_byte_to_lower_case_byte(byte),
			byte @ b'a' ..= b'z' => byte,
			byte @ b'0' ..= b'9' => Err(FirstByteCanNotBeNumeric(byte)),
			byte @ _ => return Err(FirstByteOutOfRange(byte))
		};
		unsafe { * vec.get_unchecked_mut(0) = case_folded_byte };
		
		for index in 1 .. length
		{
			let case_folded_byte = match unsafe { * bytes.get_unchecked(index) }
			{
				byte @ b'A' ..= b'Z' => case_fold_upper_case_byte_to_lower_case_byte(byte),
				byte @ b'a' ..= b'z' => byte,
				byte @ _ => return Err(SubsequentByteOutOfRange(subsequent_byte, unsafe { NonZeroU8::new_unchecked(index as u8) }))
			};
			unsafe { * vec.get_unchecked_mut(index) = case_folded_byte };
		}
		
		Ok(vec.into_boxed_slice())
	}
}
