// This file is part of likely. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/likely/master/COPYRIGHT. No part of likely, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of likely. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/likely/master/COPYRIGHT.


/// likely.
#[macro_export]
macro_rules! likely
{
	($expr: expr) =>
	{
		unsafe { ::std::intrinsics::likely($expr) }
	}
}
