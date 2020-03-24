// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


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
	fn append_to(self, line: &mut Vec<u8>, sign: u8)
	{
		line.push(sign);
		line.extend_from_slice(self.to_bytes())
	}

	#[inline(always)]
	fn to_bytes(self) -> &'static [u8]
	{
		use self::Controller::*;
		match self
		{
			io => b"io" as &[u8],
			memory => b"memory" as &[u8],
			pids => b"pids" as &[u8],
			perf_event => b"perf_event" as &[u8],
			rdma => b"rdma" as &[u8],
			cpu => b"cpu" as &[u8],
		}
	}
}
