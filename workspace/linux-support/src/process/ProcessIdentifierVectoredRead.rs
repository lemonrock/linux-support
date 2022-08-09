// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A wrapper for vectored reads.
///
/// DOES NOT ADJUST `from_remote` for bytes read.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessIdentifierVectoredRead<'a>
{
	/// Process identifier.
	pub process_identifier: ProcessIdentifier,

	/// From remote.
	pub from_remote: &'a [&'a [u8]],
}

impl<'a> VectoredRead for ProcessIdentifierVectoredRead<'a>
{
	#[inline(always)]
	fn read_vectored(&self, to_local: &[&mut [u8]]) -> io::Result<usize>
	{
		self.process_identifier.vectored_read(to_local, self.from_remote).map_err(|creation_error| io::Error::from_raw_os_error(creation_error as i32))
	}
}

impl<'a> Read for ProcessIdentifierVectoredRead<'a>
{
	#[inline(always)]
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
	{
		VectoredRead::read_vectored(self, &[buf])
	}

	#[inline(always)]
	fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>
	{
		VectoredRead::read_vectored(self, unsafe { transmute(bufs) })
	}
}
