// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn sub_types() -> HashMap<EnumServiceSubTypeType, IndexMap<NaptrSubType, EnumServiceSubTypeMember>>
{
	hashmap!
	{
		"EmailEnumServiceSubType" => indexmap!
		{
			"mailto" => "mailto"
		},
	
		"EmsEnumServiceSubType" => indexmap!
		{
			"mailto" => "mailto",
			"tel" => "tel",
		},
	
		"FaxEnumServiceSubType" => indexmap!
		{
			"tel" => "tel",
		},
	
		"FileServiceServiceSubType" => indexmap!
		{
			"ftp" => "ftp",
		},
	
		"WebEnumServiceSubType" => indexmap!
		{
			"http" => "http",
			"https" => "https",
		},
	
		"PstnEnumServiceSubType" => indexmap!
		{
			"sip" => "sip",
			"tel" => "tel",
		},
	
		"UnifiedMessagingEnumServiceSubType" => indexmap!
		{
			"http" => "http",
			"https" => "https",
			"sip" => "sip",
			"sips" => "sips",
		},
	
		"VoiceEnumServiceSubType" => indexmap!
		{
			"tel" => "tel",
		},
	
		"VoiceMessageEnumServiceSubType" => indexmap!
		{
			"http" => "http",
			"https" => "https",
			"sip" => "sip",
			"sips" => "sips",
			"tel" => "tel",
		},
	
		"VpimEnumServiceSubType" => indexmap!
		{
			"ldap" => "ldap",
			"mailto" => "mailto",
		},
	}
}
