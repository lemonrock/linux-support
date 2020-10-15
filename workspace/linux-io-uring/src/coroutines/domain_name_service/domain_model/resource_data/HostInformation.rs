// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Host information.
///
/// Brought back from obscurity by RFC 8482.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HostInformation<'a>
{
	/// `CPU` field.
	///
	/// In RFC 8482, this will be `RFC8482`.
	pub cpu: &'a [u8],

	/// `OS` field.
	///
	/// In RFC 8482, this will be ``.
	pub os: &'a [u8],
}

impl<'a> HostInformation<'a>
{
	/// Is this a RFC 8482 answer to the `ANY` / `*` `QTYPE` question?
	#[inline(always)]
	pub fn is_rfc_8482_answer_to_any_question(&self) -> bool
	{
		self.cpu == b"RFC8482" && self.os.is_empty()
	}

	/// Is this a CloudFlare answer to the `ANY` / `*` `QTYPE` question?
	#[inline(always)]
	pub fn is_cloudflare_answer_to_any_question(&self) -> bool
	{
		self.cpu == b"ANY obsoleted" && self.os == b"See draft-ietf-dnsop-refuse-any"
	}
}
