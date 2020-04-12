// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Implementors support vectored reads.
pub trait VectoredRead
{
	/// Performs a vectored read.
	///
	/// All buffers should be non-zero sized otherwise the results are ambiguous (ie was nothing read or is this end-of-file).
	///
	/// For processes, see `ProcessIdentifier::to_vectored_read()`.
	fn read_vectored(&self, buffers: &[&mut [u8]]) -> io::Result<usize>;
}
