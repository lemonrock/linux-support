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
pub(crate) struct MultiplePrioritizedThenSortedRecords<R: OwnedRecord + Ord>
{
	records: PriorityToSortedRecordsMap<R>,
}

impl<PR: ParsedRecord<OrderPriorityAndWeight=Priority, OwnedRecord=OR>, OR: OwnedRecord + Ord> From<OwnerNameToRecordsValue<PR>> for MultiplePrioritizedThenSortedRecords<OR>
{
	#[inline(always)]
	fn from(value: OwnerNameToRecordsValue<PR>) -> Self
	{
		Self
		{
			records:
			{
				let mut records = PriorityToSortedRecordsMap::default();
				for (record, priority) in value.records
				{
					records.add(priority, record.into_owned_record())
				}
				records
			},
		}
	}
}
