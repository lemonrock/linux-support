// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// ```bash
///  <------- NLA_HDRLEN ------> <-- NLA_ALIGN(payload)-->
/// +---------------------+- - -+- - - - - - - - - -+- - -+
/// |        Header       | Pad |     Payload       | Pad |
/// |   (struct nlattr)   | ing |                   | ing |
/// +---------------------+- - -+- - - - - - - - - -+- - -+
///  <-------------- nlattr->nla_len -------------->
/// ```
#[repr(C, align(4))]
pub(crate) struct nlattr
{
	/// ```bash
	/// +---+---+-------------------------------+
	/// | N | O | Attribute Type                |
	/// +---+---+-------------------------------+
	/// N := Carries nested attributes (`NLA_F_NESTED`).
	/// O := Payload stored in network byte order (`NLA_F_NET_BYTEORDER`).
	///
	/// Note: The N and O flags are mutually exclusive.
	/// ```
	pub(crate) nla_type: u16,
	pub(crate) nla_len: u16,
}

impl nlattr
{
	const fn NLA_ALIGN(len: u16) -> u16
	{
		(len + Self::NLA_ALIGNTO - 1) & !(Self::NLA_ALIGNTO - 1)
	}
	
	/// A 14-bit value.
	const fn attribute_type(self) -> u16
	{
		self.nla_type & Self::NLA_TYPE_MASK
	}
	
	const fn is_nested(self) -> bool
	{
		self.nla_type & Self::NLA_F_NESTED != 0
	}
	
	const fn payload_is_stored_in_network_byte_order(self) -> bool
	{
		self.nla_type & Self::NLA_F_NET_BYTEORDER != 0
	}
	
	pub(crate) const NLA_F_NESTED: u16 = 1 << 15;
	
	pub(crate) const NLA_F_NET_BYTEORDER: u16 = 1 << 14;
	
	const NLA_TYPE_MASK: u16 = !(Self::NLA_F_NESTED | Self::NLA_F_NET_BYTEORDER);
	
	const NLA_ALIGNTO: u16 = 4;
	
	/// `4`.
	const NLA_HDRLEN: u16 = Self::NLA_ALIGN(size_of::<nlattr>() as u16);
	
	#[inline(always)]
	pub(crate) fn compute_nla_len<V: Sized>() -> u16
	{
		Self::NLA_HDRLEN + (size_of::<V>() as u16)
	}
}
