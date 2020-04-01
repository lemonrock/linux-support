// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur during read of a timer instance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructWriteError
{
	/// It is not possible to write events at this time.
	///
	/// This might be because the event counter is currently at its maximum.
	WouldBlock,

	/// Event was cancelled; it is not obvious whether this can actually ever occur.
	Cancelled,

	/// `EINTR` occurred; this can be handled by either re-trying the write-like operation or might actual be fatal depending on the signal handling strategy in use.
	Interrupted,
}

impl Display for StructWriteError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<StructWriteError as Debug>::fmt(self, f)
	}
}

impl error::Error for StructWriteError
{
}
