// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Etc group record.
///
/// Has a lifetime as it shares its data with the underlying file's bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct EtcGroupRecordDiagnostic
{
	/// Raw value.
	///
	/// Use `self.group_name()` for a lifetime-independent, but cloned, value.
	pub raw_group_name: Vec<u8>,
	
	/// User names.
	pub raw_user_names: Vec<Vec<u8>>,
}

impl<'a> From<EtcGroupRecord<'a>> for EtcGroupRecordDiagnostic
{
	#[inline(always)]
	fn from(value: EtcGroupRecord<'a>) -> Self
	{
		Self
		{
			raw_group_name: value.raw_group_name.to_vec(),
			raw_user_names: value.raw_user_names().map(|raw_user_name| raw_user_name.to_vec()).collect(),
		}
	}
}
