// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The value encoded in this opcode is NOT the same as that defined by IANA; instead the encoded values are the IANA values left-shifted by 3.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct MessageOpcode;

impl MessageOpcode
{
	/// Query.
	///
	/// Defined in RFC 1035.
	pub(crate) const Query: u8 = 0;

	/// Inverse Query ('IQuery').
	///
	/// Defined in RFC 1035; made obsolete by RFC 3425.
	pub(crate) const InverseQuery: u8 = 1;

	/// Status.
	///
	/// Defined in RFC 1035.
	pub(crate) const Status: u8 = 2;

	/// Notify.
	///
	/// Defined in RFC 1996.
	pub(crate) const Notify: u8 = 4;

	/// Update.
	///
	/// Defined in RFC 2136.
	pub(crate) const Update: u8 = 5;

	/// DNS Stateful Operations, DSO.
	///
	/// Defined in [RFC-ietf-dnsop-session-signal-20](http://www.iana.org/go/draft-ietf-dnsop-session-signal-20).
	pub(crate) const DnsStatefulOperations: u8 = 6;
}
