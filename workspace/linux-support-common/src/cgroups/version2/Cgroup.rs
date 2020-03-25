// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A version 2 cgroup.
///
/// See <https://www.kernel.org/doc/Documentation/cgroup-v2.txt>.
///
/// By convention, a leaf cgroup is called `leaf` but this is not enforced.
#[derive(Debug, Clone)]
pub enum Cgroup<'a>
{
	/// Root.
	Root,

	/// Non-root.
	NonRoot(NonRootCgroup<'a>),
}

impl<'a> Cgroup<'a>
{
	/// To an owned path.
	#[inline(always)]
	pub fn to_owned_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.to_path(mount_point).into_owned()
	}

	/// To a path.
	#[inline(always)]
	pub fn to_path<'b>(&self, mount_point: &'b CgroupMountPoint) -> Cow<'b, Path>
	{
		use self::Cow::*;
		use self::Cgroup::*;

		match self
		{
			&Root => Borrowed(mount_point.to_path()),

			&NonRoot(ref non_root) => Owned(non_root.to_path(mount_point)),
		}
	}

	/// Returns `None` if this is the root cgroup.
	#[inline(always)]
	pub fn parent(&self) -> Option<&'a Self>
	{
		use self::Cgroup::*;

		match self
		{
			&Root => None,

			&NonRoot(ref non_root) => Some(non_root.parent()),
		}
	}

	/// The set of controllers match the set of controllers in the parents' cgroup's `subtree_control()`.
	#[inline(always)]
	pub fn read_available_controllers(&self, mount_point: &CgroupMountPoint) -> Result<Controllers, ControllersParseError>
	{
		self.read_controllers(mount_point, "cgroup.controllers")
	}

	/// Reads maximum depth.
	///
	/// Since Linux version 4.14.
	#[inline(always)]
	pub fn read_maximum_depth(&self, mount_point: &CgroupMountPoint) -> Result<MaximumNumber, MaximumNumberParseError>
	{
		self.read_maximum_number(mount_point, "cgroup.max.depth")
	}

	/// Writes maximum depth.
	///
	/// Since Linux version 4.14.
	#[inline(always)]
	pub fn write_maximum_depth(&self, mount_point: &CgroupMountPoint, maximum_number: MaximumNumber) -> io::Result<()>
	{
		self.write_maximum_number(mount_point, "cgroup.max.depth", maximum_number)
	}

	/// Reads maximum descendants.
	///
	/// Since Linux version 4.14.
	#[inline(always)]
	pub fn read_maximum_descendants(&self, mount_point: &CgroupMountPoint) -> Result<MaximumNumber, MaximumNumberParseError>
	{
		self.read_maximum_number(mount_point, "cgroup.max.descendants")
	}

	/// Writes maximum descendants.
	///
	/// Since Linux version 4.14.
	#[inline(always)]
	pub fn write_maximum_descendants(&self, mount_point: &CgroupMountPoint, maximum_number: MaximumNumber) -> io::Result<()>
	{
		self.write_maximum_number(mount_point, "cgroup.max.descendants", maximum_number)
	}

	/// List of non-zero process identifiers.
	#[inline(always)]
	pub fn get_process_identifiers(&self, mount_point: &CgroupMountPoint) -> Result<ProcessIdentifiersIterator, ProcessIdentifiersIteratorParseError>
	{
		self.read_process_identifiers(mount_point, "cgroup.procs")
	}

	/// Migrate process to this cgroup.
	#[inline(always)]
	pub fn migrate_process_to_this_cgroup(&self, mount_point: &CgroupMountPoint, process_identifier: ProcessIdentifierKind) -> io::Result<()>
	{
		self.write_process_identifier(mount_point, "cgroup.procs", process_identifier)
	}

	#[inline(always)]
	fn read_process_identifiers(&self, mount_point: &CgroupMountPoint, file_name: &str) -> Result<ProcessIdentifiersIterator, ProcessIdentifiersIteratorParseError>
	{
		let path = self.file_path(mount_point, file_name);
		Ok(ProcessIdentifiersIterator(BufReader::new(File::open(path)?).lines()))
	}

	#[inline(always)]
	fn write_process_identifier(&self, mount_point: &CgroupMountPoint, file_name: &str, process_identifier: ProcessIdentifierKind) -> io::Result<()>
	{
		let path = self.file_path(mount_point, file_name);
		let x: u32 = process_identifier.into();
		path.write_value(x)
	}

	/// Statistics.
	///
	/// Since Linux version 4.14.
	#[inline(always)]
	pub fn read_statistics(&self, mount_point: &CgroupMountPoint) -> Result<Statistics, StatisticsParseError>
	{
		let path = self.file_path(mount_point, "cgroup.stat");
		Statistics::from_file(&path)
	}

	/// Reads enabled controllers.
	#[inline(always)]
	pub fn read_enabled_controllers(&self, mount_point: &CgroupMountPoint) -> Result<Controllers, ControllersParseError>
	{
		self.read_controllers(mount_point, "cgroup.subtree_control")
	}

	/// Changes the enabled controllers.
	///
	/// *Panics* in debug mode if `enable` and `disable` sets intersect.
	#[inline(always)]
	pub fn change_enabled_controllers(&self, mount_point: &CgroupMountPoint, enable: &Controllers, disable: &Controllers) -> io::Result<()>
	{
		debug_assert_eq!(enable.intersection(disable).count(), 0, "There are controllers in both the `enable` and `disable` sets");

		let path = self.file_path(mount_point, "cgroup.subtree_control");
		let line = Controllers::create_change_line(enable, disable);
		path.write_value(line)
	}

	/// List of non-zero process identifiers.
	#[inline(always)]
	pub fn get_thread_identifiers(&self, mount_point: &CgroupMountPoint) -> Result<ProcessIdentifiersIterator, ProcessIdentifiersIteratorParseError>
	{
		self.read_process_identifiers(mount_point, "cgroup.threads")
	}

	/// Migrate thread to this cgroup.
	#[inline(always)]
	pub fn migrate_thread_to_this_cgroup(&self, mount_point: &CgroupMountPoint, thread_identifier: ProcessIdentifierKind) -> io::Result<()>
	{
		self.write_process_identifier(mount_point, "cgroup.threads", thread_identifier)
	}

	#[inline(always)]
	fn read_controllers(&self, mount_point: &CgroupMountPoint, file_name: &str) -> Result<Controllers, ControllersParseError>
	{
		let path = self.file_path(mount_point, file_name);
		Controllers::from_file(&path)
	}

	#[inline(always)]
	fn read_maximum_number(&self, mount_point: &CgroupMountPoint, file_name: &str) -> Result<MaximumNumber, MaximumNumberParseError>
	{
		let path = self.file_path(mount_point, file_name);
		MaximumNumber::from_file(&path)
	}

	#[inline(always)]
	fn write_maximum_number(&self, mount_point: &CgroupMountPoint, file_name: &str, maximum_number: MaximumNumber) -> io::Result<()>
	{
		let path = self.file_path(mount_point, file_name);
		path.write_value(maximum_number)
	}

	#[inline(always)]
	fn file_path(&self, mount_point: &CgroupMountPoint, file_name: impl AsRef<Path>) -> PathBuf
	{
		self.to_owned_path(mount_point).append(file_name)
	}
}
