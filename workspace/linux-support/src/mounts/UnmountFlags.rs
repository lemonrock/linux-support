// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Unmount flags.
	#[allow(missing_docs)]
	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields)]
	pub struct UnmountFlags: i32
	{
		/// Force.
		const Force = MNT_FORCE;

		/// Detach.
		const Detach = MNT_DETACH;

		/// Expire.
		const Expire = MNT_EXPIRE;

		// Not in libc crate
		// const NoFollow = UMOUNT_NOFOLLOW,
	}
}
