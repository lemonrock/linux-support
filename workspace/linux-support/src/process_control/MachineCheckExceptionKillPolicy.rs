// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Machine Check Exception (MCE) kill policy.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(i32)]
pub enum MachineCheckExceptionKillPolicy
{
	/// Early.
	Early = PR_MCE_KILL_EARLY,
	
	/// Late.
	Late = PR_MCE_KILL_LATE,

	/// Default.
	Default = PR_MCE_KILL_DEFAULT,
}

impl Default for MachineCheckExceptionKillPolicy
{
	#[inline(always)]
	fn default() -> Self
	{
		MachineCheckExceptionKillPolicy::Default
	}
}

impl MachineCheckExceptionKillPolicy
{
	/// For current process.
	pub fn for_current_process() -> io::Result<Self>
	{
		use self::MachineCheckExceptionKillPolicy::*;
		
		process_control_wrapper1
		(
			PR_MCE_KILL_GET,
			|non_negative_result| match non_negative_result
			{
				PR_MCE_KILL_EARLY => Ok(Early),
				
				PR_MCE_KILL_LATE => Ok(Late),
				
				PR_MCE_KILL_DEFAULT => Ok(Default),
				
				_ => Err(io::Error::new(ErrorKind::InvalidData, "Unknown value for `prctl(PR_MCE_KILL_GET)`")),
			},
			|error_number| Err(error_number.into()),
		)
	}
	
	/// Clear.
	pub fn clear_for_current_thread() -> io::Result<()>
	{
		process_control_wrapper2
		(
			PR_MCE_KILL,
			PR_MCE_KILL_CLEAR as usize,
			result_must_be_zero,
			|error_number| Err(error_number.into())
		)
	}
	
	/// Set.
	pub fn set_for_current_thread(self) -> io::Result<()>
	{
		process_control_wrapper3
		(
			PR_MCE_KILL,
			PR_MCE_KILL_SET as usize,
			self as i32 as usize,
			result_must_be_zero,
			|error_number| Err(error_number.into())
		)
	}
}
