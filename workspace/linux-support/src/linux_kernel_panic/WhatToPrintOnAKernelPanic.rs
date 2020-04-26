// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// What to print on a kernel panic.
	#[derive(Deserialize, Serialize)]
	pub struct WhatToPrintOnAKernelPanic: u8
	{
		#[allow(missing_docs)]
		const PrintAllTasksInformation = 1 << 0;

		#[allow(missing_docs)]
		const PrintAllSystemMemoryInformation = 1 << 1;

		#[allow(missing_docs)]
		const PrintTimerInformation = 1 << 2;

		/// Only if built with `CONFIG_LOCKDEP`.
		const PrintLocksInformation = 1 << 3;

		#[allow(missing_docs)]
		const PrintFTraceBuffer = 1 << 4;
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for WhatToPrintOnAKernelPanic
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		self.bits.into_line_feed_terminated_byte_string()
	}
}
