// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Over-commit setting.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum OverCommitPolicy
{
	/// The kernel attempts to estimate the amount of free memory left when userspace requests more memory.
	///
	/// This is the default.
	///
	/// Also known as 'guess'.
	Guess,

	/// Will eventually hard-fail.
	PretendThereIsAlwaysEnoughMemory,

	/// Attempts to prevent any overcommit of memory.
	///
	/// Will cause hard fails on start-up of applications that do big allocations.
	/// However, it will also cause applications that do 'just-in-case' large allocations to fail.
	///
	/// Also known as 'never'.
	Never(NeverOverCommitMemoryPolicy),
}

impl Default for OverCommitPolicy
{
	#[inline(always)]
	fn default() -> Self
	{
		OverCommitPolicy::Guess
	}
}

impl OverCommitPolicy
{
	/// Writes to `/proc/sys/vm/overcommit_memory` and one of either `/proc/sys/vm/overcommit_ratio` or `/proc/sys/vm/overcommit_kbytes`.
	#[inline(always)]
	pub fn set(self, proc_path: &ProcPath) -> io::Result<()>
	{
		use self::OverCommitPolicy::*;
		use self::NeverOverCommitMemoryPolicy::*;
		
		assert_effective_user_id_is_root("Write to `/proc/sys/vm/overcommit_memory` and one of either `/proc/sys/vm/overcommit_ratio` or `/proc/sys/vm/overcommit_kbytes`");
		
		match self
		{
			Guess => Self::write_over_commit_policy_and_default_percentage(proc_path, 0),
			
			PretendThereIsAlwaysEnoughMemory => Self::write_over_commit_policy_and_default_percentage(proc_path, 1),
			
			Never(PercentageOfPhysicalRamKilobytes(percentage)) => Self::write_over_commit_policy_and_percentage(proc_path, 2, percentage),
			
			Never(NumberOfPhysicalRamKilobytes(kilobytes)) => Self::write_over_commit_policy_and_kilobytes(proc_path, 2, kilobytes)
		}
	}
	
	#[inline(always)]
	fn write_over_commit_policy_and_default_percentage(proc_path: &ProcPath, over_commit_policy: u8) -> io::Result<()>
	{
		Self::write_over_commit_policy_and_percentage(proc_path, over_commit_policy, NeverOverCommitMemoryPolicy::DefaultPercentageOfPhysicalRamBytes)
	}
	
	#[inline(always)]
	fn write_over_commit_policy_and_percentage(proc_path: &ProcPath, over_commit_policy: u8, percentage: Percentage) -> io::Result<()>
	{
		Self::write_over_commit_policy(proc_path, over_commit_policy)?;
		Self::write_percentage(proc_path, percentage)
	}
	
	#[inline(always)]
	fn write_over_commit_policy_and_kilobytes(proc_path: &ProcPath, over_commit_policy: u8, kilobytes: Kilobytes) -> io::Result<()>
	{
		Self::write_over_commit_policy(proc_path, over_commit_policy)?;
		Self::write_kilobytes(proc_path, kilobytes)
	}
	
	#[inline(always)]
	fn write_over_commit_policy(proc_path: &ProcPath, over_commit_policy: u8) -> io::Result<()>
	{
		Self::write_value(proc_path, "overcommit_memory", UnpaddedDecimalInteger(over_commit_policy))
	}
	
	#[inline(always)]
	fn write_percentage(proc_path: &ProcPath, percentage: Percentage) -> io::Result<()>
	{
		Self::write_value(proc_path, "overcommit_ratio", percentage)
	}
	
	#[inline(always)]
	fn write_kilobytes(proc_path: &ProcPath, kilobytes: Kilobytes) -> io::Result<()>
	{
		Self::write_value(proc_path, "overcommit_kbytes", UnpaddedDecimalInteger(kilobytes))
	}
	
	#[inline(always)]
	fn write_value<'a>(proc_path: &ProcPath, file_name: &str, value: impl IntoLineFeedTerminatedByteString<'a>) -> io::Result<()>
	{
		let file_path = proc_path.sys_vm_file_path(file_name);
		if file_path.exists()
		{
			file_path.write_value(value)
		}
		else
		{
			Ok(())
		}
	}
}
