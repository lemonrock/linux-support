// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Group identifiers (GIDs).
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessGroupIdentifiers
{
	/// Real group identifier (GID).
	pub real: GroupIdentifier,

	/// Effective group identifier (GID).
	pub effective: GroupIdentifier,

	/// Saved set group identifier (GID).
	pub saved_set: GroupIdentifier,

	/// File system group identifier (GID).
	pub file_system: GroupIdentifier,
}

impl FromBytes for ProcessGroupIdentifiers
{
	type Error = ProcessStatusStatisticParseError;

	#[inline(always)]
	fn from_bytes(value: &[u8]) -> Result<Self, Self::Error>
	{
		#[inline(always)]
		fn parse_subsequent<'a>(iterator: &mut impl Iterator<Item=&'a [u8]>) -> Result<GroupIdentifier, ProcessStatusStatisticParseError>
		{
			if let Some(effective) = iterator.next()
			{
				Ok(GroupIdentifier::from_bytes(effective)?)
			}
			else
			{
				Err(ProcessStatusStatisticParseError::InvalidSeparator)
			}
		}

		let mut iterator = value.splitn(4, |byte| *byte == b'\t');

		Ok
		(
			Self
			{
				real: GroupIdentifier::from_bytes(iterator.next().unwrap())?,
				effective: parse_subsequent(&mut iterator)?,
				saved_set: parse_subsequent(&mut iterator)?,
				file_system: parse_subsequent(&mut iterator)?,
			}
		)
	}
}
