// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// What to do on a Kernel panic.
///
/// Defaults to 120 seconds.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum KernelPanicRebootAction
{
	/// Reboot immediately
	Immediate,

	/// Never reboot.
	Never,

	/// After some seconds.
	AfterSomeSeconds
	{
		/// Number of seconds to wait.
		///
		/// Default is 120.
		seconds: NonZeroU32,
	}
}

impl Default for KernelPanicRebootAction
{
	#[inline(always)]
	fn default() -> Self
	{
		KernelPanicRebootAction::AfterSomeSeconds { seconds: unsafe { NonZeroU32::new_unchecked(120) } }
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for KernelPanicRebootAction
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		use self::KernelPanicRebootAction::*;

		match self
		{
			Immediate => Cow::from(b"-1\n" as &[u8]),

			Never => Cow::from(b"0\n" as &[u8]),

			AfterSomeSeconds { seconds } => seconds.into_line_feed_terminated_byte_string()
		}
	}
}
