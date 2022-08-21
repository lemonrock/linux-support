// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Extended seek that allows the use of holes.
///
/// The hole operations `Data` and `Hole` allow applications to map holes in a sparsely allocated file.
/// This can be useful for applications such as file backup tools, which can save space when creating backups and preserve holes, if they have a mechanism for discovering holes.
///
/// For the purposes of these operations, a hole is a sequence of zeros that (normally) has not been allocated in the underlying file storage.
/// However, a filesystem is not obliged to report holes, so these operations are not a guaranteed mechanism for mapping the storage space actually allocated to a file.
/// (Furthermore, a sequence of zeros that actually has been written to the underlying storage may not be reported as a hole).
/// In the simplest implementation, a filesystem can support the operations by making `ExtendedSeekFrom::Hole` always return the offset of the end of the file, and making `ExtendedSeekFrom::Data` always return offset (ie, even if the location referred to by offset is a hole, it can be considered to consist of data that is a sequence of zeros).
pub trait ExtendedSeek: AsRawFd + Seek + FileExt
{
	/// Seek to an offset, in bytes, in a stream.
	///
	/// A seek beyond the end of a stream is allowed, but behavior is defined by the implementation.
	///
	/// If the seek operation completed successfully, this method returns the new position from the start of the stream.
	/// That position can be used later with `ExtendedSeekFrom::Start`.
	#[inline(always)]
	fn extended_seek(&mut self, pos: ExtendedSeekFrom) -> io::Result<u64>
	{
		let (whence, start) = pos.to_whence_and_start();
		let result = unsafe { lseek(self.as_raw_fd(), start, whence as i32) };
		if likely!(result >= 0)
		{
			Ok(result as u64)
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unexpected_result!(lseek, result)
		}
	}
}
