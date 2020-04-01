// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Contains fault data relevant to certain signals.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FaultData
{
	/// The address of the fault.
	pub address: u64,

	/// The trap number of the fault (only supported on the Alpha, MIPS and SPARC architectures; Rust does not support the Alpha architecture).
	///
	/// Where not supported the value is zero.
	pub trap_number: u32,
}

impl FaultData
{
	#[inline(always)]
	pub(crate) fn new(ssi: &signalfd_siginfo) -> Self
	{
		Self
		{
			address: ssi.ssi_addr,
			trap_number: ssi.ssi_trapno,
		}
	}
}
