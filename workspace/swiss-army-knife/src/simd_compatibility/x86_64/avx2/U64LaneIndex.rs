// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
enum U64LaneIndex
{
	Lane0 = 0,
	
	Lane1 = 1,
	
	Lane2 = 2,
	
	Lane3 = 3,
}

impl Default for U64LaneIndex
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Lane0
	}
}

impl U64LaneIndex
{
	/// Use this function via the macro `shuffle_and_blend_with_zeros!`.
	///
	/// `source_of_lane*` is the source lane from `value` to be placed into position in the newly `shuffled` vector after shuffling.
	///
	/// If an argument is `None` it will be replaced with zeros.
	#[inline(always)]
	fn shuffle_and_blend_with_zeros<const ShuffleControl: i32>(value: __m256i, blend_control: i32) -> __m256i
	{
		let shuffled = unsafe { _mm256_permute4x64_epi64(value, ShuffleControl) };
		let blended = unsafe { _mm256_blend_epi64(_mm256_setzero_si256(), shuffled, blend_control) };
		blended
	}
	
	#[doc(hidden)]
	const fn shuffle_control(source_of_lane3: Option<Self>, source_of_lane2: Option<Self>, source_of_lane1: Option<Self>, source_of_lane0: Option<Self>) -> i32
	{
		use self::U64LaneIndex::*;
		
		(Self::to_shuffle_control_value(Lane3, source_of_lane3) | Self::to_shuffle_control_value(Lane2, source_of_lane2) | Self::to_shuffle_control_value(Lane1, source_of_lane1) | Self::to_shuffle_control_value(Lane0, source_of_lane0)) as i32
	}
	
	#[doc(hidden)]
	const fn blend_control(source_of_lane3: Option<Self>, source_of_lane2: Option<Self>, source_of_lane1: Option<Self>, source_of_lane0: Option<Self>) -> i32
	{
		use self::U64LaneIndex::*;
		
		let blend_control = (Self::to_blend_control_value(Lane3, source_of_lane3) | Self::to_blend_control_value(Lane2, source_of_lane2) | Self::to_blend_control_value(Lane1, source_of_lane1) | Self::to_blend_control_value(Lane0, source_of_lane0)) as i32;
		
		blend_control
	}
	
	const fn to_shuffle_control_value(destination_of_lane: Self, source_of_lane: Option<Self>) -> u8
	{
		match source_of_lane
		{
			None => return 0,
			
			Some(shuffle_source) => (shuffle_source as u8) << ((destination_of_lane as u8) * 2)
		}
	}
	
	const fn to_blend_control_value(destination_of_lane: Self, source_of_lane: Option<Self>) -> u8
	{
		let value = match source_of_lane
		{
			None => 0b0,
			
			Some(_) => 0b1
		};
		value << (destination_of_lane as u8)
	}
}
