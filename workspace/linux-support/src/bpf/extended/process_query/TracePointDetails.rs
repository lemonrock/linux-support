// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Trace point details.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TracePointDetails
{
	/// Name of trace point.
	///
	/// Total length, including trailing ASCII NUL, is limited to 128 bytes.
	trace_point_name: CString,
}

impl Deref for TracePointDetails
{
	type Target = CStr;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.trace_point_name
	}
}

impl TracePointDetails
{
	/// Maximum length of `trace_point_name` (including trailing nul).
	pub const TracePointNameMaximumLengthIncludingTrailingNul: usize = 128;
	
	/// Maximum length `trace_point_name` (excluding trailing nul).
	pub const TracePointNameMaximumLengthExcludingTrailingNul: usize = Self::TracePointNameMaximumLengthIncludingTrailingNul - 1;
	
	/// Constructs a new instance, validating that the `trace_point_name`'s length including the trailing nul does not exceed `TracePointNameMaximumLengthIncludingTrailingNul`.
	#[inline(always)]
	pub fn new(trace_point_name: CString) -> Result<Self, ()>
	{
		if trace_point_name.as_bytes().len() > Self::TracePointNameMaximumLengthExcludingTrailingNul
		{
			Err(())
		}
		else
		{
			Ok
			(
				Self
				{
					trace_point_name
				}
			)
		}
	}
	
	/// Trace point name.
	#[inline(always)]
	pub fn trace_point_name(self) -> CString
	{
		self.trace_point_name
	}
}
