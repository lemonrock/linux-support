// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
#[repr(C)]
pub(crate) struct UnsignedTimestampFormat
{
	seconds: BigEndianU32,

	fraction: BigEndianU32,
}

impl Into<Unsigned3232FixedPoint> for UnsignedTimestampFormat
{
	#[inline(always)]
	fn into(self) -> Unsigned3232FixedPoint
	{
		Unsigned3232FixedPoint::from((self.seconds, self.fraction))
	}
}

impl From<Unsigned3232FixedPoint> for UnsignedTimestampFormat
{
	#[inline(always)]
	fn from(value: Unsigned3232FixedPoint) -> Self
	{
		Self
		{
			seconds: value.integer().to_be_bytes(),
			fraction: value.fraction().to_be_bytes(),
		}
	}
}
impl From<SystemTime> for UnsignedTimestampFormat
{
	#[inline(always)]
	fn from(value: SystemTime) -> Self
	{
		Self::from(Self::from_system_time(value))
	}
}

impl UnsignedTimestampFormat
{
	pub(crate) const Zero: Self = Self
	{
		seconds: [0; 4],
		
		fraction: [0; 4],
	};
	
	#[inline(always)]
	pub(crate) fn now() -> Self
	{
		Self::from(SystemTime::now())
	}
	
	#[inline(always)]
	pub(crate) fn subtract(self, rhs: Self) -> Result<Signed3232FixedPoint, NetworkTimeProtocolMessageServerReplyParseError>
	{
		self.checked_sub(rhs).ok_or(NetworkTimeProtocolMessageServerReplyParseError::TooLargeATimestampDifference { left: self, right: rhs })
	}
	
	#[inline(always)]
	pub(crate) fn checked_sub(self, rhs: Self) -> Option<Signed3232FixedPoint>
	{
		let left = self.into();
		let right = self.into();
		match left.cmp(&right)
		{
			Equal => Some(Signed3232FixedPoint::Zero),
			
			Less =>
			{
				let difference = right - left;
				difference.try_into().ok().map(|absolute| absolute.neg())
			}
			
			Greater =>
			{
				let difference = left - right;
				difference.try_into().ok()
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn from_system_time(system_time: SystemTime) -> Unsigned3232FixedPoint
	{
		let (seconds, fraction) = Self::seconds_and_fraction_from_system_time(system_time);
		Unsigned3232FixedPoint::new(seconds as u32, fraction as u32)
	}
	
	#[inline(always)]
	pub(crate) fn into_system_time(value: Unsigned3232FixedPoint) -> SystemTime
	{
		let (seconds, fraction): (u32, u32) = value.into();
		Self::seconds_and_fraction_into_system_time(seconds as u64, fraction as u64)
	}
	
	// NTP's epoch is January 1, 1900 (UTC).
	// Unix's epoch is January 1, 1970 (UTC).
	const UnixEpochOffsetToNtpEpochInSeconds: u64 =
	{
		const NumberOfDays: u64 =
		{
			const NumberOfYears: u64 = 70;
			
			const DaysPerYear: u64 = 365;
			
			const NumberOfLeapYears: u64 = 17;
			
			const ExtraDayForEachLeapYear: u64 = 1;
			
			(NumberOfYears * DaysPerYear) + (NumberOfLeapYears * ExtraDayForEachLeapYear)
		};
		
		const SecondsPerDay: u64 =
		{
			const SecondsPerMinute: u64 = 60;
			const MinutesPerHour: u64 = 60;
			const HoursPerDay: u64 = 24;
			SecondsPerMinute * MinutesPerHour * HoursPerDay
		};
		
		NumberOfDays * SecondsPerDay
	};
	
	const FractionSizeInBits: u64 = 32;
	
	const FractionsPerSecond: u64 = 1 << Self::FractionSizeInBits;
	
	const NanosecondsPerSecond: u64 = 1_0000_0000_000;
	
	#[inline(always)]
	const fn seconds_and_fraction_from_system_time(system_time: SystemTime) -> (u64, u64)
	{
		let duration = system_time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
		let seconds = Self::UnixEpochOffsetToNtpEpochInSeconds + duration.as_secs();
		let fraction =
		{
			let nanoseconds = duration.subsec_nanos() as u64;
			(nanoseconds << Self::FractionSizeInBits) / Self::NanosecondsPerSecond
		};
		(seconds, fraction)
	}
	
	#[inline(always)]
	const fn seconds_and_fraction_into_system_time(seconds: u64, fraction: u64) -> SystemTime
	{
		let seconds = seconds - Self::UnixEpochOffsetToNtpEpochInSeconds;
		let nanoseconds = (fraction * Self::NanosecondsPerSecond) >> Self::FractionSizeInBits;
		let duration = Duration::new(seconds, nanoseconds as u32);
		SystemTime::UNIX_EPOCH + duration
	}
}
