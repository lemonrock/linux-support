// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


const NUL: u8 = 0x00;
const ASTERISK: u8 = b'*';
const COMMA: u8 = b',';
const SLASH: u8 = b'/';
const SEMICOLON: u8 = b';';
const LESSTHAN: u8 = b'<';
const GREATERTHAN: u8 = b'>';
const AT: u8 = b'@';
const OPENSQUAREBRACKET: u8 = b'[';
const BACKTICK: u8 = b'`';
const OPENBRACE: u8 = b'{';

macro_rules! byte_match
{
	($self: ident) =>
	{
		byte_match!($self @)
	};
	
	($self: ident @ $($upper: ident $(| $lower: ident)* => $block: expr)*) =>
	{
		byte_match!($self parse_s_naptr @ $($upper $(| $lower)* => $block)*)
	};
	
	($self: ident $empty: ident @ $($upper: ident $(| $lower: ident)* => $block: expr)*) =>
	{
		byte_match!($self $empty parse_s_naptr @ $($upper $(| $lower)* => $block)*)
	};
	
	($self: ident $empty: ident $default: ident @ $($upper: ident $(| $lower: ident)* => $block: expr)*) =>
	{
		match
		{
			let current_index = $self.current_index();
			
			if unlikely!(current_index >= $self.length())
			{
				return $self.$empty()
			}
			else
			{
				let byte = * $self.byte_ref_unchecked(current_index);
				$self.current_index.set(current_index + 1);
				byte
			}
		}
		{
			byte @ NUL ..= ASTERISK | byte @ COMMA | byte @ SLASH | byte @ SEMICOLON ..= LESSTHAN | byte @ GREATERTHAN ..= AT | byte @ OPENSQUAREBRACKET ..= BACKTICK | byte @ OPENBRACE ..= 0xFF => return Err(ServiceFieldParseError::OutOfRange(byte, $self.current_index() - 1)),
		
			$($upper $(| $lower)* => $block,)*
			
			_ => $self.$default(),
		}
	};
}

const HYPHEN: u8 = b'-';
const PERIOD: u8 = b'.';
const PLUS: u8 = b'+';
const COLON: u8 = b':';
const EQUALS: u8 = b'=';
const _2: u8 = b'2';
const A: u8 = b'A';
const a: u8 = b'a';
const D: u8 = b'D';
const d: u8 = b'd';
const E: u8 = b'E';
const e: u8 = b'e';
const I: u8 = b'I';
const i: u8 = b'i';
const P: u8 = b'P';
const p: u8 = b'p';
const S: u8 = b'S';
const s: u8 = b's';
const T: u8 = b'T';
const t: u8 = b't';
const U: u8 = b'U';
const u: u8 = b'u';
const W: u8 = b'W';
const w: u8 = b'w';

pub(super) struct ServiceFieldParser<'message>
{
	service_field_bytes: &'message [u8],
	current_index: Cell<usize>,
}

