// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Each secure setting is implemented using two bits.
/// One bit specifies  whether the setting is on or off.
/// The other bit specify whether the setting is locked or not.
/// A setting which is locked cannot be changed from user-level.
const fn issecure_mask(secure: u32) -> u32
{
	1 << secure
}
