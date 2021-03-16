// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `left_shift_number_of_bits` must be between 0 and 64 inclusive.
#[inline(always)]
pub unsafe fn _mm_lane_crossing_left_shift_upto_64_bits<const left_shift_number_of_bits: u8>(data: __m128i) -> __m128i
{
	debug_assert!(left_shift_number_of_bits <= MaximumLaneCrossingShift);
	
	// In shifting left, we lose bits that need to be rotated in but in the lane above ourselves.
	let shift_left_all_64_bit_lanes = _mm_slli_epi64(data, left_shift_number_of_bits as i32);
	
	// Here we find the bits that would be lost.
	// We rotate them to the lane above ourselves.
	// We want lane 0 to go to lane 1.
	// We want zeros to fill lane 0.
	let carry_out_of_shift_left = _mm_srli_epi64(data, (MaximumLaneCrossingShift - left_shift_number_of_bits) as i32);
	
	/*
		What should the value of the immediate value `control` be for `_mm_shuffle_epi64(a, b, control)`?
		
		 * 1 0		Destination Lane 1	Destination Lane 0
		0b_0_0		Source b Lane 0		Source a Lane 0
		0b_0_1		Source b Lane 0		Source a Lane 1
		0b_1_0		Source b Lane 1		Source a Lane 0
		0b_1_1		Source b Lane 1		Source a Lane 1
										* Destination lane index.
		If:-
			* Source a is zeros
			* Source b is `carry_out_of_shift_left`
		
		 * 1 0		Destination Lane 1	Destination Lane 0
		0b_0_0		Source b Lane 0		Zeros
		0b_0_1		Source b Lane 0		Zeros
		0b_1_0		Source b Lane 1		Zeros
		0b_1_1		Source b Lane 1		Zeros
										* Destination lane index.
		Thus `control` can be either `0b_0_0` or `0b_0_1`.
	 */
	const control: i32 = 0b_0_0;
	let left_rotated_64_bits_with_bottom_lane_zeroed = _mm_shuffle_epi64_constant::<control>(_mm_setzero_si128(), carry_out_of_shift_left);
	
	// Combine left shifted bits and the bits rotated into the lane above.
	_mm_or_si128(shift_left_all_64_bit_lanes, left_rotated_64_bits_with_bottom_lane_zeroed)
}
