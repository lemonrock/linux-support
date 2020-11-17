// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn enumservices() -> HashMap<&'static str, (&'static str, Option<EnumServiceSubTypeType>)>
{
	hashmap!
	{
		"acct" =>
		(
			"acct",
			None,
		),
		
		"email" =>
		(
			"email",
			Some("EmailEnumServiceSubType"),
		),
		
		"ems" =>
		(
			"ems",
			Some("EmsEnumServiceSubType"),
		),
		
		"fax" =>
		(
			"fax",
			Some("FaxEnumServiceSubType"),
		),
		
		"ft" =>
		(
			"ft",
			Some("FileServiceServiceSubType"),
		),
		
		"h323" =>
		(
			"h323",
			None,
		),
		
		"iax" =>
		(
			"iax",
			None,
		),
		
		"ical-access" =>
		(
			"ical_access",
			Some("WebEnumServiceSubType"),
		),
		
		"ical-sched" =>
		(
			"ical_sched",
			Some("EmailEnumServiceSubType"),
		),
		
		"ifax" =>
		(
			"ifax",
			Some("EmailEnumServiceSubType"),
		),
		
		"im" =>
		(
			"im",
			None,
		),
		
		"mms" =>
		(
			"mms",
			Some("EmsEnumServiceSubType"),
		),
		
		"pres" =>
		(
			"pres",
			None,
		),
		
		"pstn" =>
		(
			"pstn",
			Some("PstnEnumServiceSubType"),
		),
		
		"sip" =>
		(
			"sip",
			None,
		),
		
		"sms" =>
		(
			"sms",
			Some("EmsEnumServiceSubType"),
		),
		
		"unifmsg" =>
		(
			"unifmsg",
			Some("UnifiedMessagingEnumServiceSubType"),
		),
		
		"vcard" =>
		(
			"vcard",
			Some("WebEnumServiceSubType"),
		),
		
		"videomsg" =>
		(
			"videomsg",
			Some("UnifiedMessagingEnumServiceSubType"),
		),
		
		"voice" =>
		(
			"voice",
			Some("VoiceEnumServiceSubType"),
		),
		
		"voicemsg" =>
		(
			"voicemsg",
			Some("VoiceMessageEnumServiceSubType"),
		),
		
		"vpim" =>
		(
			"vpim",
			Some("VpimEnumServiceSubType"),
		),
		
		"web" =>
		(
			"web",
			Some("WebEnumServiceSubType"),
		),
		
		"xmpp" =>
		(
			"xmpp",
			None,
		),
	}
}
