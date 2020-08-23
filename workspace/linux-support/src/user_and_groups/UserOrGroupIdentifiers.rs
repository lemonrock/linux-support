// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// User identifiers (`uid`s) or group identifiers (`gid`s).
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserOrGroupIdentifiers<UOGI: UserOrGroupIdentifier>
{
	/// Real user identifier (`uid`) or group identifier (`gid`).
	pub real: UOGI,
	
	/// Effective user identifier (`uid`) or group identifier (`gid`).
	pub effective: UOGI,
	
	/// Saved set user identifier (`uid`) or group identifier (`gid`).
	pub saved_set: UOGI,
	
	/// File system user identifier (`uid`) or group identifier (`gid`).
	pub file_system: UOGI,
}

impl<UOGI: UserOrGroupIdentifier> FromBytes for UserOrGroupIdentifiers<UOGI>
{
	type Error = StatusStatisticParseError;
	
	#[inline(always)]
	fn from_bytes(value: &[u8]) -> Result<Self, Self::Error>
	{
		#[inline(always)]
		fn parse_subsequent<'a, UOGI: UserOrGroupIdentifier>(iterator: &mut impl Iterator<Item=&'a [u8]>) -> Result<UOGI, StatusStatisticParseError>
		{
			if let Some(effective) = iterator.next()
			{
				Ok(UOGI::from_bytes(effective)?)
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
					real: UOGI::from_bytes(iterator.next().unwrap())?,
					effective: parse_subsequent(&mut iterator)?,
					saved_set: parse_subsequent(&mut iterator)?,
					file_system: parse_subsequent(&mut iterator)?,
				}
			)
	}
}
