// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// For record types such as:-
///
/// * `A`.
/// * `AAAA`.
/// * `NS`.
pub struct MultipleSortedRecords<OR: OwnedRecord + Ord>
{
	records: BTreeSet<OR>,
}

impl<PR: ParsedRecord<OrderPriorityAndWeight=(), OwnedRecord=OR>, OR: OwnedRecord + Ord> From<OwnerNameToRecordsValue<PR>> for MultipleSortedRecords<OR>
{
	#[inline(always)]
	fn from(value: OwnerNameToRecordsValue<PR>) -> Self
	{
		Self
		{
			records:
			{
				let mut records = BTreeSet::new();
				for (record, _) in value.records
				{
					records.insert(record.into_owned_record())
				}
				records
			},
		}
	}
}

impl<OR: OwnedRecord + Ord> MultipleSortedRecords<OR>
{
	#[inline(always)]
	pub(crate) fn single(owned_record: OR) -> Self
	{
		Self
		{
			records: btreeset!
			{
				owned_record
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn add(&mut self, owned_record: OR)
	{
		self.records.insert(owned_record);
	}
}
