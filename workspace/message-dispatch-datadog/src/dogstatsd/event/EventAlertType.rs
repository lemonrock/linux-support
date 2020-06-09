// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Defaults to `info` if not specified.
///
/// Sent as a byte string.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventAlertType
{
	/// `error`.
	Error,
	
	/// `warning`.
	Warning,
	
	/// `success`.
	Success,
	
	/// `info`.
	///
	/// This is the default.
	Informational,
}

impl Default for EventAlertType
{
	#[inline(always)]
	fn default() -> Self
	{
		EventAlertType::Informational
	}
}

impl EventAlertType
{
	#[inline(always)]
	fn to_bytes(self) -> &'static [u8]
	{
		use self::EventAlertType::*;
		
		match self
		{
			Error => b"error",
			
			Warning => b"warning",
			
			Success => b"success",
			
			Informational => b"info",
		}
	}
}