impl<'message> ServiceFieldParser<'message>
{
	#[inline(always)]
	pub(super) const fn new(service_field_bytes: &'message [u8]) -> Self
	{
		Self
		{
			service_field_bytes,
			current_index: Cell::new(0),
		}
	}
	
	/// RFC 3958
	///
	/// service-parms = [ [app-service] *(":" app-protocol)]
	/// app-service   = experimental-service  / iana-registered-service
	/// app-protocol  = experimental-protocol / iana-registered-protocol
	/// experimental-service	  = "x-" 1*30ALPHANUMSYM
	/// experimental-protocol	 = "x-" 1*30ALPHANUMSYM
	/// iana-registered-service   = ALPHA *31ALPHANUMSYM
	/// iana-registered-protocol  = ALPHA *31ALPHANUMSYM
	/// ALPHA		 =  %x41-5A / %x61-7A   ; A-Z / a-z
	/// DIGIT		 =  %x30-39 ; 0-9
	/// SYM		   =  %x2B / %x2D / %x2E  ; "+" / "-" / "."
	/// ALPHANUMSYM   =  ALPHA / DIGIT / SYM
	/// ; The app-service and app-protocol tags are limited to 32
	/// ; characters and must start with an alphabetic character.
	/// ; The service-parms are considered case-insensitive.
	///
	/// Thus, the Service Parameters may consist of an empty string, an app-
	/// service, or an app-service with one or more app-protocol
	/// specifications separated by the ":" symbol.
	///
	/// Note that this is similar to, but not the same as the syntax used in
	/// the URI DDDS application ([6]).  The DDDS DNS database requires each
	/// DDDS application to define the syntax of allowable service strings.
	/// The syntax here is expanded to allow the characters that are valid in
	/// any URI scheme name (see [8]).  As "+" (the separator used in the
	/// RFC3404 service parameter string) is an allowed character for URI
	/// scheme names, ":" is chosen as the separator here.
	///
	///
	/// app-service seems to be optional but the discussing text implies it can be absent and app-protocol (s) still be present.
	///
	///
	/// RFC 3404, Section 4.4 Services Parameters
	///
	/// service_field = [ [protocol] *("+" rs)]
	/// protocol	  = ALPHA *31ALPHANUM
	/// rs			= ALPHA *31ALPHANUM
	/// ; The protocol and rs fields are limited to 32
	/// ; characters and must start with an alphabetic.
	///
	///
	/// RFC 6116 (enum service)
	///
	/// Service Parameters for this Application take the following Augmented
	/// Backus-Naur Form (ABNF, specified in [RFC5234]) and are found in the
	/// Services field of the NAPTR record that holds a terminal Rule.  Where
	/// the NAPTR holds a non-terminal Rule, the Services field SHOULD be
	/// empty, and clients SHOULD ignore its content.
	///
	///  service-field = "E2U" 1*(servicespec)
	///  servicespec   = "+" enumservice
	///  enumservice   = type 0*(subtypespec)
	///  subtypespec   = ":" subtype
	///  type		  = 1*32(ALPHA / DIGIT / "-")
	///  subtype	   = 1*32(ALPHA / DIGIT / "-")
	///
	/// In other words, a non-optional "E2U" (used to denote ENUM only
	/// Rewrite Rules in order to mitigate record collisions) is followed by
	/// one or more Enumservices that indicate the class of functionality a
	/// given end point offers.  Each Enumservice is indicated by an initial
	/// '+' character.
	pub(super) fn parse(&self) -> Result<Option<ServiceField>, ServiceFieldParseError>
	{
		byte_match!
		(
			self @
			
			// `A`.
			A | a => byte_match!
			(
				self @
				
				// `AA`.
				A | a => byte_match!
				(
					self @
					
					// `AAA`.
					A | a => byte_match!
					(
						self
						
						parse_s_naptr_modern_diamter_aaa @
						
						// `AAA+`
						PLUS => self.parse_legacy_diameter_or_s_naptr_modern_diameter_or_radius()
						
					)
				)
			)
			
			// `E`.
			E | e => byte_match!
			(
				self @
				
				// `E2`.
				_2 => byte_match!
				(
					self @
					
					// `E2U`.
					U | u => byte_match!
					(
						self @
						
						// `E2U+`
						PLUS => self.parse_legacy_enum_service()
						
					)
				)
			)
			
			// `S`.
			S | s => byte_match!
			(
				self @
				
				// `SI`.
				I | i => byte_match!
				(
					self @
					
					// `SIP`.
					P | p => byte_match!
					(
						self @
						
						// `SIP+`
						PLUS => self.parse_legacy_sip()
						
					)
					
					// `SIPS`.
					S | s => byte_match!
					(
						self @
						
						// `SIPS+`
						PLUS => self.parse_legacy_sip_secure()
						
					)
				)
			)
		)
	}
	
	#[inline(always)]
	fn parse_legacy_diameter_or_s_naptr_modern_diameter_or_radius(&self) -> Result<Option<ServiceField>, ServiceFieldParseError>
	{
		use self::ServiceField::LegacyDiameter;
		use self::DiameterLegacyResolutionService::*;
		
		const LegacyDiameterTemplate: &'static [u8] = b"AAA+D2n";
		const LegacyDiameterTemplateLength: usize = LegacyDiameterTemplate.len();
		
		if likely!(self.length() == LegacyDiameterTemplateLength)
		{
			byte_match!
			(
				self unreachable parse_s_naptr_modern_diameter_or_radius_with_plus_sign @
				
				// `AAA+D`.
				D | d => byte_match!
				(
					self @
					
					// `AAA+D2`.
					_2 => byte_match!
					(
						self @
						
						// `AAA+D2T`: Legacy diameter over TCP.
						T | t => Ok(Some(LegacyDiameter(D2T)))
						
						// `AAA+D2S`: Legacy diameter over SCTP.
						S | s => Ok(Some(LegacyDiameter(D2S)))
					)
				)
			)
		}
		// Draft RFC <https://tools.ietf.org/html/draft-jones-dime-extended-naptr-01> made use of an extension with `+AP`.
		// However, this draft, which eventually became RFC 6408, dropped this idea and moved to `S-NAPTR` instead.
		else
		{
			self.parse_s_naptr_modern_diameter_or_radius_with_plus_sign()
		}
	}
	
	/// Is just `aaa`.
	#[inline(always)]
	fn parse_s_naptr_modern_diamter_aaa(&self) -> Result<Option<ServiceField>, ServiceFieldParseError>
	{
		Ok(Some(ServiceField::ModernDiameter { application_identifier: None, transport_protocols: HashSet::default() }))
	}
	
	#[inline(always)]
	fn unreachable(&self) -> Result<Option<ServiceField>, ServiceFieldParseError>
	{
		unreachable_code_const("Validated length greater than next byte")
	}
	
	#[inline(always)]
	fn parse_legacy_sip(&self) -> Result<Option<ServiceField>, ServiceFieldParseError>
	{
		use self::ServiceField::LegacySip;
		use self::SipLegacyResolutionService::*;
		
		const LegacySipTemplate: &'static [u8] = b"SIP+nnn";
		const LegacySipTemplateLength: usize = LegacySipTemplate.len();
		
		if likely!(self.length() == LegacySipTemplateLength)
		{
			byte_match!
			(
				self unreachable @
				
				// `SIP+D`.
				D | d => byte_match!
				(
					self @
					
					// `SIP+D2`.
					_2 => byte_match!
					(
						self @
						
						// `SIP+D2T`: SIP over TCP.
						T | t => Ok(Some(LegacySip(D2T)))
						
						// `SIP+D2U`: SIP over UDP.
						U | u => Ok(Some(LegacySip(D2U)))
						
						// `SIP+D2S`: SIP over SCTP.
						S | s => Ok(Some(LegacySip(D2S)))
						
						// `SIP+D2W`: SIP over Web Socket (WS).
						W | w => Ok(Some(LegacySip(D2W)))
					)
				)
			)
		}
		else
		{
			self.parse_s_naptr()
		}
	}
	
	#[inline(always)]
	fn parse_legacy_sip_secure(&self) -> Result<Option<ServiceField>, ServiceFieldParseError>
	{
		use self::ServiceField::LegacySipSecure;
		use self::SipSecureLegacyResolutionService::*;
		
		const LegacySipsTemplate: &'static [u8] = b"SIPS+nnn";
		const LegacySipsTemplateLength: usize = LegacySipsTemplate.len();
		
		if likely!(self.length() == LegacySipsTemplateLength)
		{
			byte_match!
			(
				self unreachable @
				
				// `SIPS+D`.
				D | d => byte_match!
				(
					self @
					
					// `SIPS+D2`.
					_2 => byte_match!
					(
						self @
						
						// `SIPS+D2T`: SIPS over TCP.
						T | t => Ok(Some(LegacySipSecure(D2T)))
						
						// `SIPS+D2S`: SIPS over SCTP.
						S | s => Ok(Some(LegacySipSecure(D2S)))
						
						// `SIPS+D2W`: SIPS over Web Socket (WS).
						W | w => Ok(Some(LegacySipSecure(D2W)))
					)
				)
			)
		}
		else
		{
			self.parse_s_naptr()
		}
	}
	
	#[inline(always)]
	fn parse_legacy_enum_service(&self) -> Result<Option<ServiceField>, ServiceFieldParseError>
	{
		const ShortestE2UTemplate: &'static [u8] = b"E2U+im";
		const ShortestE2ULength: usize = ShortestE2UTemplate.len();
		
		if self.length() < ShortestE2ULength
		{
			self.parse_s_naptr()
		}
		else
		{
			unimplemented!()
		}
	}
	
	/// Could be `aaa+apX` (diameter) or one of the radius protocols `aaa+acct`, `aaa+auth` or `aaa+dynauth` or something new.
	#[inline(always)]
	fn parse_s_naptr_modern_diameter_or_radius_with_plus_sign(&self) -> Result<Option<ServiceField>, ServiceFieldParseError>
	{
		let map: HashMap<X, X>;
		
		let mut hasher = map.hasher().build_hasher();
		
		for byte in [b'a', b'a', b'a', b'+'].iter()
		{
			byte.hash(&mut hasher)
		}
		
		// TODO: Consider using nom, eg with the alt!: https://docs.rs/nom/6.0.0/nom/macro.alt.html
		// https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md
		
		// TODO: codegen
		/*
			For each possible string combination, generation all the permutations of upper and lower case
			Then sort them
			Then match byte-by-byte, with lots of duplicate code paths.
			
			eg "SIP+D2W" and "AAA+D2W" and "aaa+ap1:diameter_dtls_sctp" and "aaa+ap1:diameter_tls_tcp"
			- generate all permutations of string; put into a BTreeSet
			
			- also the "x-"
			
			- also all the enumservice palavers.
			
			
			Then do match parsing, but, instead of going byte-by-byte, consider going u64, u32, u16, u8 blocks at a time?
		
		
		 */
	}
	
	#[inline(always)]
	fn parse_s_naptr(&self) -> Result<Option<ServiceField>, ServiceFieldParseError>
	{
		unimplemented!()
	}
	
	#[inline(always)]
	fn byte_ref_unchecked(&self, index: usize) -> &u8
	{
		unsafe { self.service_field_bytes.get_unchecked(index) }
	}
	
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.service_field_bytes.len()
	}
	
	#[inline(always)]
	fn current_index(&self) -> usize
	{
		self.current_index.get()
	}
}
