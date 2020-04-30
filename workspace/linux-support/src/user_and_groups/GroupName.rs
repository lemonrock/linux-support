// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A group name.
///
/// Defaults to `root`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct GroupName(CString);

impl Default for GroupName
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(CString::new(b"root".to_vec()).unwrap())
	}
}

impl GroupName
{
	/// Equals raw name?
	#[inline(always)]
	pub fn equals_raw_name(&self, raw_name: &[u8]) -> bool
	{
		self.0.as_bytes() == raw_name
	}
}
