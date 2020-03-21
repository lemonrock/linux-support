// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents `/proc`.
///
/// Frankly, there are files in `/proc` that really belong in `/sys`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ProcPath(PathBuf);

impl Default for ProcPath
{
	#[inline(always)]
	fn default() -> Self
	{
		ProcPath(PathBuf::from("/proc"))
	}
}

impl ProcPath
{
	/// Get a folder path for the current process (`process` is `0`) or another process.
	#[inline(always)]
	pub fn process_folder_path(&self, process: pid_t, relative_path: &str) -> PathBuf
	{
		let mut path = if process == 0
		{
			self.file_path("self")
		}
		else
		{
			self.file_path(&format!("{}", process))
		};
		path.push(relative_path);
		path
	}

	/// Get a file path within the ProcPath, `/proc`.
	#[inline(always)]
	pub fn file_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.path();
		path.push(file_name);
		path
	}

	#[inline(always)]
	fn path(&self) -> PathBuf
	{
		self.0.to_owned()
	}
}
