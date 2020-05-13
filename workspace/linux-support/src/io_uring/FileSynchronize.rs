// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// File synchronization choice.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum FileSynchronize
{
	/// Data and metadata.
	All = 0,

	/// Just data.
	JustData = IORING_FSYNC_DATASYNC,
}

impl Default for FileSynchronize
{
	#[inline(always)]
	fn default() -> Self
	{
		FileSynchronize::All
	}
}
