// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A non-root version 2 cgroup.
#[derive(Debug, Clone)]
pub struct NonRootCgroup<'a>
{
	parent: &'a Cgroup<'a>,
	name: OsString,
}

impl<'a> Into<Cgroup<'a>> for NonRootCgroup<'a>
{
	#[inline(always)]
	fn into(self) -> Cgroup<'a>
	{
		Cgroup::NonRoot(self)
	}
}

impl<'a> NonRootCgroup<'a>
{
	/// To path.
	#[inline(always)]
	pub fn to_path<'b>(&self, mount_point: &'b CgroupMountPoint) -> PathBuf
	{
		self.parent.file_path(mount_point, &self.name)
	}

	/// Returns `None` if this is the root cgroup.
	#[inline(always)]
	pub fn parent(&self) -> &'a Cgroup<'a>
	{
		self.parent
	}

	/// Read type.
	#[inline(always)]
	pub fn read_type(&self, mount_point: &CgroupMountPoint) -> Result<NonRootCgroupType, io::Error>
	{
		self.type_file_path(mount_point).read_value()
	}

	/// Write type.
	#[inline(always)]
	pub fn make_type_threaded(&self, mount_point: &CgroupMountPoint) -> io::Result<()>
	{
		let path = self.type_file_path(mount_point);
		path.write_value(b"threaded\n" as &[u8])
	}

	/// Is populated?
	#[inline(always)]
	pub fn read_events_is_populated(&self, mount_point: &CgroupMountPoint) -> io::Result<bool>
	{
		let bytes = self.events_file_path(mount_point).read_raw_without_line_feed()?;
		match &bytes[..]
		{
			b"populated 0" => Ok(false),
			b"populated 1" => Ok(true),
			_ => Err(io::Error::from(ErrorKind::InvalidData)),
		}
	}

	/// Events file descriptor for epoll.
	///
	/// Caller must close this file descriptor.
	#[inline(always)]
	pub fn events_file_descriptor_for_epoll(&self, mount_point: &CgroupMountPoint) -> io::Result<RawFd>
	{
		let file: File = File::open(self.events_file_path(mount_point))?;
		Ok(file.into_raw_fd())
	}

	#[inline(always)]
	fn type_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cgroup.type")
	}

	#[inline(always)]
	fn events_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cgroup.events")
	}

	#[inline(always)]
	fn file_path(&self, mount_point: &CgroupMountPoint, file_name: &str) -> PathBuf
	{
		self.to_path(mount_point).append(file_name)
	}
}
