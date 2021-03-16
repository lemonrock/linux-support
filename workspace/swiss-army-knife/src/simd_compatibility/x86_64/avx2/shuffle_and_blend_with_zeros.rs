// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! shuffle_and_blend_with_zeros
{
	($value: ident, $source_of_lane3: expr, $source_of_lane2: expr, $source_of_lane1: expr, $source_of_lane0: expr) =>
	{
		{
			const ShuffleControl: i32 = U64LaneIndex::shuffle_control($source_of_lane3, $source_of_lane2, $source_of_lane1, $source_of_lane0);
			const BlendControl: i32 = U64LaneIndex::blend_control($source_of_lane3, $source_of_lane2, $source_of_lane1, $source_of_lane0);
			U64LaneIndex::shuffle_and_blend_with_zeros::<ShuffleControl>($value, BlendControl)
		}
	}
}
