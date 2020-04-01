// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[derive(Default, Debug, Clone)]
#[repr(C)]
pub(crate) struct mmsghdr
{
	/// PosixMessage header.
	pub(crate) msg_hdr: msghdr,

	/// Number of received bytes for header.
	///
	/// This field has the same value as the return value of a single `recvmsg()` call.
	pub(crate) msg_len: c_uint,
}
