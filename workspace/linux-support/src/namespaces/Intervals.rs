// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is a poor man's interval tree.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Intervals<V>(BTreeMap<u32, (NonZeroU32, V)>);

impl<V> Deref for Intervals<V>
{
	type Target = BTreeMap<u32, (NonZeroU32, V)>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<V> Intervals<V>
{
	pub fn add_interval(&mut self, inclusive_start: u32, length: NonZeroU32, value: V)
	{
		let length_u64 = length.get() as u64;
		const MaximumInclusiveIdentifierExcludingNoUser: u64 = (i32::MAX - 1) as u64;

		debug_assert!((inclusive_start as u64 + length_u64) <= MaximumInclusiveIdentifierExcludingNoUser, "length is too long for inclusive_start");

		if unlikely!(self.0.contains_key(&inclusive_start))
		{
			panic!("Already added `{}`", inclusive_start);
		}

		let mut previous_range = self.0.range(0 .. inclusive_start);
		if let Some((&previous_inclusive_start, previous_length_and_value)) = previous_range.next_back()
		{
			let previous_inclusive_start = previous_inclusive_start;
			let previous_length = previous_length_and_value.0.get();
			if unlikely!(inclusive_start < (previous_inclusive_start + previous_length))
			{
				panic!("Already added previous")
			}
		}

		let mut next_range = self.0.range(inclusive_start .. (inclusive_start + length.get()));
		if next_range.next().is_some()
		{
			panic!("Already added next")
		}

		self.0.insert(inclusive_start, (length, value));
	}
}
