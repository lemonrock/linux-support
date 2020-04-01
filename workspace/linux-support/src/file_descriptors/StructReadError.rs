// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur during read.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructReadError
{
	/// There is no data to read at this time.
	WouldBlock,

	/// For timers: the timer was cancelled because it depends on the realtime clock and the realtime clock was adjusted.
	///
	/// Not obvious if this can occur for other file descriptors.
	Cancelled,

	/// `EINTR` occurred; this can be handled by either re-trying the read operation or might actual be fatal depending on the signal handling strategy in use.
	Interrupted,
}

impl Display for StructReadError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<StructReadError as Debug>::fmt(self, f)
	}
}

impl error::Error for StructReadError
{
}
