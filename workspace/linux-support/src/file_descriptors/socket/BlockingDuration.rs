// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Defaults to `BlocksForever`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockingDuration
{
	/// Blocking.
	BlocksForever,
	
	/// Blocking but times out.
	BlocksButTimesOut
	{
		/// Time out after this number of microseconds.
		microseconds: NonZeroU64,
	},
}

impl Default for BlockingDuration
{
	#[inline(always)]
	fn default() -> Self
	{
		BlockingDuration::BlocksForever
	}
}

impl BlockingDuration
{
	#[inline(always)]
	pub(crate) fn set_time_out_assuming_never_set_before(&self, socket_file_descriptor: &SocketFileDescriptor<impl SocketData>, option_name: i32)
	{
		if let BlockingDuration::BlocksButTimesOut { microseconds } = self
		{
			const MicrosecondsInASecond: i64 = 1_000_000;
			let microseconds: i64 = (*microseconds).get().try_into().expect("Too large");
			let value = timeval
			{
				tv_sec: microseconds / MicrosecondsInASecond,
				tv_usec: microseconds % MicrosecondsInASecond,
			};
			socket_file_descriptor.set_socket_option(SOL_SOCKET, option_name, &value)
		}
	}
}
