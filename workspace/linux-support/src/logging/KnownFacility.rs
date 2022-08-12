// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 5424, Section 6.2.1, Table 1.
///
/// RFC 5424, Section 6.2.1: "Facility values MUST be in the range of 0 to 23 inclusive".
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum KnownFacility
{
	/// kernel messages.
	kernel_messages = 0,
	
	/// user-level messages.
	user_level_messages = 1,
	
	/// mail system.
	mail_system = 2,
	
	/// system daemons.
	system_daemons = 3,
	
	/// security/authorization messages (0).
	security_or_authorization_messages_0 = 4,
	
	/// messages generated internally by syslogd.
	messages_generated_internally_by_syslogd = 5,
	
	/// line printer subsystem.
	line_printer_subsystem = 6,
	
	/// network news subsystem.
	network_news_subsystem = 7,
	
	/// UUCP subsystem.
	UUCP_subsystem = 8,
	
	/// clock daemon (0).
	clock_daemon_0 = 9,
	
	/// security/authorization messages (1).
	security_or_authorization_messages_1 = 10,
	
	/// FTP daemon.
	ftp_daemon = 11,
	
	/// NTP subsystem.
	ntp_subsystem = 12,
	
	/// log audit.
	log_audit = 13,
	
	/// log alert.
	log_alert = 14,
	
	/// clock daemon (2).
	clock_daemon_2 = 15,
	
	/// local use 0 (local0).
	local_use_0 = 16,
	
	/// local use 1 (local1).
	local_use_1 = 17,
	
	/// local use 2 (local2).
	local_use_2 = 18,
	
	/// local use 3 (local3).
	local_use_3 = 19,
	
	/// local use 4 (local4).
	local_use_4 = 20,
	
	/// local use 5 (local5).
	local_use_5 = 21,
	
	/// local use 6 (local6).
	local_use_6 = 22,
	
	/// local use 7 (local7).
	local_use_7 = 23,
}

impl Default for KnownFacility
{
	#[inline(always)]
	fn default() -> Self
	{
		KnownFacility::security_or_authorization_messages_0
	}
}
