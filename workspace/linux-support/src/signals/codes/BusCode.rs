// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a code that can be associated with a kernel-raised `SIGBUS` signal.
///
/// Definitions valid as of Linux v4.20-rc5.
///
/// Note that the definitions for `BUS_MCEERR_AR` and `BUS_MCEERR_AO` are deliberately *NOT* present in this enum, as they require special handling.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum BusCode
{
	/// Invalid address alignment.
	///
	/// Known as `BUS_ADRALN` in Linux sources.
	InvalidAddressAlignment = 1,

	/// Nonexistent physical address.
	///
	/// Known as `BUS_ADRERR` in Linux sources.
	NonexistentPhysicalAddress = 2,

	/// Object-specific hardware error.
	///
	/// Known as `BUS_OBJERR` in Linux sources.
	ObjectSpecificHardwareError = 3,
}

impl Into<i32> for BusCode
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self as i32
	}
}

impl Code for BusCode
{
	/// No Linux equivalent.
	const InclusiveMaximum: Self = BusCode::ObjectSpecificHardwareError;

	#[inline(always)]
	fn rehydrate(validated_si_code: i32) -> Self
	{
		unsafe { transmute(validated_si_code)}
	}
}
