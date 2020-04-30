// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Iterates records (entries) in `/etc/group`.
pub struct EtcGroupIterator<'a>
{
	memchr: Memchr<'a>,
	bytes: &'a [u8],
	last_end_of_line: usize,
}

impl<'a> Iterator for EtcGroupIterator<'a>
{
	type Item = Result<EtcGroupRecord<'a>, EtcGroupParseError>;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if unlikely!(self.last_end_of_line == self.bytes.len())
		{
			return None
		}
		Some(self.next_record())
	}
}

impl<'a> EtcGroupIterator<'a>
{
	#[inline(always)]
	fn next_record(&mut self) -> Result<EtcGroupRecord<'a>, EtcGroupParseError>
	{
		use self::EtcGroupParseError::*;

		let next_needle = self.memchr.next().ok_or(MissingLastLineFeed)?;

		let line = &self.bytes[self.last_end_of_line .. next_needle];
		self.last_end_of_line = next_needle + 1;

		let mut fields = line.splitn(7, |byte| *byte == b':');

		Ok
		(
			EtcGroupRecord
			{
				raw_group_name: fields.next().ok_or(MissingNameField)?,
				group_identifier:
				{
					let password = fields.next().ok_or(MissingPasswordField)?;
					if unlikely!(password != b"x")
					{
						return Err(PasswordFieldIsInvalid)
					}
					GroupIdentifier::from_bytes(fields.next().ok_or(MissingGidField)?).map_err(|cause| ParseNumberGid(cause))?
				},
				raw_user_list: fields.next().ok_or(MissingUserListField)?,
			}
		)
	}
}
