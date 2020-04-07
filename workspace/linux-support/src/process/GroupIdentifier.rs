// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A group identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct GroupIdentifier(gid_t);

impl From<gid_t> for GroupIdentifier
{
	#[inline(always)]
	fn from(value: gid_t) -> Self
	{
		Self(value)
	}
}

impl Into<gid_t> for GroupIdentifier
{
	#[inline(always)]
	fn into(self) -> gid_t
	{
		self.0
	}
}

impl FromBytes for GroupIdentifier
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn from_bytes(value: &[u8]) -> Result<Self, Self::Error>
	{
		Ok(Self(gid_t::parse_decimal_number(value)?))
	}
}

impl Default for GroupIdentifier
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::current_real()
	}
}

impl UserOrGroupIdentifier for GroupIdentifier
{
	const Zero: Self = GroupIdentifier(0);

	const FileName: &'static str = "gid_map";

	const Root: Self = Self(0);

	#[inline(always)]
	fn current_real() -> Self
	{
		Self(unsafe { getgid() })
	}

	#[inline(always)]
	fn current_effective() -> Self
	{
		Self(unsafe { getegid() })
	}

	#[allow(deprecated)]
	#[inline(always)]
	fn current_real_effective_and_saved_set() -> (Self, Self, Self)
	{
		let mut real: gid_t = unsafe { uninitialized() };
		let mut effective: gid_t = unsafe { uninitialized() };
		let mut saved_set: gid_t = unsafe { uninitialized() };
		let result = unsafe { getresgid(&mut real, &mut effective, &mut saved_set) };
		if likely!(result == 0)
		{
			(Self(real), Self(effective), Self(saved_set))
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EFAULT => panic!("Invalid address for real, effective or saved_set argument pointer"),
				unexpected @ _ => panic!("Unexpected error `{}` from `getresgid()`", unexpected),
			}
		}
		else
		{
			panic!("Unexpectec result `{}` from `getresgid()`", result)
		}
	}
}
