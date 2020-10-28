// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Known to exist.
#[derive(Debug)]
pub struct Exists<Record: Sized + Debug>(PriorityToSortedWeightedRecordsMap<Record>);

impl<Record: Sized + Debug> Exists<Record>
{
	/// Iterate destructively.
	///
	/// The iterator will always contain at least one instance of `SortedWeightedRecords` which will always contain at least one `Record`.
	///
	/// Does not implement `IntoIterator` as we do not know `IntoIterator::IntoIter`'s type.
	#[inline(always)]
	pub fn into_iter(self) -> impl Iterator<Item=Rc<Record>>
	{
		(self.0).0.into_iter().flat_map(|(_priority, sorted_weighted_records)| sorted_weighted_records)
	}
}
