// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DogStatsD service check status.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ServiceCheckStatus
{
	OK = b'0',
	
	WARN = b'1',
	
	CRITICAL = b'2',
	
	UNKNOWN = b'3',
}

impl ServiceCheckStatus
{
	#[inline(always)]
	fn dog_stats_d_write(&self, dog_stats_d_writer: &mut DogStatsDWriter) -> Result<(), ()>
	{
		dog_stats_d_writer.write_pipe()?;
		dog_stats_d_writer.write_byte((*self) as u8)
	}
}
