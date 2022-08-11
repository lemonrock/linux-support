// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a system call number.
#[derive(Copy, Clone)]
pub union UnconstrainedSystemCallNumber
{
	/// Known value.
	pub known: SystemCallNumber,

	/// As returned for `SIGSYS` in `signalfd_siginfo`.
	pub signalfd: i32,

	/// Any value.
	other: usize,
}

impl Debug for UnconstrainedSystemCallNumber
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "SystemCallNumber({})", unsafe { self.other })
	}
}

impl PartialEq for UnconstrainedSystemCallNumber
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.other == other.other }
	}
}

impl Eq for UnconstrainedSystemCallNumber
{
}

impl PartialOrd for UnconstrainedSystemCallNumber
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.other.partial_cmp(&other.other) }
	}
}

impl Ord for UnconstrainedSystemCallNumber
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.other.cmp(&other.other) }
	}
}

impl Hash for UnconstrainedSystemCallNumber
{
	#[inline(always)]
	fn hash<H>(&self, state: &mut H)
	where H: Hasher
	{
		unsafe { self.other.hash(state) }
	}
}

impl From<i32> for UnconstrainedSystemCallNumber
{
	#[inline(always)]
	fn from(signalfd: i32) -> Self
	{
		Self
		{
			signalfd
		}
	}
}
