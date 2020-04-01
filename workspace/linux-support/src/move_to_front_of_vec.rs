// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Forcing the constraint of `T: Copy` ensures that we do not need to drop the discarded values (Copy is incompatible with Drop).
#[inline(always)]
pub(crate) fn move_to_front_of_vec<T: Copy>(mut vec: Vec<T>, from_inclusive_index: usize) -> Vec<T>
{
	let pointer = vec.as_mut_ptr();
	let bytes_to_move = vec.len() - from_inclusive_index;
	unsafe
	{
		pointer.copy_from(pointer.offset(from_inclusive_index as isize), bytes_to_move);
		vec.set_len(bytes_to_move)
	}
	vec
}
