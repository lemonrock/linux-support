// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// When to change the terminal attributes.
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum WhenToChangeTerminalAttributes
{
	/// Change immediately.
	Now = TCSANOW,

	/// Change after waiting until all queued output has been written.
	///
	/// You should usually use this option when changing parameters that affect output.
	Drain = TCSADRAIN,

	/// Change after waiting until all queued output has been written and after discarding all queued input.
	Flush = TCSAFLUSH,
}

impl WhenToChangeTerminalAttributes
{
	#[inline(always)]
	pub(crate) fn flags(self, ignore_control_flags: bool) -> c_int
	{
		let flags = self as c_int;
		if unlikely!(ignore_control_flags)
		{
			flags | TCSASOFT
		}
		else
		{
			flags
		}
	}
}
