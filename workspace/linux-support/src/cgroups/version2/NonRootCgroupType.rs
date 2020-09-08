// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The type of a non-root version 2 cgroup.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum NonRootCgroupType
{
	/// A normal valid domain cgroup.
	///
	/// Will never validly exist under `Threaded` or `ThreadedDomain`.
	Domain,

	/// A threaded domain cgroup which is serving as the root of a threaded subtree.
	///
	/// A `Domain` cgroup is turned into a `ThreadedDomain` when either:-
	///
	/// * one of its children becomes `threaded` (its `cgroup.type` is written with `threaded`; not possible if non-threaded (ie domain) controllers are enabled in the parent's `cgroup.subtree_control` tree), or;
	/// * threaded controllers are enabled in the `cgroup.subtree_control` file while there are processes in the cgroup.
	///
	/// Even though threads may be in different child `Threaded` cgroups, the associated processes for these threads reside in the `ThreadedDomain`'s `cgroup.procs` file.
	///
	/// A `ThreadedDomain` is only permitted to have threaded controllers.
	ThreadedDomain,

	/// A cgroup which is in an invalid state.
	/// It can't be populated or have controllers enabled.
	/// It may be allowed to become a `Threaded` cgroup.
	///
	/// Can occur when there are two children of a non-root cgroup and one of the children has been made `threaded` and the other has not (its `cgroup.type` is still `domain`).
	InvalidDomain,

	/// A threaded cgroup which is a member of a threaded subtree.
	Threaded,
}

impl FromBytes for NonRootCgroupType
{
	type Error = ParseNonRootCgroupTypeError;

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		use self::NonRootCgroupType::*;
		let variant = match bytes
		{
			b"domain" => Domain,
			b"domain threaded" => ThreadedDomain,
			b"domain invalid" => InvalidDomain,
			b"threaded" => Threaded,

			_ => return Err(ParseNonRootCgroupTypeError::UnknownVariant(bytes.to_vec()))
		};
		Ok(variant)
	}
}

impl NonRootCgroupType
{
	/// String description that matches that used by Linux.
	#[inline(always)]
	pub fn to_str(self) -> &'static str
	{
		use self::NonRootCgroupType::*;
		match self
		{
			Domain => "domain",
			ThreadedDomain => "domain threaded",
			InvalidDomain => "domain invalid",
			Threaded => "threaded",
		}
	}
}
