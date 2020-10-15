// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(transparent)]
pub(crate) struct AssociationModeAndVersionNumberAndLeapIndicator(u8);

impl AssociationModeAndVersionNumberAndLeapIndicator
{
	pub(crate) const SimpleNetworkTimeProtocolClientRequest: Self = Self::new
	(
		AssociationMode::Client,
		
		VersionNumber::Version4,
		
		LeapIndicator::AlarmConditionClockNotSynchronized,
	);
	
	const AssociationModeBitShift: u8 = 6;
	
	const VersionNumberMask: u8 = 0b0001_1100;
	
	const VersionNumberBitShift: u8 = 2;
	
	const LeapIndicatorMask: u8 = 0b0000_0011;
	
	#[inline(always)]
	const fn new(association_mode: AssociationMode, vn: VersionNumber, li: LeapIndicator) -> Self
	{
		Self(((association_mode as u8) << Self::AssociationModeBitShift) | ((vn as u8) << Self::VersionNumberBitShift) (li as u8))
	}
	
	#[inline(always)]
	pub(crate) fn parse_into_constituent_bitfields(first_byte: u8) -> Result<LeapIndicator, NetworkTimeProtocolMessageServerReplyFirstByteParseError>
	{
		use self::NetworkTimeProtocolMessageServerReplyFirstByteParseError::*;
		
		{
			let association_mode: AssociationMode = unsafe { transmute(first_byte >> Self::AssociationModeBitShift) };
			if unlikely!(association_mode != AssociationMode::Server)
			{
				return Err(InvalidAssociationMode(association_mode))
			}
		}
		
		{
			let version_number_raw = (first_byte & Self::VersionNumberMask) >> Self::VersionNumberBitShift;
			match version_number_raw
			{
				1 ..= 3 => return Err(ObsoleteVersionNumber(unsafe { transmute(version_number_raw) })),
				
				4 => (),
				
				0 | 4 ..= 7 => return Err(UnknownVersionNumber(version_number_raw)),
				
				_ => unreachable!(),
			}
		}
		
		Ok(unsafe { transmute(first_byte & Self::LeapIndicatorMask) })
	}
}
