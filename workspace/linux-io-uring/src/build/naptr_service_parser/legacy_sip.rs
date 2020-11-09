// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// See <https://www.iana.org/assignments/sip-table/sip-table.xhtml>.
fn legacy_sip() -> HashMap<&'static str, &'static str>
{
	hashmap!
	{
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIP+D2T" => "LegacySip(SipLegacyResolutionService::D2T)",
		
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIPS+D2T" => "LegacySipSecure(SipSecureLegacyResolutionService::D2T)",
		
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIP+D2U" => "LegacySip(SipLegacyResolutionService::D2U)",
		
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIP+D2S" => "LegacySip(SipLegacyResolutionService::D2S)",
		
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIPS+D2S" => "LegacySipSecure(SipSecureLegacyResolutionService::D2S)",
		
		// RFC 7118.
		"SIP+D2W" => "LegacySip(SipLegacyResolutionService::D2W)",
		
		// RFC 7118.
		"SIPS+D2W" => "LegacySipSecure(SipSecureLegacyResolutionService::D2W)",
	}
}
