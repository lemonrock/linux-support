// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// For record types such as:-
///
/// * `TXT`.
///
/// Where the original upstream sort order needs to be retained.
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct MultipleUnsortedRecords<OR: OwnedRecord + Ord>
{
	records: Vec<OR>,
}

impl<OR: OwnedRecord + Ord> MultipleUnsortedRecords<OR>
{
	#[inline(always)]
	pub(crate) const fn new(records: Vec<OR>) -> Self
	{
		Self
		{
			records
		}
	}
	
	/// Iterate.
	#[inline(always)]
	pub fn iterate<'a>(&'a self) -> MultipleUnsortedRecordsIterator<'a, OR>
	{
		MultipleUnsortedRecordsIterator
		{
			source: &self.records,
			
			next_index: 0,
		}
	}
}
