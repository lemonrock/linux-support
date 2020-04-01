// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// See documentation for `setsockopt()` for common documentation.
	///
	/// For `getsockopt()`, `optlen` is a value-result argument, initially containing the size of the buffer pointed to by `optval`, and modified on return to indicate the actual size of the value returned.
	///
	/// If no option value is to be supplied or returned, `optva`l may be `NULL`.
	pub(crate) fn getsockopt(sockfd: RawFd, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int;
}
