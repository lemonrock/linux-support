// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 5424, Section 6.2.1, Table 2.
///
/// RFC 5424, Section 6.2.1: "Severity values MUST be in the range of 0 to 7 inclusive".
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum VerifierLogLevel
{
	/// Alert: action must be taken immediately.
	Alert = 1,
	
	/// Critical: critical conditions.
	Critical = 2,
	
	/// Error: error conditions.
	Error = 3,
	
	/// Warning: warning conditions.
	Warning = 4,
	
	/// Notice: normal but significant condition.
	Notice = 5,
	
	/// Informational: informational messages.
	Informational = 6,
	
	/// Debug: debug-level messages.
	Debug = 7,
}

impl Default for VerifierLogLevel
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::VerifierLogLevel::*;
		
		if cfg!(debug_assertions)
		{
			Debug
		}
		else
		{
			Warning
		}
	}
}
