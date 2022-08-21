// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// File deduplication range information.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct file_dedupe_range_info
{
	pub(super) dest_fd: i64,
	
	pub(super) dest_offset: u64,
	
	pub(super) bytes_deduped: u64,
	
	pub(super) status: i32,
	
	pub(super) reserved: u32,
}

impl file_dedupe_range_info
{
	/// Outcome.
	#[inline(always)]
	pub fn outcome(&self) -> io::Result<DeduplicationOutcome>
	{
		use self::DeduplicationOutcome::*;
		
		match self.status
		{
			FILE_DEDUPE_RANGE_SAME => Ok(Deduplicated { number_of_bytes_deduplicated: self.bytes_deduped }),
			
			FILE_DEDUPE_RANGE_DIFFERS => Ok(RangeDiffers),
			
			error @ SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => Err(SystemCallErrorNumber::from_negative_i32_unchecked(error).into()),
			
			unexpected @ _ => unexpected_result!(status, unexpected),
		}
	}
}
