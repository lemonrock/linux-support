// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A wrapper for vectored writes.
///
/// DOES NOT ADJUST `to_remote` for bytes read.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessIdentifierVectoredWrite<'a>
{
	/// Process identifier.
	pub process_identifier: ProcessIdentifier,

	/// To remote.
	pub to_remote: &'a [&'a mut [u8]],
}

impl<'a> VectoredWrite for ProcessIdentifierVectoredWrite<'a>
{
	#[inline(always)]
	fn write_vectored(&self, from_local: &[&[u8]]) -> io::Result<usize>
	{
		self.process_identifier.vectored_write(from_local, self.to_remote).map_err(|creation_error| creation_error.into())
	}
}

impl<'a> Write for ProcessIdentifierVectoredWrite<'a>
{
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize>
	{
		VectoredWrite::write_vectored(self, &[buf])
	}

	#[inline(always)]
	fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize>
	{
		VectoredWrite::write_vectored(self, unsafe { transmute(bufs) })
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()>
	{
		Ok(())
	}
}
