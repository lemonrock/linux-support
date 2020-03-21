/// Represents a finite quantity or infinite (ie no) limit for a resource.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
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

	/// Obtains the maximum number of file descriptors as a finite resource limit.
	pub fn maximum_number_of_open_file_descriptors(proc_path: &ProcPath) -> Result<ResourceLimit, io::Error>
	{
		Ok(ResourceLimit::Finite(proc_path.maximum_number_of_open_file_descriptors()?))
	}

	/// Value.
	#[inline(always)]
	pub fn value(&self) -> u64
	{
		use self::ResourceLimit::*;

		match *self
		{
			Finite(limit) => limit,
			Infinite => ::std::u64::MAX,
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
