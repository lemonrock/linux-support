// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents the contents of the file `/proc/<N>/stat`.
pub struct ProcStat
{

}

impl ProcStat
{
	/// Status information from `/proc/self/status`.
	///
	/// Assumes at least Linux 3.5 is in use.
	#[inline(always)]
	pub fn self_stat(proc_path: &ProcPath) -> Result<Self, ProcessStatusFileParseError>
	{
		Self::process_stat(proc_path, 0)
	}

	/// Status information from `/proc/<IDENTIFIER>/status` where `<IDENTIFIER>` is `identifier`.
	///
	/// Assumes at least Linux 3.5 is in use.
	#[inline(always)]
	pub fn process_stat(proc_path: &ProcPath, identifier: pid_t) -> Result<Self, ProcessStatusFileParseError>
	{
		let line = proc_path.process_file_path(identifier, "stat").read_raw_without_line_feed()?;

		let mut byte_index = 0;

		/// Read using `sscanf`'s `%d`.
		fn read_d(line: &[u8], byte_index: usize) -> (d, usize)
		{
			// Matches an optionally signed decimal integer; the next pointer must be a pointer to int.
		}
	}
}
