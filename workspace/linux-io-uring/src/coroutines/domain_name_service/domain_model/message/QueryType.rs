// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See RFC 6895, Section 3.1, Paragraph 3.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
pub(crate) struct QueryType(pub(crate) BigEndianU16);

impl QueryType
{
	/// `IXFR`.
	///
	/// This is a `QTYPE` and is only valid in a query section.
	///
	/// Defined in RFC 1035, clarified in RFC 6895, Section 3.1, Paragraph 3.
	pub(crate) const IXFR_higher: u8 = 0x00;
	pub(crate) const IXFR_lower: u8 = 251;
	pub(crate) const IXFR: Self = Self([Self::IXFR_higher, Self::IXFR_lower]);

	/// `AXFR`.
	///
	/// This is a `QTYPE` and is only valid in a query section.
	///
	/// Defined in RFC 1035, clarified in RFC 6895, Section 3.1, Paragraph 3.
	pub(crate) const AXFR_higher: u8 = 0x00;
	pub(crate) const AXFR_lower: u8 = 252;
	pub(crate) const AXFR: Self = Self([Self::AXFR_higher, Self::AXFR_lower]);

	/// `MAILB`.
	///
	/// This is a `QTYPE` and is only valid in a query section.
	///
	/// Clarified in RFC 6895, Section 3.1, Paragraph 3.
	///
	/// Defined in RFC 883 and made effectively obsolete by RFC 2505.
	pub(crate) const MAILB_higher: u8 = 0x00;
	pub(crate) const MAILB_lower: u8 = 253;
	pub(crate) const MAILB: Self = Self([Self::MAILB_higher, Self::MAILB_lower]);

	/// `MAILA`.
	///
	/// This is a `QTYPE` and is only valid in a query section.
	///
	/// Clarified in RFC 6895, Section 3.1, Paragraph 3.
	///
	/// Defined in RFC 883 and made obsolete in RFC 973.
	pub(crate) const MAILA_higher: u8 = 0x00;
	pub(crate) const MAILA_lower: u8 = 254;
	pub(crate) const MAILA: Self = Self([Self::MAILA_higher, Self::MAILA_lower]);

	/// `*`.
	///
	/// This is a `QTYPE` and is only valid in a query section.
	///
	/// Returns all records of all types currently cached for a domain name from a name server; if no records are cached then the request will be forwarded on.
	///
	/// Also known as `ANY` and as `ALL`.
	///
	/// Defined in RFC 1035, clarified in RFC 6895, Section 3.1, Paragraph 3.
	pub(crate) const Asterisk_higher: u8 = 0x00;
	pub(crate) const Asterisk_lower: u8 = 255;
	pub(crate) const Asterisk: Self = Self([Self::Asterisk_higher, Self::Asterisk_lower]);
}
