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
	/// Get a folder path for the current process (`process_identifier` is `0`) or another process.
	#[inline(always)]
	pub fn process_file_path(&self, process_identifier: impl ProcessIdentifier, relative_path: &str) -> PathBuf
	{
		self.file_path(&process_identifier.to_file_name()).append(relative_path)
	}

	/// Get a file path within the ProcPath, `/proc/sys/fs/<file_name>`.
	#[inline(always)]
	pub fn sys_fs_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_file_path("fs").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc/sys/user/<file_name>`.
	#[inline(always)]
	pub fn sys_user_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_file_path("user").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc/sys/kernel/<file_name>`.
	#[inline(always)]
	pub fn sys_kernel_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_file_path("kernel").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc/sys/<file_name>`.
	#[inline(always)]
	pub fn sys_file_path(&self, file_name: &str) -> PathBuf
	{
		self.file_path("sys").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc`.
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
