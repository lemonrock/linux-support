// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub(super) struct uffdio_copy
{
	/// Destination of copy.
	pub(super) dst: u64,
	
	/// Source of copy.
	pub(super) src: u64,
	
	/// Number of bytes to copy.
	pub(super) len: u64,
	
	/// Flags controlling behavior of copy.
	pub(super) mode: CopyMode,
	
	/// Number of bytes copied, or negated error (which is the error returned by the internal Linux function `mcopy_atomic()`.
	///
	/// `copy` is written by the ioctl.
	pub(super) copy: i64,
}
