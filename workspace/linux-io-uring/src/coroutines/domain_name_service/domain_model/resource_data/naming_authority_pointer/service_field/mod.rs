// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;


/// Legacy resolution services that fit the ABNF for the `type` production part of the `enumservice` production in RFC 6116, Section 3.4.3 Service Parameters.
pub mod enum_services;


/// Legacy resolution services that fit the ABNF for the `rs` production in RFC 3404, Section 4.4 Services Parameters.
pub mod legacy_resolution_services;


/// Application services and protocols as first defined by RFC 3958 (`S-NAPTR`), Section 6.5 Service Parameters and registered with IANA at <https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml>.
///
/// Includes definitions for both `S-NAPTR` and `U-NAPTR`.
pub mod s_naptr;


include!("CaseFoldedNamingAuthorityProtocol.rs");
include!("CaseFoldedNamingAuthorityResolutionService.rs");
include!("ProtocolOrResolutionServiceParseError.rs");
include!("ServiceFieldParser.rs");
include!("ServiceFieldParseError.rs");
