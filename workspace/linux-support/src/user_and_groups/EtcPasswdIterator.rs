// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Iterates records (entries) in `/etc/passwd`.
pub struct EtcPasswdIterator<'a>
{
	memchr: Memchr<'a>,
	bytes: &'a [u8],
	last_end_of_line: usize,
}

impl<'a> Iterator for EtcPasswdIterator<'a>
{
	type Item = Result<EtcPasswdRecord<'a>, EtcPasswdParseError>;

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

impl<'a> EtcPasswdIterator<'a>
{
	#[inline(always)]
	fn next_record(&mut self) -> Result<EtcPasswdRecord<'a>, EtcPasswdParseError>
	{
		use self::EtcPasswdParseError::*;

		let next_needle = self.memchr.next().ok_or(MissingLastLineFeed)?;

		let line = &self.bytes[self.last_end_of_line .. next_needle];
		self.last_end_of_line = next_needle + 1;

		let mut fields = line.splitn(7, |byte| *byte == b':');

		Ok
		(
			EtcPasswdRecord
			{
				raw_user_name: fields.next().ok_or(MissingNameField)?,
				user_identifier:
				{
					let password = fields.next().ok_or(MissingPasswordField)?;
					if unlikely!(password != b"x")
					{
						return Err(PasswordFieldIsInvalid)
					}
					UserIdentifier::from_bytes(fields.next().ok_or(MissingUidField)?).map_err(|cause| ParseNumberUid(cause))?
				},
				group_identifier: GroupIdentifier::from_bytes(fields.next().ok_or(MissingGidField)?).map_err(|cause| ParseNumberGid(cause))?,
				gecos: fields.next().ok_or(MissingGecosField)?,
				raw_home_directory: fields.next().ok_or(MissingHomeDirectoryField)?,
				raw_shell: fields.next().ok_or(MissingShellField)?,
			}
		)
	}
}
