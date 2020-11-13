// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This trait supports sparse files.
///
/// Use this trait in conjunction with `CopyFileRange` to efficiently copy files from one location to another (by finding out where there are zero-byte holes, and so not copying them)..
pub trait Sparseness: AsRawFd + Seek + FileExt
{
	/// This finds all the 'non-sparse' parts of a file's inode.
	///
	/// `logical_range_in_bytes` specifies a relative, logical location in the file to search for extents.
	/// It can not be empty.
	/// Returned extents `fiemap_extent` may have a `logical_range_in_bytes()` that starts before or ends after this value.
	///
	/// If successeful, `Ok(Left(FileExtents))` is returned.
	/// If the choice of flags in `retrieve_file_extents_flags` is not supported, `Ok(Right(RetrieveFileExtentsFlags))` with supported flags is returned.
	#[inline(always)]
	fn get_file_extents(&self, logical_range_in_bytes: RangeInclusive<u64>, retrieve_file_extents_flags: RetrieveFileExtentsFlags) -> io::Result<Either<FileExtents, RetrieveFileExtentsFlags>>
	{
		match self.get_extents(logical_range_in_bytes, retrieve_file_extents_flags, 0)?
		{
			Left(file_extents) => Ok(Left(file_extents)),
			Right(incompatible_flags) => Ok(Right(RetrieveFileExtentsFlags::from_bits_truncate(incompatible_flags)))
		}
	}

	/// The extents returned will describe the file's inode's extended attribute lookup tree, instead of its data tree.
	///
	/// Most file systems do not support this.
	///
	/// The only supported by `ext4` and `f2fs`; if not, the error `EOPNOTSUPP` will be set in `Err(Error)`.
	///
	/// `logical_range_in_bytes` specifies a relative, logical location in the extended attributes to search for extents.
	/// It can not be empty.
	/// Returned extents `fiemap_extent` may have a `logical_range_in_bytes()` that starts before or ends after this value.
	///
	/// If successeful, `Ok(Left(FileExtents))` is returned.
	/// If the choice of flags in `retrieve_file_extents_flags` is not supported, `Ok(Right(RetrieveFileExtentsFlags))` with supported flags is returned.
	#[inline(always)]
	fn get_extended_attribute_extents(&self, logical_range_in_bytes: RangeInclusive<u64>, retrieve_file_extents_flags: RetrieveFileExtentsFlags) -> io::Result<Either<FileExtents, RetrieveFileExtentsFlags>>
	{
		match self.get_extents(logical_range_in_bytes, retrieve_file_extents_flags, FIEMAP_FLAG_XATTR)?
		{
			Left(file_extents) => Ok(Left(file_extents)),
			Right(incompatible_flags) => if incompatible_flags & FIEMAP_FLAG_XATTR != 0
			{
				Err(io::Error::from_raw_os_error(EOPNOTSUPP))
			}
			else
			{
				Ok(Right(RetrieveFileExtentsFlags::from_bits_truncate(incompatible_flags)))
			},
		}
	}

	#[doc(hidden)]
	#[inline(always)]
	fn get_extents(&self, logical_range_in_bytes: RangeInclusive<u64>, retrieve_file_extents_flags: RetrieveFileExtentsFlags, flags: u32) -> io::Result<Either<FileExtents, u32>>
	{
		const InitialGuessOfNumberOfExtents: NonZeroU32 = new_non_zero_u32(4);
		let mut file_extents = FileExtents::new(InitialGuessOfNumberOfExtents, logical_range_in_bytes, retrieve_file_extents_flags, flags);

		// Loop until we retrieve all file extents in one call.
		// Given that most files will only have one extent, and we allocate enough for 4 (above), this should normally not loop at all.
		loop
		{
			let result = unsafe { ioctl(self.as_raw_fd(), FS_IOC_FIEMAP, file_extents.data_pointer()) };
			if likely!(result == 0)
			{
				if file_extents.contains_all_possible_extents()
				{
					file_extents.shrink_to_fit();
					return Ok(Left(file_extents))
				}
				file_extents.increase_capacity();
				continue
			}
			else if likely!(result == -1)
			{
				return match errno().0
				{
					EBADR =>
					{
						// Could not handle flag combinations; actual flags used will be set.
						// Usually these are just input flags & !FIEMAP_FLAGS_COMPAT.
						let incompatible_flag_bits = file_extents.fm_flags();
						Ok(Right(incompatible_flag_bits))
					}

					_ => Err(io::Error::last_os_error()),
				}
			}
			else
			{
				unreachable_code(format_args!("ioctl() failed with unexpected error {}", result))
			}
		}
	}
}
