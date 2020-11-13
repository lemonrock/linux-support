// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// Strictly-speaking, unsized.
#[repr(C)]
pub(super) struct io_uring_probe
{
	/// last opcode supported (inclusive).
	///
	/// Union of `u8` and `IORING_OP`.
	pub(super) last_op: u8,

	/// length of `ops` array below.
	pub(super) ops_len: u8,

	resv: u16,

	resv2: [u32; 3],

	ops: [io_uring_probe_op; IORING_OP::IORING_OP_LAST],
}

impl io_uring_probe
{
	#[inline(always)]
	pub(super) fn new() -> Self
	{
		Self
		{
			last_op: unsafe_uninitialized(),
			ops_len: IORING_OP::IORING_OP_LAST as u8,
			resv: unsafe_zeroed(),
			resv2: unsafe_zeroed(),
			ops: unsafe_uninitialized(),
		}
	}

	#[inline(always)]
	pub(super) fn ops(&self) -> &[io_uring_probe_op]
	{
		&self.ops[0 .. self.ops_len as usize]
	}
}
