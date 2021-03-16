// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `right_shift_number_of_bits` must be between 0 and 64 inclusive.
#[inline(always)]
pub unsafe fn _mm_lane_crossing_right_shift_upto_64_bits<const right_shift_number_of_bits: u8>(data: __m128i) -> __m128i
{
	debug_assert!(right_shift_number_of_bits <= MaximumLaneCrossingShift);
	
	// In shifting right, we lose bits that need to be rotated out but in the lane below ourselves.
	let shift_right_all_64_bit_lanes = _mm_srli_epi64(data, right_shift_number_of_bits as i32);
	
	// Here we find the bits that would be lost.
	// We rotate them to the lane below ourselves.
	// We want lane 1 to go to lane 0.
	// We want to zeros to fill lane 1.
	let carry_out_of_shift_right = _mm_slli_epi64(data, (MaximumLaneCrossingShift - right_shift_number_of_bits) as i32);
	
	/*
		What should the value of the immediate value `control` be for `_mm_shuffle_epi64(a, b, control)`?
		
		 * 1 0		Destination Lane 1	Destination Lane 0
		0b_0_0		Source b Lane 0		Source a Lane 0
		0b_0_1		Source b Lane 0		Source a Lane 1
		0b_1_0		Source b Lane 1		Source a Lane 0
		0b_1_1		Source b Lane 1		Source a Lane 1
										* Destination lane index.
		If:-
			* Source a is `carry_out_of_shift_right`
			* Source b is zeros
		
		 * 1 0		Destination Lane 1	Destination Lane 0
		0b_0_0		Zeros				Source a Lane 0
		0b_0_1		Zeros				Source a Lane 1
		0b_1_0		Zeros				Source a Lane 0
		0b_1_1		Zeros				Source a Lane 1
										* Destination lane index.
		Thus `control` can be either `0b_0_1` or `0b_1_1`.
	 */
	const control: i32 = 0b_0_1;
	let right_rotated_64_bits_with_top_lane_zeroed = _mm_shuffle_epi64_constant::<control>(carry_out_of_shift_right, _mm_setzero_si128());
	
	// Combine right shift bits and the bits rotated into the lane above.
	_mm_or_si128(shift_right_all_64_bit_lanes, right_rotated_64_bits_with_top_lane_zeroed)
}
