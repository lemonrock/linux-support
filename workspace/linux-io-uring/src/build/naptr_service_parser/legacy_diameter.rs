// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// RFC 3588, Section 11.6 NAPTR Service Fields.
fn legacy_diameter() -> HashMap<&'static str, &'static str>
{
	hashmap!
	{
		"AAA+D2T" => "LegacyDiameter { resolution_service: DiameterResolutionService::D2T }",
		"AAA+D2S" => "LegacyDiameter { resolution_service: DiameterResolutionService::D2S }",
	}
}
