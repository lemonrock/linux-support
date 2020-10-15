// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See RFC 6895, Section 3.1, paragraph 3.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
pub(crate) struct MetaType(pub(crate) BigEndianU16);

impl MetaType
{
	/// Defined in RFC 6891.
	pub(crate) const OPT_higher: u8 = 0x00;
	pub(crate) const OPT_lower: u8 = 41;
	pub(crate) const OPT: Self = Self([Self::OPT_higher, Self::OPT_lower]);

	/// Defined in RFC 2930.
	pub(crate) const TKEY_higher: u8 = 0x00;
	pub(crate) const TKEY_lower: u8 = 249;
	pub(crate) const TKEY: Self = Self([Self::TKEY_higher, Self::TKEY_lower]);

	/// Defined in RFC 2845.
	pub(crate) const TSIG_higher: u8 = 0x00;
	pub(crate) const TSIG_lower: u8 = 250;
	pub(crate) const TSIG: Self = Self([Self::TSIG_higher, Self::TSIG_lower]);
}
