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

	/// Get a file path within the ProcPath, `/proc/sys/fs/<file_name>`.
	#[inline(always)]
	pub fn sys_fs_file_path(&self, file_name: &str) -> PathBuf
	{
		let mut fs_folder_path = self.sys_file_path("fs");
		fs_folder_path.push(file_name);
		fs_folder_path
	}

	/// Get a file path within the ProcPath, `/proc/sys/user/<file_name>`.
	#[inline(always)]
	pub fn sys_user_file_path(&self, file_name: &str) -> PathBuf
	{
		let mut user_folder_path = self.sys_file_path("user");
		user_folder_path.push(file_name);
		user_folder_path
	}

	/// Get a file path within the ProcPath, `/proc/sys/kernel/<file_name>`.
	#[inline(always)]
	pub fn sys_kernel_file_path(&self, file_name: &str) -> PathBuf
	{
		let mut kernel_folder_path = self.sys_file_path("kernel");
		kernel_folder_path.push(file_name);
		kernel_folder_path
	}

	/// Get a file path within the ProcPath, `/proc/sys/<file_name>`.
	#[inline(always)]
	pub fn sys_file_path(&self, file_name: &str) -> PathBuf
	{
		let mut sys_folder_path = self.file_path("sys");
		sys_folder_path.push(file_name);
		sys_folder_path
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
