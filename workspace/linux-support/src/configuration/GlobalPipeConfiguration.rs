// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global pipe configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalPipeConfiguration
{
	/// Maximum pipe (FIFO) capacity in bytes.
	///
	/// Requires root.
	pub maximum_pipe_capacity: Option<NonZeroU32>,

	/// Pipe soft limit in pages per unprivileged user.
	///
	/// Requires root.
	///
	/// `Some(None)` means to configure no limit whatsoever.
	pub pipe_soft_limit: Option<Option<NonZeroNumberOfPages>>,

	/// Pipe hard limit in pages per unprivileged user.
	///
	/// Requires root.
	///
	/// `Some(None)` means to configure no limit whatsoever.
	pub pipe_hard_limit: Option<Option<NonZeroNumberOfPages>>,
}

impl GlobalPipeConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalPipeConfigurationError>
	{
		use self::GlobalPipeConfigurationError::*;

		set_value(proc_path, set_maximum_pipe_capacity, self.maximum_pipe_capacity, CouldNotChangeMaximumPipeCapacity)?;
		set_value(proc_path, set_pipe_user_pages_soft_limit, self.pipe_soft_limit, CouldNotChangePipeSoftLimit)?;
		set_value(proc_path, set_pipe_user_pages_hard_limit, self.pipe_hard_limit, CouldNotChangePipeHardLimit)
	}
}
