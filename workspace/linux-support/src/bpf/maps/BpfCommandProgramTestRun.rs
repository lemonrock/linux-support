// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used by `BPF_PROG_TEST_RUN` command.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct BpfCommandProgramTestRun
{
	pub(crate) prog_fd: u32,
	
	pub(crate) retval: u32,
	
	/// Size of data pointed to by `data_in`.
	pub(crate) data_size_in: u32,
	
	/// Size of data pointed to by `data_out`.
	pub(crate) data_size_out: u32,
	
	/// Pointer to data in.
	pub(crate) data_in: AlignedU64,
	
	/// Pointer to data out.
	pub(crate) data_out: AlignedU64,
	
	pub(crate) repeat: u32,
	
	pub(crate) duration: u32,
	
	/// Size of data pointed to by `ctx_in`.
	pub(crate) ctx_size_in: u32,
	
	/// Size of data pointed to by `ctx_out`.
	pub(crate) ctx_size_out: u32,
	
	/// Pointer to context in data.
	pub(crate) ctx_in: AlignedU64,
	
	/// Pointer to context out data.
	pub(crate) ctx_out: AlignedU64,
}
