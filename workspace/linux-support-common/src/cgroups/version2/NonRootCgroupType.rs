// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The type of a non-root version 2 cgroup.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NonRootCgroupType
{
	/// A normal valid domain cgroup.
	Domain,

	/// A threaded domain cgroup which is serving as the root of a threaded subtree.
	ThreadedDomain,

	/// A cgroup which is in an invalid state.
	/// It can't be populated or have controllers enabled.
	/// It may be allowed to become a threaded cgroup.
	InvalidDomain,

	/// A threaded cgroup which is a member of a threaded subtree.
	Threaded,
}

impl FromStr for NonRootCgroupType
{
	type Err = ();

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		use self::NonRootCgroupType::*;
		let variant = match s
		{
			"domain" => Domain,
			"domain_threaded" => ThreadedDomain,
			"domain_invalid" => InvalidDomain,
			"threaded" => Threaded,

			_ => return Err(())
		};
		Ok(variant)
	}
}

impl NonRootCgroupType
{
	#[inline(always)]
	fn from_file(file_path: &Path) -> Result<Self, NonRootCgroupTypeParseError>
	{
		let contents = read_to_string(file_path)?;
		Self::from_file_contents(contents)
	}

	#[inline(always)]
	fn from_file_contents(mut contents: String) -> Result<Self, NonRootCgroupTypeParseError>
	{
		use self::NonRootCgroupTypeParseError::*;

		if unlikely!(!contents.ends_with('\n'))
		{
			return Err(DoesNotEndWithLineFeed)
		}
		contents.truncate(contents.len() - 1);
		let name = contents;

		Self::from_str(&name).map_err(|_: ()| InvalidTypeName { name })
	}

	/// String description that matches that used by Linux.
	#[inline(always)]
	pub fn to_str(self) -> &'static str
	{
		use self::NonRootCgroupType::*;
		match self
		{
			Domain => "domain",
			ThreadedDomain => "domain_threaded",
			InvalidDomain => "domain_invalid",
			Threaded => "threaded",
		}
	}
}
