// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// For the record type:-
///
/// * NAPTR.
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct MultipleOrderedThenPrioritizedThenUnsortedRecords<OR: OwnedRecord>
{
	records: Vec<MultipleOrderedThenPrioritizedThenUnsortedRecordsItem<OR>>,
}

impl<OR: OwnedRecord> OwnedRecords<OR> for MultipleOrderedThenPrioritizedThenUnsortedRecords<OR>
{
}

impl<OR: OwnedRecord> MultipleOrderedThenPrioritizedThenUnsortedRecords<OR>
{
	#[inline(always)]
	pub(crate) const fn new(records: Vec<MultipleOrderedThenPrioritizedThenUnsortedRecordsItem<OR>>) -> Self
	{
		Self
		{
			records
		}
	}
	
	/// Iterate.
	#[inline(always)]
	pub fn iterate<'a>(&'a self) -> MultipleOrderedThenPrioritizedThenUnsortedRecordsIterator<'a, OR>
	{
		MultipleOrderedThenPrioritizedThenUnsortedRecordsIterator
		{
			source: &self.records,
			
			next_order_starts_at_index: 0
		}
	}
}
