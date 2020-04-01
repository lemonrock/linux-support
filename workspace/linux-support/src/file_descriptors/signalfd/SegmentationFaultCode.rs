// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a code that can be associated with a kernel-raised `SIGSEGV` signal.
///
/// Definitions valid as of Linux v4.20-rc5.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum SegmentationFaultCode
{
	/// Address not mapped to object.
	///
	/// Known as `SEGV_MAPERR` in Linux sources.
	AddressNotMappedToObject = 1,

	/// Invalid permissions for mapped object.
	///
	/// Known as `SEGV_MAPERR` in Linux sources.
	InvalidPermissionsForMappedObject = 2,

	/// Failed address bound checks.
	///
	/// Known as `SEGV_BNDERR` in Linux sources.
	FailedAddressBoundChecks = 3,

	/// Failed protection key checks.
	///
	/// Known as `SEGV_PKUERR` in Linux sources.
	///
	/// For Itanium (`ia64`) this is known as `__SEGV_PSTKOVF` in the Linux sources and is called a 'Paragraph Stack Overflow'.
	FailedProtectionKeyChecks = 4,

	/// ADI (MCD) not enabled for mapped object.
	///
	/// Known as `SEGV_ACCADI` in Linux sources.
	AdiNotEnabledForMappedObject = 5,

	/// ADI (MCD) mismatch disrupting exception.
	///
	/// Known as `SEGV_ACCADI` in Linux sources.
	AdiMismatchDisruptingException = 6,

	/// ADI (MCD) mismatch precise exception.
	///
	/// Known as `SEGV_ACCADI` in Linux sources.
	AdiMismatchPreciseException = 7,
}

impl Into<i32> for SegmentationFaultCode
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self as i32
	}
}

impl Code for SegmentationFaultCode
{
	type Data = FaultData;

	const InclusiveMaximum: Self = SegmentationFaultCode::AdiMismatchPreciseException;

	#[inline(always)]
	fn convert(code: i32) -> Self
	{
		unsafe { transmute(code) }
	}
}
