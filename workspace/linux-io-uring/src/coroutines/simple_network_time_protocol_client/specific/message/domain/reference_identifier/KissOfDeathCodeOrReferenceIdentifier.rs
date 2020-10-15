// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
pub(crate) union KissOfDeathCodeOrReferenceIdentifier
{
	/// Value of field if `stratum` is `PacketStratum::UnspecifiedOrInvalid`.
	pub(crate) kiss_of_death_code: KissOfDeathCode,
	
	/// Value of field if `stratum` is `PacketStratum::PrimaryServer`.
	pub(crate) reference_identifier: ReferenceIdentifier,

	/// Value of field for Internet Protocol version 4 secondary servers (`stratum` is in the range `PacketStratum::SecondaryServer0` to `PacketStratum::SecondaryServer15` inclusive.
	///
	/// Is the server that provides the source of time data.
	secondary_internet_protocol_version_4_address: in_addr,

	/// Value of field for Internet Protocol version 6 secondary servers (`stratum` is in the range `PacketStratum::SecondaryServer0` to `PacketStratum::SecondaryServer15` inclusive.
	///
	/// Is the server that provides the source of time data.
	secondary_internet_protocol_version_6_address_md5_hash_first_32_bits: BigEndianU32,
	
	pub(crate) unspecified: [u8; 4],
}
