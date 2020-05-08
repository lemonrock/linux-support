// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Splice flags.
	pub struct SpliceFlags: u32
	{
		#[allow(missing_docs)]
		const Move = SPLICE_F_MOVE;

		#[allow(missing_docs)]
		const NonBlock = SPLICE_F_NONBLOCK;

		#[allow(missing_docs)]
		const More = SPLICE_F_MORE;

		/// Not used with splice().
		#[doc(hidden)]
		const Gift = SPLICE_F_GIFT;
	}
}
