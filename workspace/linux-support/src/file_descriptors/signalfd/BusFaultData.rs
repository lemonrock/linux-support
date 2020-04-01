// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Contains fault data relevant to the `SIGBUS` signal.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BusFaultData
{
	/// Fault data.
	pub fault_data: FaultData,

	/// The least significant bit of the reported address; indicates the extent of the corruption.
	///
	/// For example, if a full page was corrupted, this field would be `log2(sysconf(_SC_PAGESIZE))`.
	///
	/// Only populated when `code` is `BusCode::HardwareErrorMachineCheckActionRequired` or `BusCode::HardwareErrorMachineCheckActionOptional`.
	///
	/// Only populated since Linux 2.6.37, although the codes were added in Linux 2.6.32.
	///
	/// When not populated the value is zero.
	pub address_least_significant_bit: u16
}

impl BusFaultData
{
	#[inline(always)]
	pub(crate) fn new(ssi: &signalfd_siginfo) -> Self
	{
		Self
		{
			fault_data: FaultData::new(ssi),
			address_least_significant_bit: ssi.ssi_addr_lsb,
		}
	}
}
