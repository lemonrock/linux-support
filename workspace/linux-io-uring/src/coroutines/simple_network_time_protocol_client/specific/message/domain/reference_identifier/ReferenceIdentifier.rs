// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Left-justified, zero-padded 0 to 4 byte (inclusive) ASCII explanatory code.
///
/// Defined in RFC 4330, Section 8, Figure 3.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct ReferenceIdentifier([i8; 4]);

impl ReferenceIdentifier
{
	/// Uncalibrated local clock.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const LOCL: Self = Self(b"LOCL");
	
	/// Calibrated Cesium clock.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const CESM: Self = Self(b"CESM");
	
	/// Calibrated Rubidium clock.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const RBDM: Self = Self(b"RBDM");
	
	/// Calibrated quartz clock or other pulse-per-second source.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const PPS : Self = Self(b"PPS\0");
	
	/// Inter-Range Instrumentation Group.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const IRIG: Self = Self(b"IRIG");
	
	/// NIST telephone modem service.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const ACTS: Self = Self(b"ACTS");
	
	/// USNO telephone modem service.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const USNO: Self = Self(b"USNO");
	
	/// PTB (Germany) telephone modem service.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const PTB : Self = Self(b"PTB\0");
	
	/// Allouis (France) Radio 164 kHz.
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const TDF : Self = Self(b"TDF\0");
	
	/// Mainflingen (Germany) Radio 77.5 kHz.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const DCF : Self = Self(b"DCF\0");
	
	/// Rugby (UK) Radio 60 kHz.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const MSF : Self = Self(b"MSF\0");
	
	/// Ft. Collins (US) Radio 2.5, 5, 10, 15, 20 MHz.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const WWV : Self = Self(b"WWV\0");
	
	/// Boulder (US) Radio 60 kHz.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const WWVB: Self = Self(b"WWVB");
	
	/// Kauai Hawaii (US) Radio 2.5, 5, 10, 15 MHz.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const WWVH: Self = Self(b"WWVH");
	
	/// Ottawa (Canada) Radio 3330, 7335, 14670 kHz.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const CHU : Self = Self(b"CHU\0");
	
	/// LORAN-C radionavigation system.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const LORC: Self = Self(b"LORC");
	
	/// OMEGA radionavigation system.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const OMEG: Self = Self(b"OMEG");
	
	/// Global Positioning Service.
	///
	/// Defined in RFC 4330, Section 4, Figure 2.
	pub const GPS : Self = Self(b"GPS\0");
}
