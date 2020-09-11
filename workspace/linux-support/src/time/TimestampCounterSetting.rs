// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Timestamp counter (TSC) setting.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[repr(i32)]
pub enum TimestampCounterSetting
{
	/// Permit use of the `TSC`.
	Permit = PR_TSC_ENABLE,

	/// Sends a `SIGSEGV` signal.
	DenyAndSendASegmentationFaultSignal = PR_TSC_SIGSEGV,
}

impl TimestampCounterSetting
{
	/// Set.
	#[inline(always)]
	pub fn set(self) -> Result<(), Errno>
	{
		let value: i32 = self as i32;
		
		process_control_wrapper2(PR_SET_TSC,&value as *const i32 as usize,result_must_be_zero,Err)
	}
	
	/// Get.
	#[allow(deprecated)]
	pub fn current() -> io::Result<Self>
	{
		let mut value: i32 = unsafe { uninitialized() };
		
		process_control_wrapper2
		(
			PR_GET_TSC,
			&mut value as *mut i32 as usize,
			|non_negative_result| if likely!(non_negative_result == 0)
			{
				use self::TimestampCounterSetting::*;
				match value
				{
					PR_TSC_ENABLE => Ok(Permit),
					PR_TSC_SIGSEGV => Ok(DenyAndSendASegmentationFaultSignal),
					_ => Err(io_error_invalid_data("Unknown value"))
				}
			}
			else
			{
				unreachable!("Positive result")
			},
			error_number_to_io_error,
		)
	}
}
