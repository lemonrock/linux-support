// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// A trait that indicates the implementor can read from a file using `sendfile()`.
pub trait SendFile
{
	/// Reads from file at current offset.
	///
	/// A successful result returning `0` means end-of-input, unless `maximum_number_of_bytes_to_transfer` was `0`.
	///
	/// Returns the number of bytes transferred.
	///
	/// Can return `Cancelled` for general I/O errors.
	fn write_output_from_file<F: AsRef<File>>(&self, from_file: &F, maximum_number_of_bytes_to_transfer: usize) -> Result<usize, StructWriteError>;

	/// Reads from file at given offset.
	///
	/// A successful result returning `0` means end-of-input, unless `maximum_number_of_bytes_to_transfer` was `0`.
	///
	/// Returns the number of bytes transferred and the updated offset.
	///
	/// Can return `Cancelled` for general I/O errors.
	fn write_output_from_file_with_offset<F: AsRef<File>>(&self, from_file: &F, offset: i64, maximum_number_of_bytes_to_transfer: usize) -> Result<(usize, i64), StructWriteError>;
}
