// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A driver name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct DriverName(#[serde(with = "serde_bytes")] Box<[u8]>);

impl<'a> From<&'a OsStr> for DriverName
{
	#[inline(always)]
	fn from(value: &'a OsStr) -> Self
	{
		Self(value.as_bytes().to_vec().into_boxed_slice())
	}
}

impl Into<OsString> for DriverName
{
	#[inline(always)]
	fn into(self) -> OsString
	{
		OsString::from_vec(self.0.to_vec())
	}
}

impl<'a> Into<OsString> for &'a DriverName
{
	#[inline(always)]
	fn into(self) -> OsString
	{
		OsString::from_vec(self.0.clone().to_vec())
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for &'a DriverName
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		let mut bytes = self.0.clone().to_vec();
		bytes.push(b'\n');
		Cow::from(bytes)
	}
}
