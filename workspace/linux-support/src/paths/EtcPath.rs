// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents `/etc`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct EtcPath(PathBuf);

impl Default for EtcPath
{
	#[inline(always)]
	fn default() -> Self
	{
		EtcPath(PathBuf::from("/etc"))
	}
}

impl EtcPath
{
	/// `/etc/zoneinfo/<timezone_file_name>`.
	#[inline(always)]
	pub fn zoneinfo(&self, timezone_file_name: &str) -> PathBuf
	{
		self.file_path("zoneinfo").append(timezone_file_name)
	}

	/// `/etc/passwd`.
	#[inline(always)]
	pub fn passwd(&self) -> PathBuf
	{
		self.file_path("passwd")
	}

	/// `/etc/group`.
	#[inline(always)]
	pub fn group(&self) -> PathBuf
	{
		self.file_path("group")
	}

	/// File path.
	#[inline(always)]
	pub fn file_path(&self, file_name: &str) -> PathBuf
	{
		self.path().append(file_name)
	}

	#[inline(always)]
	fn path(&self) -> PathBuf
	{
		self.0.to_owned()
	}
}
