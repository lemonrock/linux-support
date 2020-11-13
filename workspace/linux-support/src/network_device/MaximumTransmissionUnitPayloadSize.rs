// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Layer 2 Ethernet Frame Maximum Transmission Unit (MTU) excluding:-
///
/// * Leading fields:-
/// 	* `MAC destination` (6 bytes).
/// 	* `MAC source` (6 bytes).
/// 	* `EtherType` (6 bytes).
/// * Trailing fields:-
/// 	* `Frame Check Sequence` (FCS).
///
/// In effect, it is the size of the `Payload`; it is also known as `Path MTU`.
///
/// This is the same as:-
/// * the `mtu` displayed by `ip link show eth0`.
/// * returned by Netlink's attribute `IFLA_MTU`.
/// * The definition in <https://www.cloudflare.com/learning/network-layer/what-is-mtu/>.
/// * Amazon AWS (<https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/network_mtu.html>) calls `1300 MTU`, `1500 MTU` or `9001 MTU`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct MaximumTransmissionUnitPayloadSize(NonZeroU16);

impl Default for MaximumTransmissionUnitPayloadSize
{
	#[inline(always)]
	fn default() -> Self
	{
		MaximumTransmissionUnitPayloadSize::NonJumboFrameInclusiveMaximum
	}
}

impl TryFrom<u8> for MaximumTransmissionUnitPayloadSize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value == 0
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Self::try_from(new_non_zero_u16(value as u16))
		}
	}
}

impl TryFrom<u16> for MaximumTransmissionUnitPayloadSize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		if value == 0
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Self::try_from(new_non_zero_u16(value))
		}
	}
}

impl TryFrom<u32> for MaximumTransmissionUnitPayloadSize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		Self::try_from(value as u64)
	}
}

impl TryFrom<usize> for MaximumTransmissionUnitPayloadSize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: usize) -> Result<Self, Self::Error>
	{
		Self::try_from(value as u64)
	}
}

impl TryFrom<u64> for MaximumTransmissionUnitPayloadSize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u64) -> Result<Self, Self::Error>
	{
		if value == 0
		{
			Err(ParseNumberError::WasZero)
		}
		else if value > (u16::MAX as u64)
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Self::try_from(new_non_zero_u16(value as u16))
		}
	}
}

impl TryFrom<NonZeroU16> for MaximumTransmissionUnitPayloadSize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU16) -> Result<Self, Self::Error>
	{
		if value < Self::InclusiveMinimum.0
		{
			Err(ParseNumberError::TooSmall)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl Into<usize> for MaximumTransmissionUnitPayloadSize
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0.get() as usize
	}
}

impl Into<i32> for MaximumTransmissionUnitPayloadSize
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self.0.get() as i32
	}
}

impl MaximumTransmissionUnitPayloadSize
{
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self::new_unchecked(46);
	
	/// Amazon AWS Wavelength Zone.
	pub const AmazonAwsWavelengthZoneInclusiveMaximum: Self = Self::new_unchecked(1300);
	
	/// Inclusive maximum for non-jumboframes.
	pub const NonJumboFrameInclusiveMaximum: Self = Self::new_unchecked(1500);
	
	#[inline(always)]
	const fn new_unchecked(value: u16) -> Self
	{
		Self(new_non_zero_u16(value))
	}
	
	/// Overhead of 18 bytes for a frame, including a trailing Frame Check Sequence (FCS).
	pub const EthernetFrameOverheadIncludingTrailingFrameCheckSequence: usize =
	{
		const header_fields_length: usize =
		{
			const MediaAccessControlAddressLength: usize = 6;
			const DestinationAddressLength: usize = MediaAccessControlAddressLength;
			const SourceAddressLength: usize = MediaAccessControlAddressLength;
			const EthertypeLength: usize = 2;
			
			DestinationAddressLength + SourceAddressLength + EthertypeLength
		};
		
		const trailer_fields_length: usize =
		{
			const FrameCheckSequenceLength: usize = 4;
			
			FrameCheckSequenceLength
		};
		
		header_fields_length + trailer_fields_length
	};
	
	/// Frame size, including trailing Frame Check Sequence (FCS).
	#[inline(always)]
	pub const fn frame_size_including_trailing_frame_check_sequence(self) -> usize
	{
		self.0.get() as usize + Self::EthernetFrameOverheadIncludingTrailingFrameCheckSequence
	}
}
