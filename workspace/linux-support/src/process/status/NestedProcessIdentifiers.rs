// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process Identifiers in nesting order.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct NestedProcessIdentifiers(IndexSet<ProcessIdentifier>);

impl Deref for NestedProcessIdentifiers
{
	type Target = IndexSet<ProcessIdentifier>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl FromBytes for NestedProcessIdentifiers
{
	type Error = StatusStatisticParseError;
	
	#[inline(always)]
	fn from_bytes(value: &[u8]) -> Result<Self, Self::Error>
	{
		let mut process_identifiers = IndexSet::new();
		for value in value.split_bytes(b'\t')
		{
			let was_added_for_the_first_time = process_identifiers.insert(ProcessIdentifier::parse_decimal_number(value)?);
			if unlikely!(!was_added_for_the_first_time)
			{
				return Err(StatusStatisticParseError::DuplicatedStatisticValue)
			}
		}
		Ok(Self(process_identifiers))
	}
}
