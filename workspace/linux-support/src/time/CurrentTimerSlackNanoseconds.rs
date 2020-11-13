// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The `current` timer slack value, which differs to the non-configurable `default` timer slack value.
///
/// Does not apply to real-time threads.
///
/// Internally, Linux uses an `u64`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct CurrentTimerSlackNanoseconds(NonZeroUsize);

impl Default for CurrentTimerSlackNanoseconds
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::InitDefault
	}
}

impl CurrentTimerSlackNanoseconds
{
	/// The default for processes and threads forked from `init` is 50,000 nanoseconds.
	pub const InitDefault: Self = Self::new_unsafe(50_000);
	
	/// Current.
	pub fn current() -> io::Result<Self>
	{
		process_control_wrapper1
		(
			PR_GET_TIMERSLACK,
			|non_negative_result| if likely!(non_negative_result > 0)
			{
				Ok(Self::new_unsafe(non_negative_result as usize))
			}
			else
			{
				Err(io_error_invalid_data("Timer slack of 0 is invalid"))
			},
			error_number_to_io_error
		)
	}
	
	/// Reset to `default`, which may not be `Self::InitDefault`.
	pub fn reset_to_default() -> Result<(), Errno>
	{
		Self::set_internal(0)
	}
	
	/// Set.
	pub fn set(self) -> Result<(), Errno>
	{
		Self::set_internal(self.0.get())
	}
	
	#[inline(always)]
	fn set_internal(value: usize) -> Result<(), Errno>
	{
		process_control_wrapper2(PR_SET_TIMERSLACK,&value as *const usize as usize,result_must_be_zero,Err)
	}
	
	#[inline(always)]
	const fn new_unsafe(nanoseconds: usize) -> Self
	{
		Self(new_non_zero_usize(nanoseconds))
	}
}
