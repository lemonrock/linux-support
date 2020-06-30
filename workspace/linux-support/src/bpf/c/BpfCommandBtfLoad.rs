// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for the command `BPF_BTF_LOAD`.
///
/// BTF is BPF Type Format.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct BpfCommandBtfLoad
{
	/// Pointer to data.
	pub(crate) btf: AlignedU64,
	
	/// Pointer to data.
	pub(crate) btf_log_buf: AlignedU64,
	
	/// Size of data pointed to by `btf`.
	pub(crate) btf_size: u32,
	
	/// Size of data pointed to by `btf_log_buf`.
	pub(crate) btf_log_size: u32,
	
	pub(crate) btf_log_level: u32,
}
