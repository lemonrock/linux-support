// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// For the record types:-
///
/// * MX.
/// * KX.
/// * IPSECKEY.
/// * NID.
/// * L32.
/// * L64.
/// * LP.
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct MultiplePrioritizedThenSortedRecords<OR: OwnedRecord + Ord>
{
	records: Vec<MultiplePrioritizedThenSortedRecordsItem<OR>>,
}

impl<OR: OwnedRecord> OwnedRecords<OR> for MultiplePrioritizedThenSortedRecords<OR>
{
}

impl<OR: OwnedRecord> MultiplePrioritizedThenSortedRecords<OR>
{
	#[inline(always)]
	pub(crate) const fn new(records: Vec<MultiplePrioritizedThenSortedRecordsItem<OR>>) -> Self
	{
		Self
		{
			records
		}
	}
	
	/// Iterate.
	#[inline(always)]
	pub fn iterate<'a>(&'a self) -> MultiplePrioritizedThenSortedRecordsIterator<'a, OR>
	{
		MultiplePrioritizedThenSortedRecordsIterator
		{
			source: &self.records,
			
			next_priority_starts_at_index: 0
		}
	}
}
