// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `left_shift_number_of_bits` must be between 0 and 64 inclusive.
#[inline(always)]
pub unsafe fn _mm256_lane_crossing_left_shift_upto_64_bits(data: __m256i, left_shift_number_of_bits: u8) -> __m256i
{
	use self::U64LaneIndex::*;
	
	const MaximumShift: u8 = U64LaneIndex::BitsPerLane;
	
	debug_assert!(left_shift_number_of_bits <= MaximumShift);
	
	// In shifting left, we lose bits that need to be rotated in but in the lane above ourselves.
	let shift_left_all_64_bit_lanes = _mm256_slli_epi64(data, left_shift_number_of_bits as i32);
	
	// Here we find the bits that would be lost.
	// We rotate them to the lane above ourselves (eg Lane 2 goes to Lane 3).
	// We don't want Lane 3 at all.
	// We set Lane 0 to zero (we shift in zeroes).
	let carry_out_of_shift_left = _mm256_srli_epi64(data, (MaximumShift - left_shift_number_of_bits) as i32);
	let left_rotated_64_bits_with_bottom_lane_zeroed = shuffle_and_blend_with_zeros!(carry_out_of_shift_left, Some(Lane2), Some(Lane1), Some(Lane0), None);
	
	// Combine left shifted bits and the bits rotated into the lane above.
	_mm256_or_si256(shift_left_all_64_bit_lanes, left_rotated_64_bits_with_bottom_lane_zeroed)
}
