// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Groups.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Groups(BTreeSet<GroupIdentifier>);

impl Deref for Groups
{
	type Target = BTreeSet<GroupIdentifier>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl FromBytes for Groups
{
	type Error = ProcessStatusStatisticParseError;

	#[inline(always)]
	fn from_bytes(value: &[u8]) -> Result<Self, Self::Error>
	{
		let mut groups = BTreeSet::new();
		for value in value.split(|byte| *byte == b' ')
		{
			let was_added_for_the_first_time = groups.insert(GroupIdentifier::from_bytes(value)?);
			if unlikely!(!was_added_for_the_first_time)
			{
				return Err(ProcessStatusStatisticParseError::DuplicatedStatisticValue)
			}
		}
		Ok(Self(groups))
	}
}
