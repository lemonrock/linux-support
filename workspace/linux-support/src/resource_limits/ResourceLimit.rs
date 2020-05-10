// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a finite quantity or infinite (ie no) limit for a resource.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum ResourceLimit
{
	/// A finite limit; may be zero.
	Finite(u64),

	/// An infinite limit, ie no limit.
	///
	/// Not all resources support an infinite limit.
	///
	/// Also used to signify 'true' for resources which have an on-off setting.
	Infinite,
}

impl ResourceLimit
{
	const Infinity: rlim_t = RLIM_INFINITY as rlim_t;

	/// Obtains the maximum number of file descriptors (`/proc/sys/fs/nr_open`) as a finite resource limit.
	///
	/// Default is 1,048,576.
	pub fn maximum_number_of_open_file_descriptors(proc_path: &ProcPath) -> Result<ResourceLimit, io::Error>
	{
		Self::nr_open_file_path(proc_path).read_value().map(ResourceLimit::Finite)
	}

	/// Sets the maximum number of file descriptors (`/proc/sys/fs/nr_open`).
	///
	/// Default is 1,048,576.
	#[inline(always)]
	pub fn set_maximum_number_of_open_file_descriptors(proc_path: &ProcPath, value: u64) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/fs/nr_open");
		let file_path = Self::nr_open_file_path(proc_path);

		if file_path.exists()
		{
			file_path.write_value(value)
		}
		else
		{
			Ok(())
		}
	}

	#[inline(always)]
	fn nr_open_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_fs_file_path("nr_open")
	}

	/// Value.
	#[inline(always)]
	pub fn value(&self) -> u64
	{
		use self::ResourceLimit::*;

		match *self
		{
			Finite(limit) => limit,
			Infinite => u64::MAX,
		}
	}

	/// Convert a value to a ResourceLimit.
	#[inline(always)]
	pub fn convert(value: rlim_t) -> ResourceLimit
	{
		use self::ResourceLimit::*;

		if value >= Self::Infinity
		{
			Infinite
		}
		else
		{
			Finite(value)
		}
	}

	/// Unwrap.
	#[inline(always)]
	pub fn unwrap(&self) -> rlim_t
	{
		use self::ResourceLimit::*;

		match *self
		{
			Finite(limit) =>
			{
				assert!(limit < Self::Infinity, "limit '{}' equals or exceeds Infinity '{}'", limit, Self::Infinity);
				limit
			},
			Infinite => Self::Infinity
		}
	}

	/// Is this value finite?
	#[inline(always)]
	pub fn is_finite(&self) -> bool
	{
		match *self
		{
			ResourceLimit::Finite(_) => true,
			_ => false,
		}
	}

	/// Is this value infinite?
	#[inline(always)]
	pub fn is_infinite(&self) -> bool
	{
		match *self
		{
			ResourceLimit::Infinite => true,
			_ => false,
		}
	}
}
