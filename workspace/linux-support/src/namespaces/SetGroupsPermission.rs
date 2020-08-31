// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `setgroups` permission.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SetGroupsPermission
{
	/// Allow.
	Allow,

	/// Deny.
	Deny,
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for SetGroupsPermission
{
	/// Converts data to a byte string terminated with a new line (`\n`).
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		use self::SetGroupsPermission::*;

		match self
		{
			Allow => Cow::from(b"allow\n" as &[u8]),
			Deny => Cow::from(b"deny\n" as &[u8]),
		}
	}
}
