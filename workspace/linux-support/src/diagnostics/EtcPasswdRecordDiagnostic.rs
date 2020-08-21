// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Etc passwd record.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct EtcPasswdRecordDiagnostic
{
	/// Raw value.
	pub raw_user_name: Vec<u8>,
	
	/// Group identifier.
	pub group_identifier: GroupIdentifier,
	
	/// GECOS ("General Electric Comprehensive Operating System", also known as GCOS when owned by Honeywell) or comment field.
	///
	/// One of the original designers of Unix, Denis Ritchie, has, paraphrased, called this field 'not elegent' - it was originally a hack.
	///
	/// Often empty, might be the same as `UserName` or comma-separated, eg `Linux User,,,`.
	pub gecos: Vec<u8>,
	
	/// Raw value.
	pub raw_home_directory: Vec<u8>,
	
	/// Raw value.
	pub raw_shell: Vec<u8>,
}

impl<'a> From<EtcPasswdRecord<'a>> for EtcPasswdRecordDiagnostic
{
	#[inline(always)]
	fn from(value: EtcPasswdRecord<'a>) -> Self
	{
		Self
		{
			raw_user_name: value.raw_user_name.to_vec(),
			group_identifier: value.group_identifier,
			gecos: value.raw_user_name.to_vec(),
			raw_home_directory: value.raw_user_name.to_vec(),
			raw_shell: value.raw_user_name.to_vec()
		}
	}
}
