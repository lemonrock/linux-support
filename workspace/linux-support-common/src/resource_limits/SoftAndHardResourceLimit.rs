/// Represents a combined soft and hard resource limit value.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
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

	fn set(&self, resource_identifier: i32)
	{
		let value = rlimit
		{
			rlim_cur: self.soft.unwrap(),
			rlim_max: self.hard.unwrap(),
		};

		match unsafe { setrlimit(resource_identifier, &value) }
		{
			0 => (),

			-1 => match errno().0
			{
				EPERM => panic!("Permission denied or tried to increase MaximumNumberOfFileDescriptors above /proc/sys/fs/nr_open"),

				EINVAL => panic!("Limit was too large or bad resource id"),
				EFAULT => panic!("Bad pointer"),

				illegal @ _ => panic!("Illegal error number '{}' from setrlimit64()", illegal),
			},

			illegal @ _ => panic!("Illegal result '{}' from setrlimit64()", illegal),
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

			-1 => match errno().0
			{
				EPERM => panic!("Permission denied"),

				EINVAL => panic!("Bad resource id"),
				EFAULT => panic!("Bad pointer"),

				illegal @ _ => panic!("Illegal error number '{}' from setrlimit64()", illegal),
			},

			illegal @ _ => panic!("Illegal result '{}' from setrlimit64()", illegal),
		};

		Self
		{
			soft: ResourceLimit::convert(value.rlim_cur),
			hard: ResourceLimit::convert(value.rlim_max),
		}
	}
}
