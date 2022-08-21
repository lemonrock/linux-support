// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a combined soft and hard resource limit value.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoftAndHardResourceLimit
{
	soft: ResourceLimit,
	hard: ResourceLimit,
}

impl SoftAndHardResourceLimit
{
	/// Both the soft and hard limits are set to be infinite.
	pub const BothInfinite: SoftAndHardResourceLimit = SoftAndHardResourceLimit
	{
		soft: ResourceLimit::Infinite,
		hard: ResourceLimit::Infinite,
	};

	/// Both the soft and hard limits are set to be zero (0).
	pub const BothZero: SoftAndHardResourceLimit = SoftAndHardResourceLimit
	{
		soft: ResourceLimit::Finite(0),
		hard: ResourceLimit::Finite(0),
	};

	/// Set both the soft and the hard resource limits to `soft_and_hard`.
	pub fn both(soft_and_hard: ResourceLimit) -> Self
	{
		Self::new(soft_and_hard.clone(), soft_and_hard)
	}

	/// Create a new instance.
	pub fn new(soft: ResourceLimit, hard: ResourceLimit) -> Self
	{
		if soft.is_infinite() && hard.is_finite()
		{
			panic!("soft limit can not be infinite if hard limit '{}' is finite", hard.unwrap());
		}

		if soft.is_finite() && hard.is_finite()
		{
			assert!(soft.unwrap() <= hard.unwrap(), "soft limit '{:?}' must be less than or the same as hard limit '{:?}'", soft, hard);
		}

		Self
		{
			soft,
			hard,
		}
	}

	/// Obtain the soft limit.
	#[inline(always)]
	pub fn soft_limit(&self) -> &ResourceLimit
	{
		&self.soft
	}

	/// Obtain the hard limit.
	#[inline(always)]
	pub fn hard_limit(&self) -> &ResourceLimit
	{
		&self.hard
	}

	fn set(&self, resource_identifier: i32) -> Result<(), ResourceLimitError>
	{
		let value = rlimit
		{
			rlim_cur: self.soft.unwrap(),
			rlim_max: self.hard.unwrap(),
		};

		use self::ResourceLimitError::*;

		match unsafe { setrlimit(resource_identifier, &value) }
		{
			0 => Ok(()),

			-1 => match SystemCallErrorNumber::from_errno_panic()
			{
				EPERM => Err(PermissionDeniedOrTriedToIncreaseAboveMaximumNumberOfFileDescriptors),
				EINVAL => Err(LimitWasTooLarge),

				EFAULT => panic!("Bad pointer"),
				
				unexpected_error @ _ => unexpected_error!(setrlimit, unexpected_error),
			},
			
			unexpected @ _ => unexpected_result!(setrlimit, result),
		}
	}

	fn get(resource_identifier: i32) -> Self
	{
		let mut value = rlimit
		{
			rlim_cur: 0,
			rlim_max: 0,
		};

		match unsafe { getrlimit(resource_identifier, &mut value) }
		{
			0 => (),

			-1 => match SystemCallErrorNumber::from_errno_panic()
			{
				EPERM => panic!("Permission denied"),

				EINVAL => panic!("Bad resource id"),
				EFAULT => panic!("Bad pointer"),

				unexpected_error @ _ => unexpected_error!(getrlimit, unexpected_error),
			},

			unexpected @ _ => unexpected_result!(getrlimit, result)
		};

		Self
		{
			soft: ResourceLimit::convert(value.rlim_cur),
			hard: ResourceLimit::convert(value.rlim_max),
		}
	}
}
