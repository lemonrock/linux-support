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
    ($self: ident $($upper: ident $(| $lower: ident)* => $block: expr)*) =>
    {
        match
        {
    	    let current_index = $self.current_index();
    	    
    		if current_index >= $self.length()
    		{
    			return $self.parse_s_naptr()
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
            
			_ => $self.parse_s_naptr(),
        }
    }
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
	/// experimental-service      = "x-" 1*30ALPHANUMSYM
	/// experimental-protocol     = "x-" 1*30ALPHANUMSYM
	/// iana-registered-service   = ALPHA *31ALPHANUMSYM
	/// iana-registered-protocol  = ALPHA *31ALPHANUMSYM
	/// ALPHA         =  %x41-5A / %x61-7A   ; A-Z / a-z
	/// DIGIT         =  %x30-39 ; 0-9
	/// SYM           =  %x2B / %x2D / %x2E  ; "+" / "-" / "."
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
	/// protocol      = ALPHA *31ALPHANUM
	/// rs            = ALPHA *31ALPHANUM
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
	///  type          = 1*32(ALPHA / DIGIT / "-")
	///  subtype       = 1*32(ALPHA / DIGIT / "-")
	///
	/// In other words, a non-optional "E2U" (used to denote ENUM only
	/// Rewrite Rules in order to mitigate record collisions) is followed by
	/// one or more Enumservices that indicate the class of functionality a
	/// given end point offers.  Each Enumservice is indicated by an initial
	/// '+' character.
    pub(super) fn parse(&self) -> Result<Option<()>, ServiceFieldParseError>
    {
        byte_match!
        (
            self
            
            A | a => byte_match!
            (
                self
                
                A | a => byte_match!
                (
                	self
                	
                	A | a => byte_match!
                	(
                		self
                		
                		PLUS => self.parse_legacy_diameter()
                	)
                )
            )
            
            E | e => byte_match!
            (
                self
            )
        )
    }

	#[inline(always)]
	fn parse_legacy_diameter(&self) -> Result<Option<()>, ServiceFieldParseError>
	{
		const DiameterTemplate: &'static [u8] = b"AAA+nnn";
		const DiameterTemplateLength: usize = DiameterTemplate.len();
		
		if unlikely!(self.length() ==
		
		unimplemented!()
	}
	
	#[inline(always)]
	fn parse_legacy_enumservice(&self) -> Result<Option<()>, ServiceFieldParseError>
	{
		const ShortestE2UTemplate: &'static [u8] = b"E2U+im";
		const ShortestE2ULength: usize = ShortestE2UTemplate.len();
		
		unimplemented!()
	}
	
	#[inline(always)]
	fn parse_legacy_sip(&self) -> Result<Option<()>, ServiceFieldParseError>
	{
		const SipTemplate: &'static [u8] = b"SIP+nnn";
		const SipTemplateLength: usize = SipTemplate.len();
		
	    unimplemented!()
	}
	
	#[inline(always)]
	fn parse_legacy_sip_secure(&self) -> Result<Option<()>, ServiceFieldParseError>
	{
		const SipsTemplate: &'static [u8] = b"SIPS+nnn";
		const SipsTemplateLength: usize = SipsTemplate.len();
		
		unimplemented!()
	}
	
	#[inline(always)]
	fn parse_s_naptr(&self) -> Result<Option<()>, ServiceFieldParseError>
	{
	    unimplemented!()
	}
	
	#[inline(always)]
	fn byte_ref_unchecked(&self, index: usize) -> &u8
	{
		unsafe { self.service_field_bytes.get_unchecked(index) }
	}
	
	#[inline(always)]
	fn cast<T>(&self) -> &T
	{
		unsafe { & * (self.service_field_bytes.as_ptr() as *const T) }
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

impl<'message> ServiceFieldParser<'message>
{
	pub(super) fn parse(&self) -> Result<Option<()>, ServiceFieldParseError>
	{
		match byte!(self, 0)
		{
			// `A`.
			A | a => match byte!(self, 1)
			{
				// `AA`.
				A | a => match byte!(self, 2)
				{
					// `AAA`.
					A | a => match byte!(3)
					{
						// `AAA+`: Probably legacy diameter.
						PLUS => match byte!(4)
						{
							// `AAA+D`.
							D | d => match byte!(5)
							{
								// `AAA+D2`.
								_2 => match byte!(6)
								{
									// `AAA+D2T`: Probably legacy diameter over TCP.
									T | t => if likely!(self.length() == DiameterTemplateLength)
									{
										Ok(Some(DiameterLegacyResolutionService::D2T))
									} else {
										// Draft RFC <https://tools.ietf.org/html/draft-jones-dime-extended-naptr-01> made use of an extension with `+AP`.
										// However, this draft, which eventually became RFC 6408, dropped this idea and moved to `S-NAPTR` instead.
										self.parse_s_naptr()
									},
									
									// `AAA+D2S`: Probably legacy diameter over SCTP.
									S | s => if likely!(self.length() == DiameterTemplateLength)
									{
										Ok(Some(DiameterLegacyResolutionService::D2S))
									} else {
										// Draft RFC <https://tools.ietf.org/html/draft-jones-dime-extended-naptr-01> made use of an extension with `+AP`.
										// However, this draft, which eventually became RFC 6408, dropped this idea and moved to `S-NAPTR` instead.
										self.parse_s_naptr()
									},
								}
							}
						}
					}
				}
			}
			
			// `E`.
			E | e => match byte!(self, 1)
			{
				// `E2`.
				_2 => match byte!(2)
				{
					// `E2U`.
					U | u => match byte!(self, 3)
					{
						// `E2U+`: Probably legacy enumservice.
						PLUS => self.parse_legacy_enumservice(),
					}
				}
			}
			
			// `S`.
			S | s => match byte!(self, 1)
			{
				// `SI`.
				I | i => match byte!(self, 2)
				{
					// `SIP`.
					P | p => match byte!(self, 3)
					{
						// `SIP+`: Probably 'legacy' SIP.
						PLUS => self.parse_legacy_sip(),
						
						// `SIPS`.
						S | s => match byte!(self, 4)
						{
							// `SIPS+`: Probably 'legacy' SIP secure.
							PLUS => self.parse_legacy_sip_secure()
						}
					}
				}
			}
		}
	}
}
