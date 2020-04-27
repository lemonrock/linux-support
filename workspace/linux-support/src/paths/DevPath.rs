// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents `/dev`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct DevPath(PathBuf);

impl Default for DevPath
{
	#[inline(always)]
	fn default() -> Self
	{
		DevPath(PathBuf::from("/dev"))
	}
}

impl DevPath
{
	/// `/dev/null`.
	#[inline(always)]
	pub fn null(&self) -> PathBuf
	{
		self.file_path("null")
	}

	/// `/dev/zero`.
	#[inline(always)]
	pub fn zero(&self) -> PathBuf
	{
		self.file_path("zero")
	}

	#[inline(always)]
	fn file_path(&self, file_name: &str) -> PathBuf
	{
		self.path().append(file_name)
	}

	#[inline(always)]
	fn path(&self) -> PathBuf
	{
		self.0.to_owned()
	}
}
