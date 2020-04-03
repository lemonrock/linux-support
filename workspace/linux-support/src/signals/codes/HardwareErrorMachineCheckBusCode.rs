// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a code that can be associated with a kernel-raised `SIGBUS` signal.
///
/// Definitions valid as of Linux v4.20-rc5.
///
/// Note that the definitions for `BUS_MCEERR_AR` and `BUS_MCEERR_AO` are deliberately *NOT* present in this enum, as they require special handling.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum HardwareErrorMachineCheckBusCode
{
	/// Hardware memory error consumed on a machine check; action required.
	///
	/// Known as `BUS_MCEERR_AR` in Linux sources.
	///
	/// Since Linux 2.6.32.
	///
	/// Since Linux 2.6.37, the presence of this code indicates that the `address_least_significant_bit` argument is populated in `SignalHandler::kernel_raised_sigbus()`.
	ActionRequired = 4,

	/// Hardware memory error detected in process but not consumed; action optional.
	///
	/// Known as `BUS_MCEERR_AO` in Linux sources.
	///
	/// Since Linux 2.6.32.
	///
	/// Since Linux 2.6.37, the presence of this code indicates that the `address_least_significant_bit` argument is populated in `SignalHandler::kernel_raised_sigbus()`.
	ActionOptional = 5,
}

impl Into<i32> for HardwareErrorMachineCheckBusCode
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self as i32
	}
}

impl Code for HardwareErrorMachineCheckBusCode
{
	/// Known as `NSIGBUS` in Linux sources.
	const InclusiveMaximum: Self = HardwareErrorMachineCheckBusCode::ActionOptional;

	#[inline(always)]
	fn rehydrate(validated_si_code: i32) -> Self
	{
		unsafe { transmute(validated_si_code)}
	}
}
