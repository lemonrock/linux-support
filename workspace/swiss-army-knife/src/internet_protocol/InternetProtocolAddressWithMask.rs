// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An Internet Protocol (IP) version 4 or version 4 address with a mask.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InternetProtocolAddressWithMask<IPA: InternetProtocolAddress>
{
	/// An Internet Protocol (IP) version 4 or version 4 address.
	pub internet_protocol_address: IPA,

	/// * From 1 inclusive to 32 inclusive for an inclusive for an Internet Protocol version 4 address.
	/// * From 1 inclusive to 128 inclusive for an inclusive for an Internet Protocol version 6 address.
	pub mask_length_in_bits: NonZeroU8,
}

impl<IPA: InternetProtocolAddress> PartialOrd for InternetProtocolAddressWithMask<IPA>
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl<IPA: InternetProtocolAddress> Ord for InternetProtocolAddressWithMask<IPA>
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.mask_length_in_bits.cmp(&rhs.mask_length_in_bits).then(self.internet_protocol_address.cmp(&rhs.internet_protocol_address))
	}
}

impl<IPA: InternetProtocolAddress> Default for InternetProtocolAddressWithMask<IPA>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::local_host()
	}
}

impl<IPA: InternetProtocolAddress> InternetProtocolAddressWithMask<IPA>
{
	/// New instance.
	#[inline(always)]
	pub fn new(internet_protocol_address: IPA, mask_length_in_bits: NonZeroU8) -> Self
	{
		assert!(IPA::InclusiveMaximumPrefixLength <= mask_length_in_bits.get(), "mask_length_in_bits {} exceeds InclusiveMaximumPrefixLength {}", mask_length_in_bits, IPA::InclusiveMaximumPrefixLength);
		Self
		{
			internet_protocol_address,
			mask_length_in_bits,
		}
	}
	
	/// Local host.
	pub fn local_host() -> Self
	{
		Self
		{
			internet_protocol_address: IPA::LocalHost,
			mask_length_in_bits: unsafe { NonZeroU8::new_unchecked(IPA::InclusiveMaximumPrefixLength) },
		}
	}
}
