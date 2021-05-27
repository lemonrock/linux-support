// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `left_shift_number_of_bits` must be between 0 and 64 inclusive.
#[inline(always)]
pub unsafe fn _mm256_lane_crossing_left_shift_upto_64_bits(data: __m256i, left_shift_number_of_bits: u8) -> __m256i
{
	use self::U64LaneIndex::*;
	
	debug_assert!(left_shift_number_of_bits <= MaximumLaneCrossingShift);
	
	macro_rules! compiler_const_generics_hack
	{
		($callback: tt, $immediate: expr) =>
		{
			{
				const immediate: i32 = ($immediate) as i32;
				$callback::<immediate>(data)
			}
		}
	}
	
	// In shifting left, we lose bits that need to be rotated in but in the lane above ourselves.
	let shift_left_all_64_bit_lanes = match left_shift_number_of_bits
	{
		0 => compiler_const_generics_hack!(_mm256_slli_epi64, 0),
		1 => compiler_const_generics_hack!(_mm256_slli_epi64, 1),
		2 => compiler_const_generics_hack!(_mm256_slli_epi64, 2),
		3 => compiler_const_generics_hack!(_mm256_slli_epi64, 3),
		4 => compiler_const_generics_hack!(_mm256_slli_epi64, 4),
		5 => compiler_const_generics_hack!(_mm256_slli_epi64, 5),
		6 => compiler_const_generics_hack!(_mm256_slli_epi64, 6),
		7 => compiler_const_generics_hack!(_mm256_slli_epi64, 7),
		8 => compiler_const_generics_hack!(_mm256_slli_epi64, 8),
		9 => compiler_const_generics_hack!(_mm256_slli_epi64, 9),
		10 => compiler_const_generics_hack!(_mm256_slli_epi64, 10),
		11 => compiler_const_generics_hack!(_mm256_slli_epi64, 11),
		12 => compiler_const_generics_hack!(_mm256_slli_epi64, 12),
		13 => compiler_const_generics_hack!(_mm256_slli_epi64, 13),
		14 => compiler_const_generics_hack!(_mm256_slli_epi64, 14),
		15 => compiler_const_generics_hack!(_mm256_slli_epi64, 15),
		16 => compiler_const_generics_hack!(_mm256_slli_epi64, 16),
		17 => compiler_const_generics_hack!(_mm256_slli_epi64, 17),
		18 => compiler_const_generics_hack!(_mm256_slli_epi64, 18),
		19 => compiler_const_generics_hack!(_mm256_slli_epi64, 19),
		20 => compiler_const_generics_hack!(_mm256_slli_epi64, 20),
		21 => compiler_const_generics_hack!(_mm256_slli_epi64, 21),
		22 => compiler_const_generics_hack!(_mm256_slli_epi64, 22),
		23 => compiler_const_generics_hack!(_mm256_slli_epi64, 23),
		24 => compiler_const_generics_hack!(_mm256_slli_epi64, 24),
		25 => compiler_const_generics_hack!(_mm256_slli_epi64, 25),
		26 => compiler_const_generics_hack!(_mm256_slli_epi64, 26),
		27 => compiler_const_generics_hack!(_mm256_slli_epi64, 27),
		28 => compiler_const_generics_hack!(_mm256_slli_epi64, 28),
		29 => compiler_const_generics_hack!(_mm256_slli_epi64, 29),
		30 => compiler_const_generics_hack!(_mm256_slli_epi64, 30),
		31 => compiler_const_generics_hack!(_mm256_slli_epi64, 31),
		32 => compiler_const_generics_hack!(_mm256_slli_epi64, 32),
		33 => compiler_const_generics_hack!(_mm256_slli_epi64, 33),
		34 => compiler_const_generics_hack!(_mm256_slli_epi64, 34),
		35 => compiler_const_generics_hack!(_mm256_slli_epi64, 35),
		36 => compiler_const_generics_hack!(_mm256_slli_epi64, 36),
		37 => compiler_const_generics_hack!(_mm256_slli_epi64, 37),
		38 => compiler_const_generics_hack!(_mm256_slli_epi64, 38),
		39 => compiler_const_generics_hack!(_mm256_slli_epi64, 39),
		40 => compiler_const_generics_hack!(_mm256_slli_epi64, 40),
		41 => compiler_const_generics_hack!(_mm256_slli_epi64, 41),
		42 => compiler_const_generics_hack!(_mm256_slli_epi64, 42),
		43 => compiler_const_generics_hack!(_mm256_slli_epi64, 43),
		44 => compiler_const_generics_hack!(_mm256_slli_epi64, 44),
		45 => compiler_const_generics_hack!(_mm256_slli_epi64, 45),
		46 => compiler_const_generics_hack!(_mm256_slli_epi64, 46),
		47 => compiler_const_generics_hack!(_mm256_slli_epi64, 47),
		48 => compiler_const_generics_hack!(_mm256_slli_epi64, 48),
		49 => compiler_const_generics_hack!(_mm256_slli_epi64, 49),
		50 => compiler_const_generics_hack!(_mm256_slli_epi64, 50),
		51 => compiler_const_generics_hack!(_mm256_slli_epi64, 51),
		52 => compiler_const_generics_hack!(_mm256_slli_epi64, 52),
		53 => compiler_const_generics_hack!(_mm256_slli_epi64, 53),
		54 => compiler_const_generics_hack!(_mm256_slli_epi64, 54),
		55 => compiler_const_generics_hack!(_mm256_slli_epi64, 55),
		56 => compiler_const_generics_hack!(_mm256_slli_epi64, 56),
		57 => compiler_const_generics_hack!(_mm256_slli_epi64, 57),
		58 => compiler_const_generics_hack!(_mm256_slli_epi64, 58),
		59 => compiler_const_generics_hack!(_mm256_slli_epi64, 59),
		60 => compiler_const_generics_hack!(_mm256_slli_epi64, 60),
		61 => compiler_const_generics_hack!(_mm256_slli_epi64, 61),
		62 => compiler_const_generics_hack!(_mm256_slli_epi64, 62),
		63 => compiler_const_generics_hack!(_mm256_slli_epi64, 63),
		64 => compiler_const_generics_hack!(_mm256_slli_epi64, 64),
		
		_ => unreachable_code_const("Only values 0 to 64 inclusive are permitted")
	};
	
	let carry_out_of_shift_left = match left_shift_number_of_bits
	{
		0 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 0),
		1 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 1),
		2 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 2),
		3 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 3),
		4 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 4),
		5 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 5),
		6 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 6),
		7 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 7),
		8 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 8),
		9 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 9),
		10 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 10),
		11 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 11),
		12 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 12),
		13 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 13),
		14 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 14),
		15 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 15),
		16 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 16),
		17 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 17),
		18 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 18),
		19 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 19),
		20 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 20),
		21 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 21),
		22 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 22),
		23 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 23),
		24 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 24),
		25 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 25),
		26 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 26),
		27 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 27),
		28 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 28),
		29 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 29),
		30 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 30),
		31 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 31),
		32 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 32),
		33 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 33),
		34 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 34),
		35 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 35),
		36 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 36),
		37 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 37),
		38 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 38),
		39 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 39),
		40 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 40),
		41 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 41),
		42 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 42),
		43 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 43),
		44 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 44),
		45 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 45),
		46 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 46),
		47 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 47),
		48 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 48),
		49 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 49),
		50 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 50),
		51 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 51),
		52 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 52),
		53 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 53),
		54 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 54),
		55 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 55),
		56 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 56),
		57 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 57),
		58 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 58),
		59 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 59),
		60 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 60),
		61 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 61),
		62 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 62),
		63 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 63),
		64 => compiler_const_generics_hack!(_mm256_srli_epi64, MaximumLaneCrossingShift - 64),
		
		_ => unreachable_code_const("Only values 0 to 64 inclusive are permitted")
	};
	
	// Here we find the bits that would be lost.
	// We rotate them to the lane above ourselves (eg Lane 2 goes to Lane 3).
	// We don't want Lane 3 at all.
	// We set Lane 0 to zero (we shift in zeroes).
	let left_rotated_64_bits_with_bottom_lane_zeroed = shuffle_and_blend_with_zeros!(carry_out_of_shift_left, Some(Lane2), Some(Lane1), Some(Lane0), None);
	
	// Combine left shifted bits and the bits rotated into the lane above.
	_mm256_or_si256(shift_left_all_64_bit_lanes, left_rotated_64_bits_with_bottom_lane_zeroed)
}
