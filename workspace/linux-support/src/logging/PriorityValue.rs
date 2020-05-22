// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 5424: "The Priority value is calculated by first multiplying the Facility number by 8 and then adding the numerical value of the Severity".
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PriorityValue(u8);

impl NumberAsBytes for PriorityValue
{
	/// Number as bytes.
	///
	/// Returns a `bytes_index` which is the position of the last, lefthand byte written by this method; it could be zero.
	fn number_as_bytes(self, bytes_index: usize, bytes: &mut [u8], radix: Radix, non_numeric_digit_case: NonNumericDigitCase) -> usize
	{
		self.0.number_as_bytes(bytes_index, bytes, radix, non_numeric_digit_case)
	}
}

impl PriorityValue
{
	const Scalar: u8 = 8;
	
	#[inline(always)]
	fn as_u8(self) -> u8
	{
		self.0
	}
	
	/// Known facility.
	#[inline(always)]
	pub const fn known_facility(self) -> KnownFacility
	{
		let facility = self.facility();
		cfn_debug_assert!(facility.raw_value_is_known());
		unsafe { facility.known }
	}
	
	/// Facility.
	#[inline(always)]
	pub const fn facility(self) -> Facility
	{
		unsafe { transmute(self.0 / Self::Scalar) }
	}
	
	/// Severity.
	#[inline(always)]
	pub const fn severity(self) -> Severity
	{
		unsafe { transmute(self.0 % Self::Scalar) }
	}
	
	/// New instance.
	#[inline(always)]
	pub const fn from_facility_and_severity(known_facility: KnownFacility, severity: Severity) -> Self
	{
		Self(((known_facility as u8) * 8) + (severity as u8))
	}
}
