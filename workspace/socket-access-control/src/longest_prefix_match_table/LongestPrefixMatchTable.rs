// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A longest prefix match table.
#[repr(transparent)]
pub struct LongestPrefixMatchTable<IPA: InternetProtocolAddress, Value>(Point<Value>, PhantomData<IPA>);

impl<IPA: InternetProtocolAddress, Value> LongestPrefixMatchTable<IPA, Value>
{
	/// Populates by sorting the entries from those with shortest mask length to those with the longest.
	///
	/// If exact matches are wanted, retain `subnets`.
	pub fn new(subnets: &BTreeMap<InternetProtocolAddressWithMask<IPA>, Arc<Value>>) -> Self
	{
		let mut root = Point::new();
		for (internet_protocol_address_with_mask, value) in subnets.iter()
		{
			root.add_from_root(internet_protocol_address_with_mask, value)
		}
		Self(root, PhantomData)
	}
	
	/// Longest match.
	#[inline(always)]
	pub fn longest_match<'a>(&'a self, internet_protocol_address: &IPA) -> Option<&'a Arc<Value>>
	{
		let remaining_bytes = internet_protocol_address.bytes();
		self.0.longest_match_next(remaining_bytes)
	}
}
