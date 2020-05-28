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
	pub fn io_result(&self) -> io::Result<()>
	{
		if likely!(self.error > 0 && self.error < 4096)
		{
			Err(io::Error::from_raw_os_error(self.error))
		}
		else if likely!(self.error == 0)
		{
			Ok(())
		}
		else
		{
			unreachable!("error field is either negative or greater than 4095: {}", self.error)
		}
	}
}
