// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
pub(super) struct nlmsgerr
{
	/// Standard `E*` error code, eg `EINVAL`.
	///
	/// Is zero if this structure represents an acknowledgment.
	pub(super) error: c_int,
	
	/// Original (request) netlink message header.
	pub(super) msg: nlmsghdr,
	
	// Variable number of attributes of type `nlmsgerr_attrs` forms the payload of the `msg` field above *if* `error` is zero.
}

impl nlmsgerr
{
	#[inline(always)]
	pub fn error_or_acknowledgment_io_result(&self) -> Result<(), SystemCallErrorNumber>
	{
		match self.error
		{
			0 => Ok(()),
			
			error @ SystemCallErrorNumber::InclusiveMinimumI32 ..= SystemCallErrorNumber::InclusiveMaximumI32 => Err(unsafe { SystemCallErrorNumber::from_unchecked(error) }),
			
			unexpected => unreachable_code(format_args!("error field is either negative or greater than 4095: {}", unexpected))
		}
	}
	
	#[inline(always)]
	pub fn over_run_io_result(&self) -> SystemCallErrorNumber
	{
		match self.error
		{
			error @ SystemCallErrorNumber::InclusiveMinimumI32 ..= SystemCallErrorNumber::InclusiveMaximumI32 => unsafe { SystemCallErrorNumber::from_unchecked(error) },
			
			unexpected => unreachable_code(format_args!("error field is either zero, negative or greater than 4095: {}", unexpected))
		}
	}
}
