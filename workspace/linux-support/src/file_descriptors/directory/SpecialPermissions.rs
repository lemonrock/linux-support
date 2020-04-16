// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// File special permissions.
	#[repr(transparent)]
	pub struct SpecialPermissions: u8
	{
		/// Also known as `svtx`.
		const StickyBit = R_OK as u8;

		/// `sgid`
		const SetGroupIdentifier = W_OK as u8;

		/// `suid`
		const SetUserIdentifier = X_OK as u8;
	}
}
