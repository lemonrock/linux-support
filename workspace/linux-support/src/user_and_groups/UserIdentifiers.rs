// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// User identifiers (`uid`s).
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserIdentifiers
{
	/// Real user identifier (`uid`).
	pub real: UserIdentifier,

	/// Effective user identifier (`uid`).
	pub effective: UserIdentifier,

	/// Saved set user identifier (`uid`).
	pub saved_set: UserIdentifier,

	/// File system user identifier (`uid`).
	pub file_system: UserIdentifier,
}

impl FromBytes for UserIdentifiers
{
	type Error = StatusStatisticParseError;

	#[inline(always)]
	fn from_bytes(value: &[u8]) -> Result<Self, Self::Error>
	{
		#[inline(always)]
		fn parse_subsequent<'a>(iterator: &mut impl Iterator<Item=&'a [u8]>) -> Result<UserIdentifier, StatusStatisticParseError>
		{
			if let Some(effective) = iterator.next()
			{
				Ok(UserIdentifier::from_bytes(effective)?)
			}
			else
			{
				Err(StatusStatisticParseError::InvalidSeparator)
			}
		}

		let mut iterator = value.split_bytes_n(4, b'\t');

		Ok
		(
			Self
			{
				real: UserIdentifier::from_bytes(iterator.next().unwrap())?,
				effective: parse_subsequent(&mut iterator)?,
				saved_set: parse_subsequent(&mut iterator)?,
				file_system: parse_subsequent(&mut iterator)?,
			}
		)
	}
}
