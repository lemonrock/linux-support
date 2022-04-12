// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


macro_rules! required_capacity
{
	($current_length: ident, $current_capacity: ident, $additional: ident, $capacity_sufficient_pointer: expr) =>
	{
		{
			let required_capacity = Self::required_capacity($current_length, $additional)?;

			if required_capacity <= $current_capacity
			{
				return Ok($capacity_sufficient_pointer)
			}
			
			required_capacity
		}
	}
}
