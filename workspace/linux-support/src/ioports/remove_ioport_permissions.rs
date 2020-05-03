// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Remove I/O port permissions.
#[cfg(not(target_arch = "powerpc64"))]
#[inline(always)]
pub fn remove_ioport_permissions()
{
	let _ = set_ioport_permissions(0 ..= 0x3FF, false);
	let _ = set_ioport_permissions(0 ..= u16::MAX, false);
}

/// Remove I/O port permissions (does nothing for PowerPC).
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub fn remove_ioport_permissions()
{
}
