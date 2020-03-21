/// A cgroups version 2 controller.
// ***TODO: If changing the number of variants, remember to change `MaximumNumberOfControllers` below***.
// ***TODO: If changing the number of variants, remember to change `from_str` below***.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum Controller
{
	/// Equivalent to cgroups version 1 controller `blkio`.
	io,

	/// Equivalent to cgroups version 1 controller `memory`.
	memory,

	/// Equivalent to cgroups version 1 controller `pids`.
	pids,

	/// Equivalent to cgroups version 1 controller `perf_event`.
	///
	/// Since Linux version 4.11.
	perf_event,

	/// Equivalent to cgroups version 1 controller `rdma`.
	///
	/// Since Linux version 4.11.
	rdma,

	/// Successor to cgroups version 1 controllers `cpu` and `cpuacct`.
	///
	/// Since Linux version 4.15.
	cpu,
}

impl FromStr for Controller
{
	type Err = ();

	// ***TODO: Adjust this match when changing the number of variants above***.
	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		use self::Controller::*;
		let variant = match s
		{
			"io" => io,
			"memory" => memory,
			"pids" => pids,
			"perf_event" => perf_event,
			"rdma" => rdma,
			"cpu" => cpu,

			_ => return Err(())
		};
		Ok(variant)
	}
}

impl Controller
{
	// ***TODO: Adjust this value when changing the number of variants above***.
	const MaximumNumberOfControllers: usize = 6;

	#[inline(always)]
	fn append_to(self, line: &mut String, sign: char)
	{
		line.push(sign);
		line.push_str(self.to_str())
	}

	#[inline(always)]
	fn to_str(self) -> &'static str
	{
		use self::Controller::*;
		match self
		{
			io => "io",
			memory => "memory",
			pids => "pids",
			perf_event => "perf_event",
			rdma => "rdma",
			cpu => "cpu",
		}
	}
}
