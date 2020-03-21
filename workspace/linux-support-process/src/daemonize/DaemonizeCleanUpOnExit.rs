// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This object encapsulates a piece of behaviour to run on exit to ensure clean-up.
///
/// Currently it justs ensures that PID files are deleted.
#[derive(Debug)]
pub struct DaemonizeCleanUpOnExit
{
	pid_file_path: PathBuf
}

impl DaemonizeCleanUpOnExit
{
	/// Cleans up.
	#[inline(always)]
	pub fn clean_up(self)
	{
		if let Err(_) = remove_file(&self.pid_file_path)
		{
			eprintln!("Could not remove PID file '{:?}'", self.pid_file_path)
		}
	}
}
