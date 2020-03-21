// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[allow(missing_docs)]
#[repr(i32)]
pub enum SyslogFacility
{
	auth = LOG_AUTH,
	cron = LOG_CRON,
	daemon = LOG_DAEMON,
	ftp = LOG_FTP,
	kern = LOG_KERN,
	lpr = LOG_LPR,
	mail = LOG_MAIL,
	news = LOG_NEWS,
	syslog = LOG_SYSLOG,
	user = LOG_USER,
	uucp = LOG_UUCP,
	local0 = LOG_LOCAL0,
	local1 = LOG_LOCAL1,
	local2 = LOG_LOCAL2,
	local3 = LOG_LOCAL3,
	local4 = LOG_LOCAL4,
	local5 = LOG_LOCAL5,
	local6 = LOG_LOCAL6,
	local7 = LOG_LOCAL7,
}

impl Default for SyslogFacility
{
	#[inline(always)]
	fn default() -> Self
	{
		SyslogFacility::auth
	}
}
